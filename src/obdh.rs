use crate::{
    definitions::{internal_msgs, telemetry},
    types::{SubsystemCommand, Telecommand, Timesync},
};
use chell::{ChellDefinition, ChellUnion, ChellValue, fd_compat_chell_union, match_value};
use defmt::{Debug2Format, error};
use embassy_futures::select::{Either3, select3};
use embassy_stm32::can::{
    BufferedFdCanReceiver, BufferedFdCanSender,
    frame::{FdEnvelope, FdFrame},
};
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::{Channel, Receiver, Sender};
use embassy_sync::signal::Signal;
use embassy_time::{Duration, Instant, Ticker};
use portable_atomic::{AtomicU8, AtomicU64, Ordering};

// common constants
type TimesyncContainer = fd_compat_chell_union!(internal_msgs::TimesyncAnswer);
const TIMESYNC_REQ_INTERVAL: Duration = Duration::from_secs(15);
const TM_BUFFER_SIZE: usize = 8;
const TC_BUFFER_SIZE: usize = 2;

// TM/TC channel types
pub type TMChannel<const SIZE: usize> =
    Channel<ThreadModeRawMutex, ChellUnion<SIZE>, TM_BUFFER_SIZE>;
pub type TMSender<'a, const SIZE: usize> =
    Sender<'a, ThreadModeRawMutex, ChellUnion<SIZE>, TM_BUFFER_SIZE>;

pub type TCChannel<Command> = Channel<ThreadModeRawMutex, Command, TC_BUFFER_SIZE>;
pub type TCReceiver<'a, Command> = Receiver<'a, ThreadModeRawMutex, Command, TC_BUFFER_SIZE>;

pub type TimesyncSignal = Signal<ThreadModeRawMutex, (u8, Instant)>;

pub struct InternalComChannels<Command, const SIZE: usize>
where
    Command: SubsystemCommand,
{
    tm_channel: TMChannel<SIZE>,
    tc_channel: TCChannel<Command>,
    timesync_req_signal: TimesyncSignal,

    // synced can device id
    can_device_id: AtomicU8,

    // timesync status
    req_time: AtomicU64,
    time_ref: AtomicU64,
    time_ref_prio: AtomicU8,
}
impl<Command, const SIZE: usize> InternalComChannels<Command, SIZE>
where
    Command: SubsystemCommand,
{
    pub const fn new(can_device_id: u8) -> Self {
        Self {
            tm_channel: TMChannel::<SIZE>::new(),
            tc_channel: TCChannel::new(),
            timesync_req_signal: Signal::new(),
            can_device_id: AtomicU8::new(can_device_id),
            req_time: AtomicU64::new(0),
            time_ref: AtomicU64::new(0),
            time_ref_prio: AtomicU8::new(u8::MAX),
        }
    }

    pub fn get_utc_us(&self) -> u64 {
        self.time_ref.load(Ordering::Acquire) + Instant::now().as_micros()
    }
    pub fn set_utc_us(&self, time_us: u64, prio: u8) {
        if self.time_ref_prio.load(Ordering::Acquire) < prio {
            return;
        }
        self.time_ref
            .store(time_us - Instant::now().as_micros(), Ordering::Release);
        self.time_ref_prio.store(prio, Ordering::Release);
    }
    pub fn get_tm_sender<'a>(&'a self) -> TMSender<'a, SIZE> {
        self.tm_channel.sender()
    }
    pub fn get_tc_receiver<'a>(&'a self) -> TCReceiver<'a, Command> {
        self.tc_channel.receiver()
    }
}

pub struct SouthCanSender<'a, Command, const SIZE: usize>
where
    Command: SubsystemCommand,
{
    can_sender: BufferedFdCanSender,
    com_channels: &'a InternalComChannels<Command, SIZE>,
}

impl<'a, Command, const SIZE: usize> SouthCanSender<'a, Command, SIZE>
where
    Command: SubsystemCommand,
{
    pub fn new(
        can_sender: BufferedFdCanSender,
        com_channels: &'a InternalComChannels<Command, SIZE>,
    ) -> Self {
        Self {
            can_sender,
            com_channels,
        }
    }
    fn gen_timesync_frame(&self) -> Option<FdFrame> {
        let frame = FdFrame::new_standard(
            internal_msgs::TimesyncRequest.id(),
            core::slice::from_ref(&self.com_channels.can_device_id.load(Ordering::Acquire)),
        )
        .unwrap();
        self.com_channels
            .req_time
            .store(Instant::now().as_micros(), Ordering::Release);
        Some(frame)
    }

    fn gen_tm_frame(&self, container: ChellUnion<SIZE>) -> Option<FdFrame> {
        Some(FdFrame::new_standard(container.id(), container.fd_bytes()).unwrap())
    }

    fn gen_time_ref_answer(&self, request_id: u8, local_recv_instant: Instant) -> Option<FdFrame> {
        let diff = self.com_channels.time_ref.load(Ordering::Acquire);
        if diff == 0 {
            return None;
        }
        let priority = self.com_channels.time_ref_prio.load(Ordering::Acquire);
        let unix_time_recv = diff + local_recv_instant.as_micros();
        let unix_time_snd = diff + Instant::now().as_micros();
        let msg = Timesync {
            request_id,
            priority,
            unix_time_recv,
            unix_time_snd,
        };
        let container = TimesyncContainer::new(&internal_msgs::TimesyncAnswer, &msg).unwrap();
        Some(FdFrame::new_standard(container.id(), container.fd_bytes()).unwrap())
    }
    pub async fn run(&mut self) -> ! {
        let mut timesync_req_ticker = Ticker::every(TIMESYNC_REQ_INTERVAL);
        loop {
            let opt_frame = match select3(
                timesync_req_ticker.next(),
                self.com_channels.tm_channel.receive(),
                self.com_channels.timesync_req_signal.wait(),
            )
            .await
            {
                Either3::First(()) => self.gen_timesync_frame(),
                Either3::Second(union) => self.gen_tm_frame(union),
                Either3::Third((request_id, local_instant_recv)) => {
                    self.gen_time_ref_answer(request_id, local_instant_recv)
                }
            };
            if let Some(frame) = opt_frame {
                self.can_sender.write(frame).await;
                defmt::debug!("Can message sent");
            }
        }
    }
}

pub struct EmptyFunc;
pub trait OnTMFunc {
    fn call(
        &mut self,
        def: &dyn ChellDefinition,
        envelope: &FdEnvelope,
    ) -> impl core::future::Future<Output = ()>;
    fn should_call(&self) -> bool {
        true
    }
}
impl OnTMFunc for EmptyFunc {
    async fn call(&mut self, _def: &dyn ChellDefinition, _envelope: &FdEnvelope) {}
    fn should_call(&self) -> bool {
        false
    }
}

pub struct SouthCanReceiver<'a, Command, const SIZE: usize, OnTM>
where
    OnTM: OnTMFunc,
    Command: SubsystemCommand,
{
    can_receiver: BufferedFdCanReceiver,
    com_channels: &'a InternalComChannels<Command, SIZE>,
    on_tm_func: OnTM,
}

impl<'a, Command, const SIZE: usize, OnTM> SouthCanReceiver<'a, Command, SIZE, OnTM>
where
    OnTM: OnTMFunc,
    Command: SubsystemCommand,
{
    pub fn new(
        can_receiver: BufferedFdCanReceiver,
        com_channels: &'a InternalComChannels<Command, SIZE>,
        on_tm_func: OnTM,
    ) -> Self {
        Self {
            can_receiver,
            com_channels,
            on_tm_func,
        }
    }

    fn update_time_ref(&self, envelope: &FdEnvelope) {
        match Timesync::read(envelope.frame.data()) {
            Ok((_len, timesync_answer)) => {
                if timesync_answer.request_id
                    != self.com_channels.can_device_id.load(Ordering::Acquire)
                    || timesync_answer.priority
                        >= self.com_channels.time_ref_prio.load(Ordering::Acquire)
                {
                    return;
                }
                self.com_channels.time_ref_prio.store(
                    timesync_answer.priority.saturating_add(1),
                    Ordering::Release,
                );
                let one_way_delay = (envelope.ts.as_micros()
                    - self.com_channels.req_time.load(Ordering::Acquire))
                    - (timesync_answer.unix_time_snd - timesync_answer.unix_time_recv);
                let time_ref =
                    timesync_answer.unix_time_snd + one_way_delay - envelope.ts.as_micros();
                self.com_channels
                    .time_ref
                    .store(time_ref, Ordering::Release);
            }
            Err(e) => error!("could not read timesync msg {}", Debug2Format(&e)),
        }
    }

    async fn handle_can_msg(&mut self, envelope: FdEnvelope) {
        if let embedded_can::Id::Standard(id) = envelope.frame.id() {
            defmt::debug!("Can message received");
            if let Ok(def) = internal_msgs::from_id(id.as_raw()) {
                match_value!(def, {
                    internal_msgs::TimesyncRequest => {
                        match u8::read(envelope.frame.data()) {
                            Ok((_, request_id)) => self.com_channels.timesync_req_signal.signal((request_id, envelope.ts)),
                            Err(_) => error!("error parsing ts req"),
                        }
                    },
                    internal_msgs::TimesyncAnswer => {
                        self.update_time_ref(&envelope);
                    },
                    internal_msgs::Telecommand => {
                        match Telecommand::read(envelope.frame.data()) {
                            Ok((_, cmd)) => {
                                if let Some(subsys_cmd) = Command::from(cmd) {
                                    self.com_channels.tc_channel.send(subsys_cmd).await
                                }
                            },
                            Err(_) => error!("error parsing tc"),
                        }
                    },
                })
            } else if self.on_tm_func.should_call()
                && let Ok(def) = telemetry::from_id(id.as_raw())
            {
                self.on_tm_func.call(def, &envelope).await;
            } else {
                error!("can id not in any chell def block")
            }
        } else {
            error!("non-standart can id")
        };
    }

    pub async fn run(&mut self) -> ! {
        loop {
            match self.can_receiver.receive().await {
                Ok(envelope) => self.handle_can_msg(envelope).await,
                Err(e) => error!("error in can frame! {}", e),
            };
        }
    }
}
