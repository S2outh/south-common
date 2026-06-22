use core::net;

use embassy_time::{Duration, Ticker};
use sntpc::{get_time, NtpContext, Error, fraction_to_microseconds};
use sntpc_net_embassy::UdpSocketWrapper;
use sntpc_time_embassy::EmbassyTimestampGenerator;
use embassy_net::udp::{BindError, UdpSocket};

const TIME_REQUEST_INTERVAL: Duration = Duration::from_secs(30);

pub struct NTPTimeSource<F: Fn(u64)> {
    set_time_fn: F,
    server_addr: net::SocketAddr,
    udp_socket: UdpSocketWrapper<'static>,
    ntp_context: NtpContext<EmbassyTimestampGenerator>
}

impl<F: Fn(u64)> NTPTimeSource<F> {
    pub fn new(mut socket: UdpSocket<'static>, server_addr: net::SocketAddr, set_time_fn: F)
        -> Result<Self, BindError> {

        socket.bind(0)?;
        let udp_socket = socket.into();
        let ntp_context = NtpContext::new(EmbassyTimestampGenerator::default());
        Ok(Self {
            set_time_fn,
            server_addr,
            udp_socket,
            ntp_context,
        })
    }
    async fn get_utc_us(&mut self) -> Result<u64, Error> {
        get_time(self.server_addr, &self.udp_socket, self.ntp_context).await
            .map(|time|
                time.sec() as u64 * 1_000_000 +
                fraction_to_microseconds(time.sec_fraction()) as u64
            )
    }
    pub async fn run(&mut self) -> ! {
        let mut ticker = Ticker::every(TIME_REQUEST_INTERVAL);
        loop {
            match self.get_utc_us().await {
                Ok(time) => (self.set_time_fn)(time),
                Err(e) => defmt::warn!("could not sync time: {}", e),
            }
            ticker.next().await;
        }
    }
}
