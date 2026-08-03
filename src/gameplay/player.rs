//! ================================================================================================
//! S.U.D.O.
//! Gameplay Player
//!
//! Defines the generic player entity used throughout S.U.D.O.
//!
//! Responsibilities
//! ----------------
//! • Spawn players.
//! • Spawn the local first-person camera.
//! • Define player components.
//! • Despawn players.
//!
//! This module intentionally contains no input, movement,
//! collision, interaction or networking logic.
//! ================================================================================================

use bevy::prelude::*;
use crate::gameplay::camera::{CameraController, CameraTarget};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Player eye height above the floor.
pub const PLAYER_EYE_HEIGHT: f32 = 1.70;

/// Walking speed (used later by movement.rs).
pub const PLAYER_WALK_SPEED: f32 = 3.5;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Generic player marker.
#[derive(Component)]
pub struct Player;

/// Marks the player controlled by this client.
#[derive(Component)]
pub struct LocalPlayer;

/// Marks the first-person camera.
#[derive(Component)]
pub struct PlayerCamera;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spawn
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_player(
    commands: &mut Commands,
    position: Vec3,
) {
    let player = commands.spawn((
        Player,
        LocalPlayer,
        CameraController::default(),
        SpatialBundle {
            transform: Transform::from_translation(position),
            ..default()
        },
    )).id();

    commands.spawn((
        PlayerCamera,
        CameraTarget(player),
        Camera3dBundle {
            transform: Transform::from_xyz(
                position.x,
                position.y + PLAYER_EYE_HEIGHT,
                position.z,
            ),
            ..default()
        },
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Despawn
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn despawn_players(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    for entity in &players {
        commands.entity(entity).despawn_recursive();
    }
}