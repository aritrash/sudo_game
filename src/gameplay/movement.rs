//! ================================================================================================
//! S.U.D.O.
//! Gameplay Movement
//!
//! Moves the local player using the current input state.
//!
//! Responsibilities
//! ----------------
//! • Read PlayerInput.
//! • Move the local player.
//!
//! This module intentionally contains no collision, interaction,
//! camera or networking logic.
//! ================================================================================================

use bevy::prelude::*;

use crate::gameplay::input::PlayerInput;
use crate::gameplay::player::{
    LocalPlayer,
    PLAYER_WALK_SPEED,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct GameplayMovementPlugin;

impl Plugin for GameplayMovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            player_movement_system,
        );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn player_movement_system(
    time: Res<Time>,
    input: Res<PlayerInput>,
    mut players: Query<&mut Transform, With<LocalPlayer>>,
) {
    for mut transform in &mut players {

        let forward = transform.forward();
        let right = transform.right();

        let movement =
            (forward * input.movement.y)
            + (right * input.movement.x);

        transform.translation +=
            movement.normalize_or_zero()
            * PLAYER_WALK_SPEED
            * time.delta_seconds();
    }
}