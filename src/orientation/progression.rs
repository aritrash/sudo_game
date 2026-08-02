//! ================================================================================================
//! Orientation Progression
//!
//! Owns the progression state of the Orientation Facility.
//!
//! This module is the ONLY authority permitted to advance or reset the
//! OrientationStage.
//!
//! It intentionally contains no UI, dialogue, gameplay or objective
//! logic.
//! ================================================================================================

use bevy::prelude::*;

use crate::orientation::stages::OrientationStage;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Events
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Advance to the next Orientation stage.
#[derive(Event)]
pub struct AdvanceOrientationEvent;

/// Reset the Orientation Facility.
#[derive(Event)]
pub struct ResetOrientationEvent;

/// Emitted whenever the Orientation stage changes.
#[derive(Event)]
pub struct OrientationStageChangedEvent {
    pub stage: OrientationStage,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct OrientationProgressionPlugin;

impl Plugin for OrientationProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            ////////////////////////////////////////////////////////////////
            // Events
            ////////////////////////////////////////////////////////////////

            .add_event::<AdvanceOrientationEvent>()
            .add_event::<ResetOrientationEvent>()
            .add_event::<OrientationStageChangedEvent>()

            ////////////////////////////////////////////////////////////////
            // Systems
            ////////////////////////////////////////////////////////////////

            .add_systems(
                Update,
                (
                    advance_orientation,
                    reset_orientation,
                ),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Advance
////////////////////////////////////////////////////////////////////////////////////////////////////

fn advance_orientation(
    mut advance_events: EventReader<AdvanceOrientationEvent>,
    mut stage: ResMut<OrientationStage>,
    mut changed_events: EventWriter<OrientationStageChangedEvent>,
) {
    if advance_events.is_empty() {
        return;
    }

    advance_events.clear();

    if *stage == OrientationStage::Finished {
        return;
    }

    *stage = stage.next();

    changed_events.send(OrientationStageChangedEvent {
        stage: *stage,
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Reset
////////////////////////////////////////////////////////////////////////////////////////////////////

fn reset_orientation(
    mut reset_events: EventReader<ResetOrientationEvent>,
    mut stage: ResMut<OrientationStage>,
    mut changed_events: EventWriter<OrientationStageChangedEvent>,
) {
    if reset_events.is_empty() {
        return;
    }

    reset_events.clear();

    *stage = OrientationStage::Boot;

    changed_events.send(OrientationStageChangedEvent {
        stage: *stage,
    });
}