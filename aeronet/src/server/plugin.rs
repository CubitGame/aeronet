use std::{fmt::Debug, marker::PhantomData};

use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use derivative::Derivative;

use crate::protocol::TransportProtocol;

use super::{ServerEvent, ServerTransport};

/// Forwards messages and events between the [`App`] and a [`ServerTransport`].
///
/// See [`server_transport_plugin`] for a function version of this plugin.
///
/// With this plugin added, the transport `T` will automatically run:
/// * [`poll`] in [`PreUpdate`] in [`ServerTransportSet::Recv`]
/// * [`flush`] in [`PostUpdate`] in [`ServerTransportSet::Send`]

/// [`poll`]: ServerTransport::poll
/// [`flush`]: ServerTransport::flush
///
/// This plugin sends out the events:
/// * [`ServerOpened`]
/// * [`ServerClosed`]
/// * [`RemoteClientConnecting`]
/// * [`RemoteClientConnected`]
/// * [`RemoteClientDisconnected`]
/// * [`FromClient`]
/// * [`AckFromClient`]
/// * [`ServerFlushError`]
///
/// This plugin provides the run conditions:
/// * [`server_open`]
/// * [`server_closed`]
///
/// These events can be read by your app to respond to incoming events. To send
/// out messages, or to disconnect a specific client, etc., you will need to
/// inject the transport as a resource into your system.
#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""), Default(bound = ""))]
pub struct ServerTransportPlugin<P, T> {
    #[derivative(Debug = "ignore")]
    _phantom: PhantomData<(P, T)>,
}

impl<P, T> Plugin for ServerTransportPlugin<P, T>
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    fn build(&self, app: &mut App) {
        server_transport_plugin::<P, T>(app);
    }
}

/// Forwards messages and events between the [`App`] and a [`ServerTransport`].
///
/// See [`ServerTransportPlugin`].
pub fn server_transport_plugin<P, T>(app: &mut App)
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    app.add_event::<ServerOpened<P, T>>()
        .add_event::<ServerClosed<P, T>>()
        .add_event::<RemoteClientConnecting<P, T>>()
        .add_event::<RemoteClientConnected<P, T>>()
        .add_event::<RemoteClientDisconnected<P, T>>()
        .add_event::<FromClient<P, T>>()
        .add_event::<AckFromClient<P, T>>()
        .add_event::<ServerFlushError<P, T>>()
        .configure_sets(PreUpdate, ServerTransportSet::Recv)
        .configure_sets(PostUpdate, ServerTransportSet::Send)
        .add_systems(PreUpdate, recv::<P, T>.in_set(ServerTransportSet::Recv))
        .add_systems(PostUpdate, send::<P, T>.in_set(ServerTransportSet::Send));
}

/// Runs the [`ServerTransportPlugin`] systems.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ServerTransportSet {
    /// Handles receiving data from the transport and updating its internal
    /// state.
    Recv,
    /// Handles flushing buffered messages and sending out buffered data.
    Send,
}

/// A [`Condition`]-satisfying system that returns `true` if the client `T`
/// exists *and* is in the [`Connected`] state.
///
/// # Example
///
/// ```
/// # use bevy_app::prelude::*;
/// # use bevy_ecs::prelude::*;
/// # use aeronet::{protocol::TransportProtocol, server::{ServerTransport, server_open}};
/// # fn run<P: TransportProtocol, T: ServerTransport<P> + Resource>() {
/// let mut app = App::new();
/// app.add_systems(Update, my_system::<P, T>.run_if(server_open::<P, T>));
///
/// fn my_system<P, T>(server: Res<T>)
/// where
///     P: TransportProtocol,
///     T: ServerTransport<P> + Resource,
/// {
///     // ..
/// }
/// # }
/// ```
///
/// [`Condition`]: bevy_ecs::schedule::Condition
/// [`Open`]: crate::server::ServerState::Open
pub fn server_open<P, T>(server: Option<Res<T>>) -> bool
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    if let Some(server) = server {
        server.state().is_open()
    } else {
        false
    }
}

/// A [`Condition`]-satisfying system that returns `true` if the client `T`
/// exists *and* is in the [`Connected`] state.
///
/// # Example
///
/// ```
/// # use bevy_app::prelude::*;
/// # use bevy_ecs::prelude::*;
/// # use aeronet::{protocol::TransportProtocol, server::{ServerTransport, server_closed}};
/// # fn run<P: TransportProtocol, T: ServerTransport<P> + Resource>() {
/// let mut app = App::new();
/// app.add_systems(Update, my_system::<P, T>.run_if(server_closed::<P, T>));
///
/// fn my_system<P, T>(server: Res<T>)
/// where
///     P: TransportProtocol,
///     T: ServerTransport<P> + Resource,
/// {
///     // ..
/// }
/// # }
/// ```
///
/// [`Condition`]: bevy_ecs::schedule::Condition
/// [`Closed`]: crate::server::ServerState::Closed
pub fn server_closed<P, T>(server: Option<Res<T>>) -> bool
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    if let Some(server) = server {
        server.state().is_closed()
    } else {
        true
    }
}

/// The server has completed setup and is ready to accept client
/// connections, changing state to [`ServerState::Open`].
///
/// See [`ServerEvent::Opened`]
///
/// [`ServerState::Open`]: crate::server::ServerState::Open
#[derive(Derivative, Event)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct ServerOpened<P, T>
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    #[derivative(Debug = "ignore")]
    #[doc(hidden)]
    pub _phantom: PhantomData<(P, T)>,
}

/// The server can no longer handle client connections, changing state to
/// [`ServerState::Closed`].
///
/// See [`ServerEvent::Closed`].
///
/// [`ServerState::Closed`]: crate::server::ServerState::Closed
#[derive(Derivative, Event)]
#[derivative(Debug(bound = "T::Error: Debug"), Clone(bound = "T::Error: Clone"))]
pub struct ServerClosed<P, T>
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    /// Why the server closed.
    pub reason: T::Error,
}

/// A remote client has requested to connect to this server.
///
/// The client has been given a key, and the server is trying to establish
/// communication with this client, but messages cannot be transported yet.
///
/// This event can be followed by [`ServerEvent::Connected`] or
/// [`ServerEvent::Disconnected`].
///
/// See [`ServerEvent::Connecting`].
#[derive(Derivative, Event)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct RemoteClientConnecting<P, T>
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    /// Key of the client.
    pub client_key: T::ClientKey,
}

/// A remote client has fully established a connection to this server.
///
/// This event can be followed by [`ServerEvent::Recv`] or
/// [`ServerEvent::Disconnected`].
///
/// After this event, you can run your player initialization logic such as
/// spawning the player's model in the world.
///
/// See [`ServerEvent::Connected`].
#[derive(Derivative, Event)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct RemoteClientConnected<P, T>
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    /// Key of the client.
    pub client_key: T::ClientKey,
}

/// A remote client has unrecoverably lost connection from this server.
///
/// This event is not raised when the server forces a client to disconnect.
///
/// See [`ServerEvent::Disconnected`].
#[derive(Derivative, Event)]
#[derivative(Debug(bound = "T::Error: Debug"), Clone(bound = "T::Error: Clone"))]
pub struct RemoteClientDisconnected<P, T>
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    /// Key of the client.
    pub client_key: T::ClientKey,
    /// Why the client lost connection.
    pub reason: T::Error,
}

/// The server received a message from a remote client.
///
/// See [`ServerEvent::Recv`].
#[derive(Derivative, Event)]
#[derivative(Debug(bound = "P::C2S: Debug"), Clone(bound = "P::C2S: Clone"))]
pub struct FromClient<P, T>
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    /// Key of the client.
    pub client_key: T::ClientKey,
    /// The message received.
    pub msg: P::C2S,
}

/// A client acknowledged that they have fully received a message sent by
/// us.
#[derive(Derivative, Event)]
#[derivative(Debug(bound = ""), Clone(bound = ""))]
pub struct AckFromClient<P, T>
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    /// Key of the client.
    pub client_key: T::ClientKey,
    /// Key of the sent message, obtained by [`ServerTransport::send`].
    pub msg_key: T::MessageKey,
}

/// [`ServerTransport::flush`] produced an error.
#[derive(Derivative, Event)]
#[derivative(Debug(bound = "T::Error: Debug"), Clone(bound = "T::Error: Clone"))]
pub struct ServerFlushError<P, T>
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    /// Error produced by [`ServerTransport::flush`].
    pub error: T::Error,
}

#[allow(clippy::too_many_arguments)]
fn recv<P, T>(
    mut server: ResMut<T>,
    mut opened: EventWriter<ServerOpened<P, T>>,
    mut closed: EventWriter<ServerClosed<P, T>>,
    mut connecting: EventWriter<RemoteClientConnecting<P, T>>,
    mut connected: EventWriter<RemoteClientConnected<P, T>>,
    mut disconnected: EventWriter<RemoteClientDisconnected<P, T>>,
    mut recv: EventWriter<FromClient<P, T>>,
    mut ack: EventWriter<AckFromClient<P, T>>,
) where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    for event in server.poll() {
        match event {
            ServerEvent::Opened => {
                opened.send(ServerOpened {
                    _phantom: PhantomData,
                });
            }
            ServerEvent::Closed { reason } => {
                closed.send(ServerClosed { reason });
            }
            ServerEvent::Connecting { client_key } => {
                connecting.send(RemoteClientConnecting { client_key });
            }
            ServerEvent::Connected { client_key } => {
                connected.send(RemoteClientConnected { client_key });
            }
            ServerEvent::Disconnected { client_key, reason } => {
                disconnected.send(RemoteClientDisconnected { client_key, reason });
            }
            ServerEvent::Recv { client_key, msg } => {
                recv.send(FromClient { client_key, msg });
            }
            ServerEvent::Ack {
                client_key,
                msg_key,
            } => {
                ack.send(AckFromClient {
                    client_key,
                    msg_key,
                });
            }
        }
    }
}

fn send<P, T>(mut server: ResMut<T>, mut errors: EventWriter<ServerFlushError<P, T>>)
where
    P: TransportProtocol,
    T: ServerTransport<P> + Resource,
{
    if let Err(error) = server.flush() {
        errors.send(ServerFlushError { error });
    }
}
