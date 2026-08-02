//! ================================================================================================
//! Gameplay Player
//!
//! Defines the generic local player entity used by all game modes.
//!
//! Responsibilities
//! ----------------
//! • Spawn the player.
//! • Spawn the player camera.
//! • Despawn the player.
//!
//! This module intentionally contains no gameplay logic, input handling,
//! collision detection or game-mode-specific behaviour.
//! ================================================================================================

use bevy::prelude::*;

use crate::gameplay::physics::{Collider, Player, Velocity};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Player collision radius.
pub const PLAYER_RADIUS: f32 = 0.30;

/// Player eye height.
pub const PLAYER_HEIGHT: f32 = 1.70;

/// Walking speed in metres per second.
pub const WALK_SPEED: f32 = 3.5;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Marks the player's first-person camera.
#[derive(Component)]
pub struct PlayerCamera;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spawn
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_player(
    commands: &mut Commands,
    spawn_position: Vec3,
) {
    let player = commands
        .spawn((
            Name::new("Player"),

            Player,

            Velocity::default(),

            Collider {
                size: Vec2::splat(PLAYER_RADIUS * 2.0),
            },

            SpatialBundle {
                transform: Transform::from_translation(spawn_position),
                ..default()
            },
        ))
        .id();

    commands.entity(player).with_children(|parent| {
        parent.spawn((
            PlayerCamera,

            Camera3dBundle {
                camera: Camera {
                    hdr: true,
                    order: 0,
                    ..default()
                },

                transform: Transform::from_xyz(
                    0.0,
                    PLAYER_HEIGHT,
                    0.0,
                ),

                ..default()
            },
        ));
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Despawn
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn despawn_player(
    mut commands: Commands,
    players: Query<Entity, With<Player>>,
) {
    for entity in &players {
        commands.entity(entity).despawn_recursive();
    }
}