//! Background music definitions.

use bevy::prelude::*;

/// All background music tracks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MusicTrack {
    MainMenu,
    Gameplay,
}

impl MusicTrack {
    /// Returns the asset path for this music track.
    pub fn asset_path(self) -> &'static str {
        match self {
            MusicTrack::MainMenu => {
                "audio/music/mainmenu.ogg"
            }

            MusicTrack::Gameplay => {
                "audio/music/gameplay.ogg"
            }
        }
    }
}

/// Requests playback of background music.
#[derive(Event)]
pub struct PlayMusicEvent {
    pub track: MusicTrack,
}

/// Requests that current music be stopped.
#[derive(Event)]
pub struct StopMusicEvent;