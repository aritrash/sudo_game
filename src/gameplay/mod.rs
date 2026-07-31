pub mod inputs;
pub mod physics;

use bevy::prelude::*;
use crate::gameplay::inputs::PlayerInput;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerInput>()
            .add_systems(Update, (
                inputs::keyboard_input_system,
                inputs::mouse_input_system,
                physics::movement_system,
                physics::collision_system,
            ));
    }
}