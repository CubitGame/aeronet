use std::{fmt::Debug, time::Duration};

use aeronet::stats::Rtt;
use steamworks::networking_sockets::{NetConnection, NetworkingSockets};

/// Statistics on a Steamworks client/server connection.
#[derive(Debug, Clone, Default)]
pub struct ConnectionStats {
    /// See [`Rtt`].
    pub rtt: Duration,
    pub connection_quality_local: f32,
    pub connection_quality_remote: f32,
    pub out_packets_per_sec: f32,
    pub out_bytes_per_sec: f32,
    pub in_packets_per_sec: f32,
    pub in_bytes_per_sec: f32,
    pub send_rate_bytes_per_sec: u32,
    pub pending: u32,
    pub queued_send_bytes: u64,
}

impl ConnectionStats {
    #[must_use]
    pub fn from_connection<M: 'static>(
        socks: &NetworkingSockets<M>,
        conn: &NetConnection<M>,
    ) -> Self {
        let Ok((info, _)) = socks.get_realtime_connection_status(conn, 0) else {
            return Self::default();
        };

        Self {
            rtt: u64::try_from(info.ping())
                .map(Duration::from_millis)
                .unwrap_or_default(),
            connection_quality_local: info.connection_quality_local(),
            connection_quality_remote: info.connection_quality_remote(),
            out_packets_per_sec: info.out_packets_per_sec(),
            out_bytes_per_sec: info.out_bytes_per_sec(),
            in_packets_per_sec: info.in_packets_per_sec(),
            in_bytes_per_sec: info.in_bytes_per_sec(),
            send_rate_bytes_per_sec: u32::try_from(info.send_rate_bytes_per_sec())
                .unwrap_or_default(),
            pending: u32::try_from(info.pending_unreliable()).unwrap_or_default(),
            queued_send_bytes: u64::try_from(info.queued_send_bytes()).unwrap_or_default(),
        }
    }
}

impl Rtt for ConnectionStats {
    fn rtt(&self) -> Duration {
        self.rtt
    }
}
