//! Tutorial Cleanup.
//!
//! Cleans up all entities and state associated with the Orientation Facility.
//!
//! Responsibilities:
//! - Remove tutorial UI
//! - Stop tutorial voice playback
//! - Reset tutorial resources
//! - Notify the rest of the game that cleanup has completed

use bevy::prelude::*;

use crate::tutorial::pause::TutorialPauseState;
use crate::tutorial::ui::{
    ContinuePrompt, DialogueState, ObjectiveState, TutorialCompletionScreen,
    TutorialContinuePrompt, TutorialDialogue, TutorialObjectivePanel, TutorialUi,
    TutorialVoiceAudio, UiAnimationState,
};
use crate::AppState;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Events
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Requests tutorial cleanup.
#[derive(Event)]
pub struct TutorialCleanupEvent;

/// Sent after tutorial cleanup has completed.
#[derive(Event)]
pub struct TutorialCleanupFinishedEvent;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct TutorialCleanupPlugin;

impl Plugin for TutorialCleanupPlugin {
    fn build(&self, app: &mut App) {
        app
            ////////////////////////////////////////////////////////////////////
            // Events
            ////////////////////////////////////////////////////////////////////
            .add_event::<TutorialCleanupEvent>()
            .add_event::<TutorialCleanupFinishedEvent>()
            ////////////////////////////////////////////////////////////////////
            // Cleanup Systems
            ////////////////////////////////////////////////////////////////////
            .add_systems(
                OnExit(AppState::Tutorial),
                (cleanup_ui, cleanup_audio, reset_resources, finish_cleanup),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cleanup Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Removes all tutorial UI entities.
pub fn cleanup_ui(
    mut commands: Commands,
    ui: Query<Entity, With<TutorialUi>>,
    dialogue: Query<Entity, With<TutorialDialogue>>,
    objectives: Query<Entity, With<TutorialObjectivePanel>>,
    prompt: Query<Entity, With<TutorialContinuePrompt>>,
    completion: Query<Entity, With<TutorialCompletionScreen>>,
) {
    for entity in &ui {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &dialogue {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &objectives {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &prompt {
        commands.entity(entity).despawn_recursive();
    }

    for entity in &completion {
        commands.entity(entity).despawn_recursive();
    }
}

/// Stops all tutorial voice playback.
pub fn cleanup_audio(mut commands: Commands, voices: Query<Entity, With<TutorialVoiceAudio>>) {
    for entity in &voices {
        commands.entity(entity).despawn_recursive();
    }
}

/// Resets tutorial resources to their default state.
pub fn reset_resources(
    mut dialogue: ResMut<DialogueState>,
    mut objective: ResMut<ObjectiveState>,
    mut prompt: ResMut<ContinuePrompt>,
    mut animation: ResMut<UiAnimationState>,
    mut pause: ResMut<TutorialPauseState>,
) {
    *dialogue = DialogueState::default();
    *objective = ObjectiveState::default();
    *prompt = ContinuePrompt::default();
    *animation = UiAnimationState::default();
    *pause = TutorialPauseState::default();
}

/// Signals that cleanup has completed.
pub fn finish_cleanup(
    mut events: EventReader<TutorialCleanupEvent>,
    mut finished: EventWriter<TutorialCleanupFinishedEvent>,
) {
    if events.is_empty() {
        return;
    }

    events.clear();

    finished.send(TutorialCleanupFinishedEvent);
}
