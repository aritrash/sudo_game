//! ================================================================================================
//! Gameplay Controller
//!
//! Maps player input to gameplay movement.
//!
//! Responsibilities
//! ----------------
//! • Read PlayerInput.
//! • Update the local player's Velocity.
//!
//! This module intentionally contains no collision, physics,
//! rendering or interaction logic.
//! ================================================================================================

use bevy::prelude::*;

use crate::gameplay::inputs::PlayerInput;
use crate::gameplay::physics::{Player, Velocity};
use crate::gameplay::player::WALK_SPEED;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Player Controller
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn player_controller_system(
    input: Res<PlayerInput>,
    mut players: Query<&mut Velocity, With<Player>>,
) {
    for mut velocity in &mut players {
        velocity.linear = input.movement * WALK_SPEED;
    }
}