//! Tutorial player state.
//!
//! This module stores all tutorial-specific state associated with the
//! local player.
//!
//! It intentionally contains no movement, physics, rendering or
//! networking logic.

use bevy::prelude::*;

use crate::tutorial::arena::TutorialRoom;
use crate::tutorial::stages::TutorialStage;

/// Tutorial-specific player state.
#[derive(Resource, Debug)]
pub struct TutorialPlayer {
    /// Current room occupied by the player.
    pub room: TutorialRoom,

    /// Whether the player is carrying the Data Packet.
    pub carrying_packet: bool,

    /// Whether the player is currently infected.
    pub infected: bool,

    /// Whether the player is currently frozen.
    pub frozen: bool,

    /// Whether the player is inside Quarantine.
    pub quarantined: bool,

    /// Whether the player may interact with objects.
    pub can_interact: bool,

    /// Whether the tutorial has been completed.
    pub graduated: bool,
}

impl Default for TutorialPlayer {
    fn default() -> Self {
        Self {
            room: TutorialRoom::SpawnLobby,
            carrying_packet: false,
            infected: false,
            frozen: false,
            quarantined: false,
            can_interact: true,
            graduated: false,
        }
    }
}

impl TutorialPlayer {
    /// Gives the player the Data Packet.
    #[inline]
    pub fn pick_up_packet(&mut self) {
        self.carrying_packet = true;
    }

    /// Removes the Data Packet.
    #[inline]
    pub fn drop_packet(&mut self) {
        self.carrying_packet = false;
    }

    /// Infects the player.
    #[inline]
    pub fn infect(&mut self) {
        self.infected = true;
    }

    /// Clears the infection state.
    #[inline]
    pub fn cure(&mut self) {
        self.infected = false;
    }

    /// Freezes the player.
    #[inline]
    pub fn freeze(&mut self) {
        self.frozen = true;
    }

    /// Unfreezes the player.
    #[inline]
    pub fn unfreeze(&mut self) {
        self.frozen = false;
    }

    /// Moves the player into Quarantine.
    #[inline]
    pub fn enter_quarantine(&mut self) {
        self.quarantined = true;
    }

    /// Leaves Quarantine.
    #[inline]
    pub fn leave_quarantine(&mut self) {
        self.quarantined = false;
    }

    /// Completes Orientation.
    #[inline]
    pub fn graduate(&mut self) {
        self.graduated = true;
    }
}
