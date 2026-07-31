use bevy::prelude::*;

use crate::AppState;

pub mod arena;
pub mod cleanup;
pub mod objectives;
pub mod pause;
pub mod player;
pub mod pong;
pub mod progression;
pub mod stages;
pub mod task;
pub mod ui;

pub use cleanup::TutorialCleanupPlugin;
pub use pause::TutorialPausePlugin;
pub use progression::TutorialProgressionPlugin;
pub use task::TutorialTaskPlugin;
pub use ui::TutorialUiPlugin;

pub struct TutorialPlugin;

impl Plugin for TutorialPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<player::TutorialPlayer>()
            .init_resource::<task::TutorialTaskState>()
            .add_plugins((
                TutorialUiPlugin,
                TutorialProgressionPlugin,
                TutorialTaskPlugin,
                TutorialPausePlugin,
                TutorialCleanupPlugin,
            ))
            .add_systems(OnEnter(AppState::Tutorial), initialize_tutorial_state);
    }
}

fn initialize_tutorial_state(
    mut stage: ResMut<stages::TutorialStage>,
    mut objective: ResMut<objectives::TutorialObjective>,
    mut player: ResMut<player::TutorialPlayer>,
    mut task: ResMut<task::TutorialTaskState>,
    mut pause: ResMut<pause::TutorialPauseState>,
) {
    *stage = stages::TutorialStage::Boot;
    *objective = objectives::TutorialObjective::from_stage(stages::TutorialStage::Boot);
    *player = player::TutorialPlayer::default();
    task.reset();
    *pause = pause::TutorialPauseState::default();
}
