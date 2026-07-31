//! Sound effect definitions.

use bevy::prelude::*;

/// Available sound effects.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum SoundEffect {
    ButtonClick,
}

impl SoundEffect {
    /// Returns the asset path.
    pub fn asset_path(self) -> &'static str {
        match self {
            SoundEffect::ButtonClick => {
                "audio/sfx/click_button.ogg"
            }
        }
    }
}

/// Requests playback of a sound effect.
#[derive(Event)]
pub struct PlaySfxEvent {
    pub effect: SoundEffect,
}