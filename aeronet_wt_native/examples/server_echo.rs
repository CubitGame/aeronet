//!

use std::{convert::Infallible, string::FromUtf8Error, time::Duration};

use aeronet::{
    ClientState, FromClient, LaneKey, LaneProtocol, Message, OnLane, RemoteConnected,
    RemoteConnecting, RemoteDisconnected, ServerClosed, ServerOpened, ServerTransport,
    ServerTransportPlugin, TokioRuntime, TransportProtocol, TryAsBytes, TryFromBytes,
};
use aeronet_wt_native::WebTransportServer;
use anyhow::Result;
use bevy::{app::ScheduleRunnerPlugin, log::LogPlugin, prelude::*};
use wtransport::{tls::Certificate, ServerConfig};

// config

#[derive(Debug, Clone, LaneKey)]
#[lane_kind(ReliableOrdered)]
struct AppLane;

#[derive(Debug, Clone, Message, OnLane)]
#[lane_type(AppLane)]
#[on_lane(AppLane)]
struct AppMessage(String);

impl<T: Into<String>> From<T> for AppMessage {
    fn from(value: T) -> Self {
        Self(value.into())
    }
}

impl TryAsBytes for AppMessage {
    type Output<'a> = &'a [u8];
    type Error = Infallible;

    fn try_as_bytes(&self) -> Result<Self::Output<'_>, Self::Error> {
        Ok(self.0.as_bytes())
    }
}

impl TryFromBytes for AppMessage {
    type Error = FromUtf8Error;

    fn try_from_bytes(buf: &[u8]) -> Result<Self, Self::Error>
    where
        Self: Sized,
    {
        String::from_utf8(buf.to_vec()).map(AppMessage)
    }
}

struct AppProtocol;

impl TransportProtocol for AppProtocol {
    type C2S = AppMessage;
    type S2C = AppMessage;
}

impl LaneProtocol for AppProtocol {
    type Lane = AppLane;
}

// logic

/*
chromium \
brave \
--webtransport-developer-mode \
--ignore-certificate-errors-spki-list=x3S9HPqXZTYoR2tOQMmVG2GiZDPyyksnWdF9I9Ko/xY=
*/

fn main() {
    App::new()
        .add_plugins((
            LogPlugin {
                filter: "aeronet=debug".into(),
                ..default()
            },
            MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_millis(100))),
            ServerTransportPlugin::<AppProtocol, WebTransportServer<_>>::default(),
        ))
        .init_resource::<TokioRuntime>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                on_opened,
                on_closed,
                on_incoming,
                on_connected,
                on_disconnected,
                on_recv,
            ),
        )
        .run();
}

fn setup(mut commands: Commands, rt: Res<TokioRuntime>) {
    match create(rt.as_ref()) {
        Ok(server) => {
            info!("Created server");
            commands.insert_resource(server);
        }
        Err(err) => panic!("Failed to create server: {err:#}"),
    }
}

fn create(rt: &TokioRuntime) -> Result<WebTransportServer<AppProtocol>> {
    let cert = rt.0.block_on(Certificate::load(
        "./aeronet_wt_native/examples/cert.pem",
        "./aeronet_wt_native/examples/key.pem",
    ))?;

    let config = ServerConfig::builder()
        .with_bind_default(25565)
        .with_certificate(cert)
        .keep_alive_interval(Some(Duration::from_secs(5)))
        .build();

    let (server, backend) = WebTransportServer::open_new(config);
    rt.0.spawn(backend);

    Ok(server)
}

fn on_opened(mut events: EventReader<ServerOpened<AppProtocol, WebTransportServer<AppProtocol>>>) {
    for ServerOpened { .. } in events.read() {
        info!("Opened server for connections");
    }
}

fn on_closed(mut events: EventReader<ServerClosed<AppProtocol, WebTransportServer<AppProtocol>>>) {
    for ServerClosed { reason } in events.read() {
        info!("Server closed: {:#}", aeronet::util::pretty_error(&reason))
    }
}

fn on_incoming(
    mut events: EventReader<RemoteConnecting<AppProtocol, WebTransportServer<AppProtocol>>>,
    mut server: ResMut<WebTransportServer<AppProtocol>>,
) {
    for RemoteConnecting { client, .. } in events.read() {
        if let ClientState::Connecting(info) = server.client_state(*client) {
            info!(
                "Client {client} incoming from {}{} ({:?})",
                info.authority, info.path, info.origin,
            );
        }
        let _ = server.accept_request(*client);
    }
}

fn on_connected(
    mut events: EventReader<RemoteConnected<AppProtocol, WebTransportServer<AppProtocol>>>,
    mut server: ResMut<WebTransportServer<AppProtocol>>,
) {
    for RemoteConnected { client, .. } in events.read() {
        if let ClientState::Connected(info) = server.client_state(*client) {
            info!(
                "Client {client} connected on {} (RTT: {:?})",
                info.remote_addr, info.rtt
            );
        };
        let _ = server.send(*client, "Welcome!");
        let _ = server.send(*client, "Send me some UTF-8 text, and I will send it back");
    }
}

fn on_disconnected(
    mut events: EventReader<RemoteDisconnected<AppProtocol, WebTransportServer<AppProtocol>>>,
) {
    for RemoteDisconnected { client, reason } in events.read() {
        info!(
            "Client {client} disconnected: {:#}",
            aeronet::util::pretty_error(reason)
        );
    }
}

fn on_recv(
    mut events: EventReader<FromClient<AppProtocol, WebTransportServer<AppProtocol>>>,
    mut server: ResMut<WebTransportServer<AppProtocol>>,
) {
    for FromClient { client, msg, .. } in events.read() {
        info!("{client} > {}", msg.0);
        let resp = format!("You sent: {}", msg.0);
        info!("{client} < {resp}");
        let _ = server.send(*client, AppMessage(resp));
    }
}
