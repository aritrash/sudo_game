//! Tutorial stage definitions.
//!
//! This module defines the ordered sequence of objectives that make up
//! the S.U.D.O. Orientation Facility. Every stage represents exactly one
//! player objective. Tutorial progression is achieved by advancing from
//! one stage to the next once the current objective has been completed.

use bevy::prelude::*;

/// Represents the current stage of the tutorial.
#[derive(Resource, Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum OrientationStage {
    #[default]
    Boot,
    Welcome,
    Movement,
    Camera,
    Interaction,
    Authentication,
    PacketPickup,
    PacketDelivery,
    Infection,
    Quarantine,
    Freeze,
    Meeting,
    Voting,
    ChatRules,
    Graduation,
    Finished,
}

impl OrientationStage {
    /// Returns the next tutorial stage.
    pub fn next(self) -> Self {
        use OrientationStage::*;

        match self {
            Boot => Welcome,
            Welcome => Movement,
            Movement => Camera,
            Camera => Interaction,
            Interaction => Authentication,
            Authentication => PacketPickup,
            PacketPickup => PacketDelivery,
            PacketDelivery => Infection,
            Infection => Quarantine,
            Quarantine => Freeze,
            Freeze => Meeting,
            Meeting => Voting,
            Voting => ChatRules,
            ChatRules => Graduation,
            Graduation => Finished,
            Finished => Finished,
        }
    }
}
