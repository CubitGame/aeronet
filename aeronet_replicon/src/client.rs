use std::marker::PhantomData;

use aeronet::{
    client::{
        client_connected, ClientEvent, ClientFlushError, ClientState, ClientTransport,
        ClientTransportSet, LocalClientConnected, LocalClientDisconnected,
    },
    protocol::TransportProtocol,
};
use bevy_app::prelude::*;
use bevy_ecs::prelude::*;
use bevy_replicon::{
    client::{
        replicon_client::{RepliconClient, RepliconClientStatus},
        ClientSet,
    },
    server::ServerSet,
};
use derivative::Derivative;

use crate::protocol::RepliconMessage;

// this REPLACES ClientTransportPlugin btw
#[derive(Derivative)]
#[derivative(Debug(bound = ""), Clone(bound = ""), Default(bound = ""))]
pub struct RepliconAeronetClientPlugin<P, T> {
    #[derivative(Debug = "ignore")]
    _phantom: PhantomData<(P, T)>,
}

impl<P, T> Plugin for RepliconAeronetClientPlugin<P, T>
where
    P: TransportProtocol<C2S = RepliconMessage, S2C = RepliconMessage>,
    T: ClientTransport<P> + Resource,
{
    fn build(&self, app: &mut App) {
        replicon_aeronet_client_plugin::<P, T>(app);
    }
}

pub fn replicon_aeronet_client_plugin<P, T>(app: &mut App)
where
    P: TransportProtocol<C2S = RepliconMessage, S2C = RepliconMessage>,
    T: ClientTransport<P> + Resource,
{
    app.configure_sets(
        PreUpdate,
        ClientSet::ReceivePackets.after(ClientTransportSet::Recv),
    )
    .configure_sets(
        PostUpdate,
        ClientSet::SendPackets.before(ClientTransportSet::Send),
    )
    .add_systems(
        PreUpdate,
        (
            recv::<P, T>.run_if(resource_exists::<T>),
            update_state::<P, T>.run_if(resource_exists::<T>),
            on_removed.run_if(resource_removed::<T>()),
        )
            .chain()
            .in_set(ServerSet::ReceivePackets),
    )
    .add_systems(
        PostUpdate,
        send::<P, T>
            .run_if(client_connected::<P, T>)
            .in_set(ServerSet::SendPackets),
    );
}

fn recv<P, T>(
    mut client: ResMut<T>,
    mut replicon: ResMut<RepliconClient>,
    mut connected: EventWriter<LocalClientConnected<P, T>>,
    mut disconnected: EventWriter<LocalClientDisconnected<P, T>>,
) where
    P: TransportProtocol<C2S = RepliconMessage, S2C = RepliconMessage>,
    T: ClientTransport<P> + Resource,
{
    for event in client.poll() {
        match event {
            ClientEvent::Connected => {
                connected.send(LocalClientConnected {
                    _phantom: PhantomData,
                });
            }
            ClientEvent::Disconnected { reason } => {
                disconnected.send(LocalClientDisconnected { reason });
            }
            ClientEvent::Recv { msg } => {
                replicon.insert_received(msg.channel_id, msg.payload);
            }
            ClientEvent::Ack { .. } => {}
        }
    }
}

fn update_state<P, T>(client: Res<T>, mut replicon: ResMut<RepliconClient>)
where
    P: TransportProtocol<C2S = RepliconMessage, S2C = RepliconMessage>,
    T: ClientTransport<P> + Resource,
{
    replicon.set_status(match client.state() {
        ClientState::Disconnected => RepliconClientStatus::Disconnected,
        ClientState::Connecting(_) => RepliconClientStatus::Connecting,
        ClientState::Connected(_) => RepliconClientStatus::Connected { client_id: None },
    });
}

fn on_removed(mut replicon: ResMut<RepliconClient>) {
    replicon.set_status(RepliconClientStatus::Disconnected);
}

fn send<P, T>(
    mut client: ResMut<T>,
    mut replicon: ResMut<RepliconClient>,
    mut flush_errors: EventWriter<ClientFlushError<P, T>>,
) where
    P: TransportProtocol<C2S = RepliconMessage, S2C = RepliconMessage>,
    T: ClientTransport<P> + Resource,
{
    for (channel_id, payload) in replicon.drain_sent() {
        // ignore send failures
        let _ = client.send(RepliconMessage {
            channel_id,
            payload,
        });
    }

    if let Err(error) = client.flush() {
        flush_errors.send(ClientFlushError { error });
    }
}
