//! ================================================================================================
//! Orientation Plugin
//!
//! Coordinates the Orientation Facility game mode.
//!
//! Responsibilities
//! ----------------
//! • Spawn the Orientation Facility.
//! • Spawn the Orientation player.
//! • Initialize Orientation state.
//! • Register Orientation-specific plugins.
//! • Clean up everything when Orientation exits.
//!
//! This module intentionally contains no gameplay logic.
//! Movement, collision and interaction are provided by the Gameplay
//! module.
//! ================================================================================================

use bevy::prelude::*;

use crate::maps::orientation::{
    build_orientation_facility,
    despawn_orientation_facility,
};

use crate::gameplay::player::{
    spawn_player,
    despawn_player,
};

use crate::AppState;
use crate::maps::orientation::spawn_lobby::PLAYER_SPAWN_POSITION;

mod dialogue;
mod objectives;
mod progression;
mod stages;
mod ui;

pub use dialogue::OrientationDialoguePlugin;
pub use progression::OrientationProgressionPlugin;
pub use ui::OrientationUiPlugin;

pub struct OrientationPlugin;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Player Spawn
////////////////////////////////////////////////////////////////////////////////////////////////////

fn spawn_orientation_player(
    mut commands: Commands,
) {
    spawn_player(
        &mut commands,
        PLAYER_SPAWN_POSITION,
    );
}

impl Plugin for OrientationPlugin {
    fn build(&self, app: &mut App) {
        app

            ////////////////////////////////////////////////////////////////
            // Resources
            ////////////////////////////////////////////////////////////////

            .init_resource::<stages::OrientationStage>()
            .init_resource::<objectives::OrientationObjective>()

            ////////////////////////////////////////////////////////////////
            // Orientation Subsystems
            ////////////////////////////////////////////////////////////////

            .add_plugins((
                OrientationUiPlugin,
                OrientationProgressionPlugin,
                OrientationDialoguePlugin,
            ))

            ////////////////////////////////////////////////////////////////
            // Lifecycle
            ////////////////////////////////////////////////////////////////

            .add_systems(
                OnEnter(AppState::Tutorial),
                (
                    build_orientation_facility,
                    spawn_orientation_player,
                ),
            )

            .add_systems(
                OnExit(AppState::Tutorial),
                (
                    despawn_orientation_facility,
                    despawn_player,
                ),
            );
    }
}