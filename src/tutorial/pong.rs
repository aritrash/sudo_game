//! Pong dialogue definitions.
//!
//! This module contains the complete dialogue script used by Pong,
//! the Orientation Instructor of the S.U.D.O. System.
//!
//! It intentionally contains no gameplay logic, UI systems or audio
//! playback. It serves as the single source of truth for all tutorial
//! dialogue and associated voice-over assets.

use crate::tutorial::stages::TutorialStage;

/// A single dialogue message spoken by Pong.
#[derive(Debug, Clone, Copy)]
pub struct PongMessage {
    /// Speaker name displayed in the dialogue window.
    pub speaker: &'static str,

    /// Dialogue title.
    pub title: &'static str,

    /// Dialogue body.
    pub body: &'static str,

    /// Path to the corresponding voice-over asset.
    ///
    /// The path is relative to the `assets/` directory and can be loaded
    /// directly using Bevy's `AssetServer`.
    ///
    /// Example:
    /// ```text
    /// audio/boot.ogg
    /// ```
    pub voice_path: Option<&'static str>,
}

/// Returns Pong's dialogue for the given tutorial stage.
pub fn message(stage: TutorialStage) -> PongMessage {
    use TutorialStage::*;

    match stage {
        Boot => PongMessage {
            speaker: "PONG",
            title: "System Boot",
            body: "Orientation Facility online.\n\nPlease wait while your execution environment is initialized.",
            voice_path: Some("audio/boot.ogg"),
        },

        Welcome => PongMessage {
            speaker: "PONG",
            title: "Welcome",
            body: "Welcome, Process.\n\nI am Pong, your Orientation Instructor.\n\nToday you will learn the standard operating procedures required for normal System operation.",
            voice_path: Some("audio/welcome.ogg"),
        },

        Movement => PongMessage {
            speaker: "PONG",
            title: "Movement",
            body: "Every process must be capable of efficient navigation.\n\nMove using the W, A, S and D keys.",
            voice_path: Some("audio/movement.ogg"),
        },

        Camera => PongMessage {
            speaker: "PONG",
            title: "Observation",
            body: "Situational awareness improves System stability.\n\nMove your mouse to observe your surroundings.",
            voice_path: Some("audio/camera.ogg"),
        },

        Interaction => PongMessage {
            speaker: "PONG",
            title: "Interaction",
            body: "Many System devices require direct interaction.\n\nProceed to the highlighted terminal.",
            voice_path: Some("audio/interaction.ogg"),
        },

        Authentication => PongMessage {
            speaker: "PONG",
            title: "Authentication",
            body: "Authentication protects critical infrastructure.\n\nLocate the password and authenticate yourself.",
            voice_path: Some("audio/authentication.ogg"),
        },

        PacketPickup => PongMessage {
            speaker: "PONG",
            title: "Data Packet",
            body: "Information moves through the System as Data Packets.\n\nCollect the packet before continuing.",
            voice_path: Some("audio/packet_pickup.ogg"),
        },

        PacketDelivery => PongMessage {
            speaker: "PONG",
            title: "Packet Delivery",
            body: "Deliver the Data Packet to the designated Uplink Terminal.\n\nReliable delivery keeps the System operational.",
            voice_path: Some("audio/packet_delivery.ogg"),
        },

        Infection => PongMessage {
            speaker: "PONG",
            title: "System Integrity",
            body: "Occasionally, processes become compromised.\n\nRemain calm and follow established procedures.",
            voice_path: Some("audio/infection.ogg"),
        },

        Quarantine => PongMessage {
            speaker: "PONG",
            title: "Quarantine",
            body: "Compromised processes must immediately report to Quarantine.\n\nThis protects the integrity of the System.",
            voice_path: Some("audio/quarantine.ogg"),
        },

        Freeze => PongMessage {
            speaker: "PONG",
            title: "Freeze",
            body: "Certain System events may temporarily suspend process execution.\n\nThis condition is known as Freeze.",
            voice_path: Some("audio/freeze.ogg"),
        },

        Meeting => PongMessage {
            speaker: "PONG",
            title: "Emergency Meeting",
            body: "Emergency Meetings allow active processes to discuss unusual behaviour.\n\nObserve the demonstration carefully.",
            voice_path: Some("audio/meeting.ogg"),
        },

        Voting => PongMessage {
            speaker: "PONG",
            title: "Voting",
            body: "Every active process has one vote.\n\nUse your judgement before making a decision.",
            voice_path: Some("audio/voting.ogg"),
        },

        ChatRules => PongMessage {
            speaker: "PONG",
            title: "Communication",
            body: "Maintain respectful communication.\n\nClear information helps preserve System stability.",
            voice_path: Some("audio/chat_rules.ogg"),
        },

        Graduation => PongMessage {
            speaker: "PONG",
            title: "Orientation Complete",
            body: "Congratulations.\n\nYou have successfully completed Orientation.\n\nThe System is now ready to assign you to operational duties.",
            voice_path: Some("audio/graduation.ogg"),
        },

        Finished => PongMessage {
            speaker: "PONG",
            title: "Good Luck",
            body: "Remember:\n\nNot every process behaves according to specification.\n\nRemain vigilant.",
            voice_path: Some("audio/finished.ogg"),
        },
    }
}
