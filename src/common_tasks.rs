
#[macro_export]
macro_rules! south_comms {
    (
    container => $tm_container:ty,
    tm_buffer_size => $tm_buf_size:literal,
    tc_buffer_size => $tc_buf_size:literal,
    device_id => $device_id:literal
    $(, on_telem => $on_telem:expr)?
    ) => {
        mod comms {
        use super::$tm_container;
        use defmt::{info, error, Debug2Format};
        use south_common::chell::{
            match_value, ChellDefinition, ChellValue, DynChellUnion, fd_compat_chell_union
        };
        use south_common::{
            definitions::{internal_msgs, telemetry},
            types::{Telecommand, Timesync},
        };
        use embassy_stm32::can::{
            BufferedFdCanReceiver, BufferedFdCanSender, frame::{FdEnvelope, FdFrame}
        };
        use embassy_time::{Instant, Duration, Ticker};
        use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
        use embassy_sync::signal::Signal;
        use embassy_sync::channel::{Channel, Receiver, Sender};
        use embassy_futures::select::{Either3, select3};
        use portable_atomic::{AtomicU8, AtomicU64, Ordering};

        // TM/TC channel types
        pub type TMChannel = Channel<ThreadModeRawMutex, $tm_container, $tm_buf_size>;
        pub type TMSender = Sender<'static, ThreadModeRawMutex, $tm_container, $tm_buf_size>;
        pub type TMReceiver = Receiver<'static, ThreadModeRawMutex, $tm_container, $tm_buf_size>;

        pub type TCChannel = Channel<ThreadModeRawMutex, Telecommand, $tc_buf_size>;
        pub type TCSender = Sender<'static, ThreadModeRawMutex, Telecommand, $tc_buf_size>;
        pub type TCReceiver = Receiver<'static, ThreadModeRawMutex, Telecommand, $tc_buf_size>;

        /// synced can device id
        static CAN_DEVICE_ID: AtomicU8 = AtomicU8::new($device_id);
        
        /// timesync status
        static REQ_TIME: AtomicU64 = AtomicU64::new(0);
        static TIME_REF: AtomicU64 = AtomicU64::new(0);
        static TIME_REF_PRIO: AtomicU8 = AtomicU8::new(u8::MAX);
        static TIMESYNC_REQUEST_SIGNAL: Signal<ThreadModeRawMutex, (u8, Instant)> = Signal::new();
        type TimesyncContainer = fd_compat_chell_union!(internal_msgs::TimesyncAnswer);
        const TIMESYNC_REQ_INTERVAL: Duration = Duration::from_secs(15);

        pub fn get_utc_us() -> u64 {
            TIME_REF.load(Ordering::Acquire) + Instant::now().as_micros()
        }
        pub fn set_utc_us(time_us: u64, prio: u8) {
            if TIME_REF_PRIO.load(Ordering::Acquire) < prio {
                return;
            }
            TIME_REF.store(time_us - Instant::now().as_micros(), Ordering::Release);
            TIME_REF_PRIO.store(prio, Ordering::Release);
        }

        fn gen_timesync_frame() -> Option<FdFrame> {
            let frame = FdFrame::new_standard(
                internal_msgs::TimesyncRequest.id(),
                core::slice::from_ref(&CAN_DEVICE_ID.load(Ordering::Acquire)),
            )
            .unwrap();
            REQ_TIME.store(Instant::now().as_micros(), Ordering::Release);
            Some(frame)
        }
        
        fn gen_tm_frame(container: $tm_container) -> Option<FdFrame> {
            Some(FdFrame::new_standard(container.id(), container.fd_bytes()).unwrap())
        }

        fn gen_time_ref_answer(request_id: u8, local_recv_instant: Instant) -> Option<FdFrame> {
            let diff = TIME_REF.load(Ordering::Acquire);
            if diff == 0 {
                return None;
            }
            let priority = TIME_REF_PRIO.load(Ordering::Acquire);
            let unix_time_recv = diff + local_recv_instant.as_micros();
            let unix_time_snd = diff + Instant::now().as_micros();
            let msg = Timesync {
                request_id,
                priority,
                unix_time_recv,
                unix_time_snd,
            };
            let container =
                TimesyncContainer::new(&internal_msgs::TimesyncAnswer, &msg).unwrap();
            Some(FdFrame::new_standard(container.id(), container.fd_bytes()).unwrap())
        }
        
        fn update_time_ref(envelope: &FdEnvelope) {
            match Timesync::read(envelope.frame.data()) {
                Ok((_len, timesync_answer)) => {
                    if timesync_answer.request_id != CAN_DEVICE_ID.load(Ordering::Acquire)
                        || timesync_answer.priority >= TIME_REF_PRIO.load(Ordering::Acquire)
                    {
                        return;
                    }
                    TIME_REF_PRIO.store(timesync_answer.priority.saturating_add(1), Ordering::Release);
                    let one_way_delay = (envelope.ts.as_micros() - REQ_TIME.load(Ordering::Acquire))
                                      - (timesync_answer.unix_time_snd - timesync_answer.unix_time_recv);
                    let time_ref = timesync_answer.unix_time_snd + one_way_delay - Instant::now().as_micros();
                    TIME_REF.store(time_ref, Ordering::Relaxed);
                }
                Err(e) => error!("could not read timesync msg {}", Debug2Format(&e)),
            }
        }

        async fn handle_can_msg(envelope: FdEnvelope, tc_channel: TCSender) {
            if let embedded_can::Id::Standard(id) = envelope.frame.id() {
                if let Ok(def) = internal_msgs::from_id(id.as_raw()) {
                    match_value!(def, {
                        internal_msgs::TimesyncRequest => {
                            match u8::read(envelope.frame.data()) {
                                Ok((_, request_id)) => TIMESYNC_REQUEST_SIGNAL.signal((request_id, envelope.ts)),
                                Err(_) => error!("error parsing ts req"),
                            }
                        },
                        internal_msgs::TimesyncAnswer => {
                            update_time_ref(&envelope);
                        },
                        internal_msgs::Telecommand => {
                            match Telecommand::read(envelope.frame.data()) {
                                Ok((_, cmd)) => tc_channel.send(cmd).await,
                                Err(_) => error!("error parsing tc"),
                            }
                        },
                    })
                }
                $(
                    else if let Ok(def) = telemetry::from_id(id.as_raw()) {
                        $on_telem(def).await;
                    }
                )?
                else {
                    error!("can id not in any chell def block")
                }
            } else {
                error!("non-standart can id")
            };
        }


        #[embassy_executor::task]
        pub async fn can_sender_thread(mut can_sender: BufferedFdCanSender, tm_channel: TMReceiver) {
            let mut timesync_req_ticker = Ticker::every(TIMESYNC_REQ_INTERVAL);
            loop {
                let opt_frame = match select3(
                    timesync_req_ticker.next(),
                    tm_channel.receive(),
                    TIMESYNC_REQUEST_SIGNAL.wait()
                ).await {
                    Either3::First(()) => gen_timesync_frame(),
                    Either3::Second(tm) => gen_tm_frame(tm),
                    Either3::Third((request_id, local_instant_recv)) => gen_time_ref_answer(request_id, local_instant_recv),
                };
                if let Some(frame) = opt_frame {
                    can_sender.write(frame).await;
                }
            }
        }

        #[embassy_executor::task]
        pub async fn can_receiver_thread(can: BufferedFdCanReceiver, tc_channel: TCSender) {
            loop {
                // receive from can
                match can.receive().await {
                    Ok(envelope) => handle_can_msg(envelope, tc_channel).await,
                    Err(e) => error!("error in can frame! {}", e),
                };
            }
        }
        }
    };
}
