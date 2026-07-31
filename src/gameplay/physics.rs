//! ============================================================
//! S.U.D.O.
//! Gameplay Physics
//!
//! Provides simple movement and collision handling.
//!
//! This is NOT a rigid-body physics engine.
//! There is:
//!     • No gravity
//!     • No jumping
//!     • No acceleration
//!     • No bouncing
//!     • No forces
//!
//! Movement is purely top-down and collision-based.
//! ============================================================

use bevy::prelude::*;


/// Marker component identifying the local player.
#[derive(Component)]
pub struct Player;


/// Velocity in world space (units / second).
#[derive(Component, Default, Debug)]
pub struct Velocity {
    pub linear: Vec2,
}


/// Axis-aligned collision box.
#[derive(Component, Debug)]
pub struct Collider {
    pub size: Vec2,
}

pub fn movement_system(
    time: Res<Time>,
    mut query: Query<(&Velocity, &mut Transform)>,
) {
    for (velocity, mut transform) in &mut query {

        let delta =
            velocity.linear
            * time.delta_seconds();

        transform.translation.x += delta.x;
        transform.translation.y += delta.y;
    }
}

pub fn collision_system(
    mut query: Query<(&Collider, &mut Transform), With<Player>>,
) {

    // Placeholder.

    // Phase 1:
    // Prevent leaving map.

    // Phase 2:
    // Prevent walking through walls.

    // Phase 3:
    // Prevent walking through other players.

    // Phase 4:
    // Trigger interaction zones.
}