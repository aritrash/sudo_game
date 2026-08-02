pub mod inputs;
pub mod physics;
pub mod player;
pub mod controller;

use bevy::prelude::*;

use inputs::PlayerInput;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app

            ////////////////////////////////////////////////////////////////
            // Resources
            ////////////////////////////////////////////////////////////////

            .init_resource::<PlayerInput>()

            ////////////////////////////////////////////////////////////////
            // Gameplay Systems
            ////////////////////////////////////////////////////////////////

            .add_systems(
                Update,
                (
                    inputs::keyboard_input_system,
                    inputs::mouse_input_system,
                    controller::player_controller_system,
                    physics::movement_system,
                    physics::collision_system,
                ),
            );
    }
}