//! Orientation Facility definition.
//!
//! This module defines the logical layout of the S.U.D.O.
//! Orientation Facility.
//!
//! It intentionally contains no rendering, physics, gameplay,
//! or Bevy scene construction.
//!
//! Every room is identified by a unique identifier and unlocked
//! at a particular tutorial stage.

use crate::tutorial::stages::TutorialStage;

/// Logical rooms within the Orientation Facility.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TutorialRoom {
    SpawnLobby,
    MovementHall,
    ObservationRoom,
    AuthenticationRoom,
    PacketStorage,
    UplinkRoom,
    QuarantineRoom,
    FreezeChamber,
    MeetingRoom,
    GraduationHall,
}

/// Logical interactable objects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum TutorialObject {
    PongCapsule,
    AuthenticationTerminal,
    StickyNote,
    DataPacket,
    UplinkTerminal,
    EmergencyButton,
    ExitDoor,
}

/// Describes one room of the Orientation Facility.
#[derive(Debug)]
pub struct RoomDefinition {
    /// Logical room identifier.
    pub room: TutorialRoom,

    /// Human-readable room name.
    pub name: &'static str,

    /// Stage at which this room becomes accessible.
    pub unlock_stage: TutorialStage,

    /// Objects contained within this room.
    pub objects: &'static [TutorialObject],
}

/// Complete Orientation Facility specification.
pub const ORIENTATION_FACILITY: &[RoomDefinition] = &[
    RoomDefinition {
        room: TutorialRoom::SpawnLobby,
        name: "Spawn Lobby",
        unlock_stage: TutorialStage::Boot,
        objects: &[TutorialObject::PongCapsule],
    },
    RoomDefinition {
        room: TutorialRoom::MovementHall,
        name: "Movement Hall",
        unlock_stage: TutorialStage::Movement,
        objects: &[],
    },
    RoomDefinition {
        room: TutorialRoom::ObservationRoom,
        name: "Observation Room",
        unlock_stage: TutorialStage::Camera,
        objects: &[],
    },
    RoomDefinition {
        room: TutorialRoom::AuthenticationRoom,
        name: "Authentication Room",
        unlock_stage: TutorialStage::Interaction,
        objects: &[
            TutorialObject::AuthenticationTerminal,
            TutorialObject::StickyNote,
        ],
    },
    RoomDefinition {
        room: TutorialRoom::PacketStorage,
        name: "Packet Storage",
        unlock_stage: TutorialStage::PacketPickup,
        objects: &[TutorialObject::DataPacket],
    },
    RoomDefinition {
        room: TutorialRoom::UplinkRoom,
        name: "Uplink Room",
        unlock_stage: TutorialStage::PacketDelivery,
        objects: &[TutorialObject::UplinkTerminal],
    },
    RoomDefinition {
        room: TutorialRoom::QuarantineRoom,
        name: "Quarantine",
        unlock_stage: TutorialStage::Quarantine,
        objects: &[],
    },
    RoomDefinition {
        room: TutorialRoom::FreezeChamber,
        name: "Freeze Demonstration Chamber",
        unlock_stage: TutorialStage::Freeze,
        objects: &[],
    },
    RoomDefinition {
        room: TutorialRoom::MeetingRoom,
        name: "Emergency Meeting Chamber",
        unlock_stage: TutorialStage::Meeting,
        objects: &[TutorialObject::EmergencyButton],
    },
    RoomDefinition {
        room: TutorialRoom::GraduationHall,
        name: "Graduation Hall",
        unlock_stage: TutorialStage::Graduation,
        objects: &[TutorialObject::ExitDoor],
    },
];
