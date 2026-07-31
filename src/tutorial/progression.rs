//! Tutorial progression controller.
//!
//! This module is the single authority responsible for advancing the
//! Orientation Facility. Neither the UI nor gameplay systems should
//! modify TutorialStage directly.
//!
//! Instead, gameplay systems request progression by emitting
//! ProgressTutorialEvent. This controller then:
//!
//! • Advances the tutorial stage
//! • Updates the active objective
//! • Requests the dialogue UI to display the new message
//! • Refreshes the objective panel
//! • Emits tutorial completion events when appropriate

use bevy::prelude::*;

use crate::tutorial::objectives::TutorialObjective;
use crate::tutorial::stages::TutorialStage;
use crate::tutorial::ui::{
    ShowDialogueEvent,
    UpdateObjectiveEvent,
    TutorialCompletedEvent,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Events
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Requests the tutorial to advance by one stage.
#[derive(Event)]
pub struct ProgressTutorialEvent;

/// Resets the tutorial back to the Boot stage.
#[derive(Event)]
pub struct ResetTutorialEvent;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct TutorialProgressionPlugin;

impl Plugin for TutorialProgressionPlugin {
    fn build(&self, app: &mut App) {
        app

            ////////////////////////////////////////////////////////////////////
            // Resources
            ////////////////////////////////////////////////////////////////////

            .init_resource::<TutorialStage>()
            .init_resource::<TutorialObjective>()

            ////////////////////////////////////////////////////////////////////
            // Events
            ////////////////////////////////////////////////////////////////////

            .add_event::<ProgressTutorialEvent>()
            .add_event::<ResetTutorialEvent>()
            .add_event::<TutorialCompletedEvent>()

            ////////////////////////////////////////////////////////////////////
            // Systems
            ////////////////////////////////////////////////////////////////////

            .add_systems(
                Update,
                (
                    progress_tutorial,
                    reset_tutorial,
                ),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Progression Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Advances the tutorial to the next stage.
///
/// This is the ONLY place in the project where TutorialStage::next()
/// should ever be called.
pub fn progress_tutorial(
    mut progress_events: EventReader<ProgressTutorialEvent>,

    mut stage: ResMut<TutorialStage>,
    mut objective: ResMut<TutorialObjective>,

    mut dialogue_events: EventWriter<ShowDialogueEvent>,
    mut objective_events: EventWriter<UpdateObjectiveEvent>,
    mut completion_events: EventWriter<TutorialCompletedEvent>,
) {
    if progress_events.is_empty() {
        return;
    }

    progress_events.clear();

    // Prevent advancing past the final stage.
    if *stage == TutorialStage::Finished {
        return;
    }

    *stage = stage.next();

    *objective = TutorialObjective::from_stage(*stage);

    objective_events.send(UpdateObjectiveEvent);

    dialogue_events.send(ShowDialogueEvent);

    if *stage == TutorialStage::Finished {
        completion_events.send(TutorialCompletedEvent);
    }
}

/// Resets the Orientation Facility.
pub fn reset_tutorial(
    mut reset_events: EventReader<ResetTutorialEvent>,

    mut stage: ResMut<TutorialStage>,
    mut objective: ResMut<TutorialObjective>,

    mut dialogue_events: EventWriter<ShowDialogueEvent>,
    mut objective_events: EventWriter<UpdateObjectiveEvent>,
) {
    if reset_events.is_empty() {
        return;
    }

    reset_events.clear();

    *stage = TutorialStage::Boot;

    *objective = TutorialObjective::from_stage(TutorialStage::Boot);

    objective_events.send(UpdateObjectiveEvent);

    dialogue_events.send(ShowDialogueEvent);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Utility Functions
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Returns the current tutorial stage.
#[inline]
pub fn current_stage(stage: Res<TutorialStage>) -> TutorialStage {
    *stage
}

/// Returns whether the tutorial has finished.
#[inline]
pub fn tutorial_finished(stage: Res<TutorialStage>) -> bool {
    *stage == TutorialStage::Finished
}