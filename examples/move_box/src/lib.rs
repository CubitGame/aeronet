#![doc = include_str!("../README.md")]

use aeronet_replicon::channel::IntoLanes;
use aeronet_webtransport::{proto::session::SessionConfig, runtime::WebTransportRuntime};
use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

/// How many units a player may move in a single second.
const MOVE_SPEED: f32 = 5000.0;

/// How many times per second we will replicate entity components.
pub const TICK_RATE: u16 = 128;

/// Sets up replication and basic game systems.
#[derive(Debug)]
pub struct MoveBoxPlugin;

/// Whether the game is currently being simulated or not.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq, Hash, States)]
pub enum GameState {
    /// Game is not being simulated.
    #[default]
    None,
    /// Game is being simulated.
    Playing,
}

/// Creates a [`SessionConfig`] from [`RepliconChannels`], customized for this
/// app.
///
/// Both the client and server should have the same [`SessionConfig`].
#[must_use]
pub fn session_config(channels: &RepliconChannels) -> SessionConfig {
    SessionConfig::default()
        .with_client_lanes(channels.client_channels().into_lanes())
        .with_server_lanes(channels.server_channels().into_lanes())
}

impl Plugin for MoveBoxPlugin {
    fn build(&self, app: &mut App) {
        app.init_state::<GameState>()
            .enable_state_scoped_entities::<GameState>()
            // use the convenience resource WebTransportRuntime for spawning tasks
            // platform-independently (native using tokio, or WASM using wasm-bindgen-futures)
            .init_resource::<WebTransportRuntime>()
            .replicate::<Player>()
            .replicate::<PlayerPosition>()
            .replicate::<PlayerColor>()
            .add_client_event::<PlayerMove>(ChannelKind::Ordered)
            .add_systems(Update, apply_movement.run_if(has_authority));
    }
}

/// Marker component for a player in the game.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct Player;

/// Player who is being controlled by a specific [`ClientId`] connected to our
/// server.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct ClientPlayer(pub ClientId);

/// Player's box position.
#[derive(Debug, Clone, Component, Deref, DerefMut, Serialize, Deserialize)]
pub struct PlayerPosition(pub Vec2);

/// Player's box color.
#[derive(Debug, Clone, Component, Deref, DerefMut, Serialize, Deserialize)]
pub struct PlayerColor(pub Color);

/// Player sent an input to move their box.
#[derive(Debug, Clone, Event, Serialize, Deserialize)]
pub struct PlayerMove(pub Vec2);

fn apply_movement(
    time: Res<Time>,
    mut move_events: EventReader<FromClient<PlayerMove>>,
    mut players: Query<(&ClientPlayer, &mut PlayerPosition)>,
) {
    for FromClient {
        client_id,
        event: PlayerMove(delta),
    } in move_events.read()
    {
        for (player, mut position) in &mut players {
            if *client_id == player.0 {
                // make sure to normalize on the server side;
                // since we're accepting arbitrary client input,
                // we have to do input validation on the server side
                **position += delta.normalize_or_zero() * time.delta_seconds() * MOVE_SPEED;
            }
        }
    }
}
