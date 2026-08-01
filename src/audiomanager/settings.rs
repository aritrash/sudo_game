//! Global audio configuration.

use bevy::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Audio Settings
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Resource, Debug, Clone)]
pub struct AudioSettings {
    /// Overall output volume.
    pub master_volume: f64,

    /// Background music volume.
    pub music_volume: f64,

    /// Sound effects volume.
    pub sfx_volume: f64,

    /// Pong narration volume.
    pub voice_volume: f64,

    /// Master mute.
    pub master_muted: bool,

    /// Music mute.
    pub music_muted: bool,

    /// Sound effect mute.
    pub sfx_muted: bool,

    /// Voice mute.
    pub voice_muted: bool,
}

impl Default for AudioSettings {
    fn default() -> Self {
        Self {
            master_volume: 1.0,
            music_volume: 1.0,
            sfx_volume: 1.0,
            voice_volume: 1.0,

            master_muted: false,
            music_muted: false,
            sfx_muted: false,
            voice_muted: false,
        }
    }
}
