mod backend;
mod frontend;

pub use frontend::*;

use std::fmt::Debug;

use aeronet::{lane::LaneKind, message::BytesMapper, protocol::ProtocolVersion};
use aeronet_proto::{lane::LaneConfig, packet};
use derivative::Derivative;

use crate::shared::{self, WebTransportProtocol};

#[cfg(target_family = "wasm")]
pub type NativeConfig = xwt::current::WebTransportOptions;
#[cfg(not(target_family = "wasm"))]
pub type NativeConfig = wtransport::ClientConfig;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientConfig {
    pub version: ProtocolVersion,
    pub lanes_in: Vec<LaneKind>,
    pub lanes_out: Vec<LaneConfig>,
    pub bandwidth: usize,
    pub max_packet_len: usize,
    pub default_packet_cap: usize,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            version: ProtocolVersion::default(),
            lanes_in: Vec::new(),
            lanes_out: Vec::new(),
            bandwidth: shared::DEFAULT_BANDWIDTH,
            max_packet_len: shared::DEFAULT_MTU,
            default_packet_cap: shared::DEFAULT_MTU,
        }
    }
}

impl ClientConfig {
    pub fn new(
        version: ProtocolVersion,
        lanes_in: impl IntoIterator<Item = impl Into<LaneKind>>,
        lanes_out: impl IntoIterator<Item = impl Into<LaneConfig>>,
    ) -> Self {
        Self {
            version,
            lanes_in: lanes_in.into_iter().map(Into::into).collect(),
            lanes_out: lanes_out.into_iter().map(Into::into).collect(),
            ..Default::default()
        }
    }
}

cfg_if::cfg_if! {
    if #[cfg(target_family = "wasm")] {
        use crate::shared::JsError;

        type EndpointConnectError = JsError;
        type ConnectingError = JsError;
    } else {
        use crate::ty;

        type EndpointConnectError = <ty::Endpoint as xwt_core::EndpointConnect>::Error;
        type ConnectingError = <ty::Connecting as xwt_core::Connecting>::Error;
    }
}

#[derive(Debug, thiserror::Error)]
pub enum BackendError {
    #[error("failed to connect")]
    Connect(#[source] EndpointConnectError),
    #[error("failed to await connection")]
    AwaitConnection(#[source] ConnectingError),
    #[error(transparent)]
    Generic(#[from] shared::BackendError),
}

#[derive(Derivative, thiserror::Error)]
#[derivative(Debug(
    bound = "packet::SendError<<P::Mapper as BytesMapper<P::C2S>>::IntoError>: Debug, packet::RecvError<<P::Mapper as BytesMapper<P::S2C>>::FromError>: Debug"
))]
pub enum ClientError<P: WebTransportProtocol> {
    #[error("already connected")]
    AlreadyConnected,
    #[error("already disconnected")]
    AlreadyDisconnected,
    #[error("not connected")]
    NotConnected,
    #[error("backend closed")]
    BackendClosed,

    #[error(transparent)]
    Backend(#[from] BackendError),
    #[error(transparent)]
    Send(#[from] packet::SendError<<P::Mapper as BytesMapper<P::C2S>>::IntoError>),
    #[error(transparent)]
    Recv(#[from] packet::RecvError<<P::Mapper as BytesMapper<P::S2C>>::FromError>),
}
