#![warn(clippy::all)]
#![warn(clippy::cargo)]

mod client;
#[cfg(feature = "bevy")]
mod client_bevy;
mod server;
#[cfg(feature = "bevy")]
mod server_bevy;
mod transport;
mod util;

pub use generational_arena::{Arena, Index};

pub use client::ClientTransport;
#[cfg(feature = "bevy")]
pub use client_bevy::{
    ClientRecvEvent, ClientSendEvent, ClientTransportError, ClientTransportPlugin,
};
pub use server::{ServerTransport, ServerTransportEvent, ServerClientsError};
#[cfg(feature = "bevy")]
pub use server_bevy::{
    ClientSet, ServerRecvEvent, ServerSendEvent, ServerTransportError, ServerTransportPlugin,
};
pub use transport::{Message, TransportSettings};
pub use util::ClientId;
