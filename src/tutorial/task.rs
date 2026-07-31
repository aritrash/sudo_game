//! Tutorial task coordinator.
//!
//! This module coordinates progression through the Orientation Facility.
//! It is the ONLY module responsible for advancing TutorialStage.
//!
//! Gameplay systems notify this module when the current objective has
//! been completed. This module then advances the tutorial and updates
//! the corresponding objective.
//!
//! This module intentionally contains no gameplay implementation.

use bevy::prelude::*;

use crate::AppState;
use crate::tutorial::progression::ProgressTutorialEvent;

/// Tracks whether the current tutorial task has been completed.
#[derive(Resource, Debug, Default)]
pub struct TutorialTaskState {
    completed: bool,
}

impl TutorialTaskState {
    /// Marks the current task as completed.
    #[inline]
    pub fn complete(&mut self) {
        self.completed = true;
    }

    /// Returns whether the current task is complete.
    #[inline]
    pub fn is_completed(&self) -> bool {
        self.completed
    }

    /// Resets the completion state.
    #[inline]
    pub fn reset(&mut self) {
        self.completed = false;
    }
}

/// Plugin that connects task completion to tutorial progression.
pub struct TutorialTaskPlugin;

impl Plugin for TutorialTaskPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TutorialTaskState>()
            .add_systems(Update, advance_tutorial.run_if(in_state(AppState::Tutorial)));
    }
}

/// Advances the tutorial after the active task has been completed.
///
/// This system coordinates task completion with the progression controller.
pub fn advance_tutorial(
    mut task: ResMut<TutorialTaskState>,
    mut progress: EventWriter<ProgressTutorialEvent>,
) {
    if !task.is_completed() {
        return;
    }

    task.reset();
    progress.send(ProgressTutorialEvent);
}