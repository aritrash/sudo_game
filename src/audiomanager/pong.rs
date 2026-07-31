//! Pong voice definitions.

use bevy::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum PongVoice {
    Boot,
    Movement,
    Camera,
    Interaction,
    Authentication,
    PacketPickup,
    PacketDelivery,
    Infection,
    Quarantine,
    Freeze,
    EmergencyMeeting,
    Communication,
    Graduation,
    Completion,
}

impl PongVoice {
    pub fn asset_path(self) -> &'static str {
        match self {
            PongVoice::Boot => "audio/pong/boot_01.ogg",

            PongVoice::Movement => "audio/pong/movement_01.ogg",

            PongVoice::Camera => "audio/pong/camera_01.ogg",

            PongVoice::Interaction => "audio/pong/interaction_01.ogg",

            PongVoice::Authentication => {
                "audio/pong/authentication_01.ogg"
            }

            PongVoice::PacketPickup => {
                "audio/pong/packet_pickup_01.ogg"
            }

            PongVoice::PacketDelivery => {
                "audio/pong/packet_delivery_01.ogg"
            }

            PongVoice::Infection => {
                "audio/pong/infection.ogg"
            }

            PongVoice::Quarantine => {
                "audio/pong/quarantine_01.ogg"
            }

            PongVoice::Freeze => {
                "audio/pong/freeze_01.ogg"
            }

            PongVoice::EmergencyMeeting => {
                "audio/pong/emergency_meetings_01.ogg"
            }

            PongVoice::Communication => {
                "audio/pong/communication_01.ogg"
            }

            PongVoice::Graduation => {
                "audio/pong/graduation_01.ogg"
            }

            PongVoice::Completion => {
                "audio/pong/completion_01.ogg"
            }
        }
    }
}

#[derive(Event)]
pub struct PlayPongVoiceEvent {
    pub clip: PongVoice,
}

#[derive(Event)]
pub struct StopPongVoiceEvent;