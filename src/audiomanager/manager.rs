//! Audio Manager
//!
//! Centralized audio playback system for S.U.D.O.
//!
//! This module owns all audio playback using bevy_kira_audio.
//! Gameplay systems should communicate only through audio events.

use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use bevy_kira_audio::AudioPlugin;

use crate::audiomanager::music::{MusicTrack, PlayMusicEvent, StopMusicEvent};

use crate::audiomanager::pong::{PlayPongVoiceEvent, PongVoice, StopPongVoiceEvent};

use crate::audiomanager::settings::AudioSettings;

use crate::audiomanager::sfx::{PlaySfxEvent, SoundEffect};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Audio Channels
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Resource)]
pub struct MusicChannel;

#[derive(Resource)]
pub struct SfxChannel;

#[derive(Resource)]
pub struct VoiceChannel;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Resources
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Resource, Default)]
pub struct CurrentMusic {
    pub track: Option<MusicTrack>,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct AudioManagerPlugin;

impl Plugin for AudioManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            //
            // Events
            //
            .add_event::<PlayMusicEvent>()
            .add_event::<StopMusicEvent>()
            .add_event::<PlaySfxEvent>()
            .add_event::<PlayPongVoiceEvent>()
            .add_event::<StopPongVoiceEvent>()
            //
            // Channels
            //
            .add_audio_channel::<MusicChannel>()
            .add_audio_channel::<SfxChannel>()
            .add_audio_channel::<VoiceChannel>()
            //
            // Resources
            //
            .init_resource::<CurrentMusic>()
            .init_resource::<AudioSettings>()
            //
            // Systems
            //
            .add_systems(
                Update,
                (
                    play_music_system,
                    stop_music_system,
                    play_sfx_system,
                    play_pong_voice_system,
                    stop_pong_voice_system,
                    update_audio_settings_system,
                ),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Music Playback
////////////////////////////////////////////////////////////////////////////////////////////////////

fn play_music_system(
    mut events: EventReader<PlayMusicEvent>,

    asset_server: Res<AssetServer>,

    music: Res<AudioChannel<MusicChannel>>,

    mut current_music: ResMut<CurrentMusic>,
) {
    for event in events.read() {
        //
        // Already playing?
        //
        if current_music.track == Some(event.track) {
            continue;
        }

        //
        // Stop previous music
        //
        music.stop();

        //
        // Resolve asset
        //
        let handle = asset_server.load(event.track.asset_path());

        //
        // Play looped
        //
        music.play(handle).looped();

        current_music.track = Some(event.track);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Stop Music
////////////////////////////////////////////////////////////////////////////////////////////////////

fn stop_music_system(
    mut events: EventReader<StopMusicEvent>,

    music: Res<AudioChannel<MusicChannel>>,

    mut current_music: ResMut<CurrentMusic>,
) {
    if events.is_empty() {
        return;
    }

    for _ in events.read() {
        music.stop();
        current_music.track = None;
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Sound Effects
////////////////////////////////////////////////////////////////////////////////////////////////////

fn play_sfx_system(
    mut events: EventReader<PlaySfxEvent>,

    asset_server: Res<AssetServer>,

    sfx: Res<AudioChannel<SfxChannel>>,
) {
    for event in events.read() {
        let handle = asset_server.load(event.effect.asset_path());

        sfx.play(handle);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Pong Voice
////////////////////////////////////////////////////////////////////////////////////////////////////

fn play_pong_voice_system(
    mut events: EventReader<PlayPongVoiceEvent>,

    asset_server: Res<AssetServer>,

    voice: Res<AudioChannel<VoiceChannel>>,
) {
    for event in events.read() {
        //
        // Interrupt any currently playing narration.
        //
        voice.stop();

        let handle = asset_server.load(event.clip.asset_path());

        voice.play(handle);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Stop Pong Voice
////////////////////////////////////////////////////////////////////////////////////////////////////

fn stop_pong_voice_system(
    mut events: EventReader<StopPongVoiceEvent>,

    voice: Res<AudioChannel<VoiceChannel>>,
) {
    if events.is_empty() {
        return;
    }

    for _ in events.read() {
        voice.stop();
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Audio Settings
////////////////////////////////////////////////////////////////////////////////////////////////////

fn update_audio_settings_system(
    settings: Res<AudioSettings>,

    music: Res<AudioChannel<MusicChannel>>,
    sfx: Res<AudioChannel<SfxChannel>>,
    voice: Res<AudioChannel<VoiceChannel>>,
) {
    if !settings.is_changed() {
        return;
    }

    //
    // Master Volume
    //
    let master_volume = if settings.master_muted {
        0.0
    } else {
        settings.master_volume
    };

    //
    // Music Volume
    //
    let music_volume = if settings.music_muted {
        0.0
    } else {
        master_volume * settings.music_volume
    };

    //
    // Sound Effects Volume
    //
    let sfx_volume = if settings.sfx_muted {
        0.0
    } else {
        master_volume * settings.sfx_volume
    };

    //
    // Voice Volume
    //
    let voice_volume = if settings.voice_muted {
        0.0
    } else {
        master_volume * settings.voice_volume
    };

    music.set_volume(music_volume as f64);

    sfx.set_volume(sfx_volume as f64);

    voice.set_volume(voice_volume as f64);
}
