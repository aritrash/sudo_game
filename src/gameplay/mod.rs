pub mod camera;
pub mod input;
pub mod movement;
pub mod player;

use bevy::prelude::*;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins((
                input::GameplayInputPlugin,
                movement::GameplayMovementPlugin,
                camera::GameplayCameraPlugin,
            ));
    }
}