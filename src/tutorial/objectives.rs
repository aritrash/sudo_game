//! Tutorial objective management.
//!
//! This module defines the player-facing objectives used throughout the
//! Orientation Facility. Every tutorial stage corresponds to exactly one
//! objective, which is displayed to the player by the UI.
//!
//! This module intentionally contains no gameplay logic. It simply maps
//! tutorial stages to objective text and tracks completion status.

use bevy::prelude::*;

use crate::tutorial::stages::TutorialStage;

/// Represents the current objective shown to the player.
#[derive(Resource, Debug, Clone)]
pub struct TutorialObjective {
    /// Short title displayed in the objective panel.
    pub title: &'static str,

    /// Detailed description of the current objective.
    pub description: &'static str,

    /// Indicates whether the objective has been completed.
    pub completed: bool,
}

impl Default for TutorialObjective {
    fn default() -> Self {
        Self::from_stage(TutorialStage::default())
    }
}

impl TutorialObjective {
    /// Creates the appropriate objective for the supplied tutorial stage.
    pub fn from_stage(stage: TutorialStage) -> Self {
        use TutorialStage::*;

        match stage {
            Boot => Self {
                title: "System Boot",
                description: "Wait for the Orientation Facility to finish initializing.",
                completed: false,
            },

            Welcome => Self {
                title: "Orientation",
                description: "Listen to Pong's introduction.",
                completed: false,
            },

            Movement => Self {
                title: "Movement",
                description: "Move using the W, A, S and D keys.",
                completed: false,
            },

            Camera => Self {
                title: "Camera Control",
                description: "Move the mouse to look around the environment.",
                completed: false,
            },

            Interaction => Self {
                title: "Interaction",
                description: "Interact with the highlighted terminal.",
                completed: false,
            },

            Authentication => Self {
                title: "Authentication",
                description: "Read the sticky note and enter the correct password.",
                completed: false,
            },

            PacketPickup => Self {
                title: "Packet Pickup",
                description: "Collect the Data Packet.",
                completed: false,
            },

            PacketDelivery => Self {
                title: "Packet Delivery",
                description: "Deliver the Data Packet to the Uplink Terminal.",
                completed: false,
            },

            Infection => Self {
                title: "Infection",
                description: "Observe how infection affects a system process.",
                completed: false,
            },

            Quarantine => Self {
                title: "Quarantine",
                description: "Proceed to the Quarantine Room before the infection timer expires.",
                completed: false,
            },

            Freeze => Self {
                title: "Freeze",
                description: "Observe the temporary Freeze effect.",
                completed: false,
            },

            Meeting => Self {
                title: "Emergency Meeting",
                description: "Observe how Emergency Meetings operate.",
                completed: false,
            },

            Voting => Self {
                title: "Voting",
                description: "Cast your vote to continue the tutorial.",
                completed: false,
            },

            ChatRules => Self {
                title: "Communication",
                description: "Read the communication guidelines.",
                completed: false,
            },

            Graduation => Self {
                title: "Training Complete",
                description: "Proceed to the exit to complete Orientation.",
                completed: false,
            },

            Finished => Self {
                title: "Orientation Complete",
                description: "Return to the Main Menu.",
                completed: true,
            },
        }
    }

    /// Marks the current objective as completed.
    #[inline]
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Resets the completion state.
    #[inline]
    pub fn reset(&mut self) {
        self.completed = false;
    }

    /// Returns whether the current objective has been completed.
    #[inline]
    pub fn is_completed(&self) -> bool {
        self.completed
    }
}
