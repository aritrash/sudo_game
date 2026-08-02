//! ================================================================================================
//! S.U.D.O.
//! Orientation Facility
//!
//! This module defines the complete single-player Orientation Facility.
//!
//! The Orientation Facility is responsible only for:
//!
//!     • Constructing the Orientation map.
//!     • Spawning the player.
//!     • Managing tutorial-specific entities.
//!
//! Gameplay systems (movement, collision, input), audio playback,
//! and UI are handled by their respective modules.
//! ================================================================================================

use bevy::prelude::*;
use crate::AppState;

pub mod map;

pub mod spawn_lobby;
pub mod movement_hall;
pub mod observation_room;
pub mod authentication_room;
pub mod packet_storage;
pub mod uplink_room;
pub mod quarantine_room;
pub mod freeze_chamber;
pub mod meeting_room;
pub mod graduation_hall;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OrientationPlugin;

impl Plugin for OrientationPlugin {
    fn build(&self, app: &mut App) {
        app

            ////////////////////////////////////////////////////////////////////
            // Enter Orientation Facility
            ////////////////////////////////////////////////////////////////////

            .add_systems(
                OnEnter(AppState::Tutorial),
                map::build_orientation_facility,
            )

            ////////////////////////////////////////////////////////////////////
            // Leave Orientation Facility
            ////////////////////////////////////////////////////////////////////

            .add_systems(
                OnExit(AppState::Tutorial),
                map::despawn_orientation_facility,
            );
    }
}