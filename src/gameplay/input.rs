//! ================================================================================================
//! S.U.D.O.
//! Gameplay Input
//!
//! Collects keyboard and mouse input from the local client.
//!
//! Responsibilities
//! ----------------
//! • Read keyboard state.
//! • Read mouse buttons.
//! • Read mouse movement.
//! • Expose input for gameplay systems.
//!
//! This module intentionally contains no gameplay logic.
//! ================================================================================================

use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Resource
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Resource, Debug, Default)]
pub struct PlayerInput {
    /// Desired movement direction.
    pub movement: Vec2,

    /// Mouse movement since the previous frame.
    pub look_delta: Vec2,

    /// Interaction key.
    pub interact: bool,

    /// Sprint modifier.
    pub sprint: bool,

    /// Pause/menu request.
    pub pause: bool,

    /// Left mouse button.
    pub left_click: bool,

    /// Right mouse button.
    pub right_click: bool,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct GameplayInputPlugin;

impl Plugin for GameplayInputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<PlayerInput>()
            .add_systems(
                Update,
                (
                    keyboard_input_system,
                    mouse_button_input_system,
                    mouse_motion_input_system,
                ),
            );
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Keyboard
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn keyboard_input_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut input: ResMut<PlayerInput>,
) {
    let mut direction = Vec2::ZERO;

    if keyboard.pressed(KeyCode::KeyW) {
        direction.y += 1.0;
    }

    if keyboard.pressed(KeyCode::KeyS) {
        direction.y -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyA) {
        direction.x -= 1.0;
    }

    if keyboard.pressed(KeyCode::KeyD) {
        direction.x += 1.0;
    }

    input.movement = direction.normalize_or_zero();

    input.interact = keyboard.just_pressed(KeyCode::KeyE);

    input.sprint = keyboard.pressed(KeyCode::ShiftLeft);

    input.pause = keyboard.just_pressed(KeyCode::Escape);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mouse Buttons
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn mouse_button_input_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut input: ResMut<PlayerInput>,
) {
    input.left_click = mouse.just_pressed(MouseButton::Left);

    input.right_click = mouse.just_pressed(MouseButton::Right);
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Mouse Motion
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn mouse_motion_input_system(
    mut events: EventReader<MouseMotion>,
    mut input: ResMut<PlayerInput>,
) {
    input.look_delta = Vec2::ZERO;

    for event in events.read() {
        input.look_delta += event.delta;
    }
}