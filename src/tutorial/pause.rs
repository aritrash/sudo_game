//! Tutorial Pause Management.
//!
//! This module manages pausing and resuming the Orientation Facility.
//! Pressing ESC pauses the entire game while the tutorial is active.
//!
//! Responsibilities:
//! - Detect pause input
//! - Maintain tutorial pause state
//! - Emit pause/resume requests
//! - Synchronize pause state
//!
//! This module intentionally contains no gameplay logic.

use crate::AppState;
use bevy::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Resources
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Stores whether the tutorial is currently paused.
#[derive(Resource, Default)]
pub struct TutorialPauseState {
    pub paused: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Events
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Requests the tutorial to pause.
#[derive(Event)]
pub struct PauseTutorialEvent;

/// Requests the tutorial to resume.
#[derive(Event)]
pub struct ResumeTutorialEvent;

/// Requests the game to pause.
///
/// This will later be consumed by the global game pause system.
#[derive(Event)]
pub struct PauseGameEvent;

/// Requests the game to resume.
///
/// This will later be consumed by the global game pause system.
#[derive(Event)]
pub struct ResumeGameEvent;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct TutorialPausePlugin;

impl Plugin for TutorialPausePlugin {
    fn build(&self, app: &mut App) {
        app
            ////////////////////////////////////////////////////////////////////
            // Resources
            ////////////////////////////////////////////////////////////////////
            .init_resource::<TutorialPauseState>()
            ////////////////////////////////////////////////////////////////////
            // Events
            ////////////////////////////////////////////////////////////////////
            .add_event::<PauseTutorialEvent>()
            .add_event::<ResumeTutorialEvent>()
            .add_event::<PauseGameEvent>()
            .add_event::<ResumeGameEvent>()
            ////////////////////////////////////////////////////////////////////
            // Input Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                Update,
                handle_pause_input.run_if(in_state(AppState::Tutorial)),
            )
            ////////////////////////////////////////////////////////////////////
            // Pause Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                Update,
                (request_pause, request_resume).run_if(in_state(AppState::Tutorial)),
            )
            ////////////////////////////////////////////////////////////////////
            // Update Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(Update, synchronize_pause_state);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Input Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Handles ESC presses.
pub fn handle_pause_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    pause_state: Res<TutorialPauseState>,
    mut pause_events: EventWriter<PauseTutorialEvent>,
    mut resume_events: EventWriter<ResumeTutorialEvent>,
) {
    if !keyboard.just_pressed(KeyCode::Escape) {
        return;
    }

    if pause_state.paused {
        resume_events.send(ResumeTutorialEvent);
    } else {
        pause_events.send(PauseTutorialEvent);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Pause Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Handles pause requests.
pub fn request_pause(
    mut events: EventReader<PauseTutorialEvent>,
    mut state: ResMut<TutorialPauseState>,
    mut game_events: EventWriter<PauseGameEvent>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    state.paused = true;

    game_events.send(PauseGameEvent);
}

/// Handles resume requests.
pub fn request_resume(
    mut events: EventReader<ResumeTutorialEvent>,
    mut state: ResMut<TutorialPauseState>,
    mut game_events: EventWriter<ResumeGameEvent>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    state.paused = false;

    game_events.send(ResumeGameEvent);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Update Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Synchronizes the pause state.
///
/// Other tutorial systems may read TutorialPauseState to disable input,
/// animations, dialogue progression, or timers while paused.
pub fn synchronize_pause_state(_pause_state: Res<TutorialPauseState>) {
    // Intentionally left empty.
    //
    // This system exists as a synchronization point for future expansion.
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Utility Functions
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Returns whether the tutorial is currently paused.
#[inline]
pub fn is_paused(state: &TutorialPauseState) -> bool {
    state.paused
}
