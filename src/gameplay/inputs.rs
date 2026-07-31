//! ============================================================
//! S.U.D.O.
//! Gameplay Input
//!
//! Handles keyboard and mouse input.
//!
//! This module translates hardware input into gameplay state.
//! It does not move entities or perform gameplay logic.
//! ============================================================

use bevy::prelude::*;

#[derive(Resource, Default, Debug)]
pub struct PlayerInput {
    /// Movement direction.
    pub movement: Vec2,

    /// Interaction key.
    pub interact: bool,

    /// Sprint modifier.
    pub sprint: bool,

    /// Left mouse button.
    pub left_click: bool,

    /// Right mouse button.
    pub right_click: bool,

    /// Pause/menu request.
    pub pause: bool,
}

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

pub fn mouse_input_system(
    mouse: Res<ButtonInput<MouseButton>>,
    mut input: ResMut<PlayerInput>,
) {
    input.left_click = mouse.just_pressed(MouseButton::Left);

    input.right_click = mouse.just_pressed(MouseButton::Right);
}

