//! ================================================================================================
//! S.U.D.O.
//! Gameplay Camera
//!
//! Controls the local player's first-person camera.
//!
//! Responsibilities
//! ----------------
//! • Rotate the local player horizontally.
//! • Rotate the camera vertically.
//! • Clamp camera pitch.
//!
//! This module intentionally contains no movement,
//! collision, interaction or networking logic.
//! ================================================================================================

use bevy::prelude::*;

use crate::gameplay::input::PlayerInput;
use crate::gameplay::player::{
    LocalPlayer,
    PlayerCamera,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Constants
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Mouse sensitivity.
pub const MOUSE_SENSITIVITY: f32 = 0.0025;

/// Maximum upward look angle.
pub const MAX_PITCH: f32 = 89.0_f32.to_radians();

/// Maximum downward look angle.
pub const MIN_PITCH: f32 = -89.0_f32.to_radians();

////////////////////////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct CameraController {
    pub yaw: f32,
    pub pitch: f32,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            yaw: 0.0,
            pitch: 0.0,
        }
    }
}

#[derive(Component)]
pub struct CameraTarget(pub Entity);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Plugin
////////////////////////////////////////////////////////////////////////////////////////////////////

pub struct GameplayCameraPlugin;

impl Plugin for GameplayCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, camera_look_system);
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Systems
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn camera_look_system(
    input: Res<PlayerInput>,

    mut players: Query<
        (
            Entity,
            &mut CameraController,
            &mut Transform,
        ),
        With<LocalPlayer>,
    >,

    mut cameras: Query<
        (
            &CameraTarget,
            &mut Transform,
        ),
        With<PlayerCamera>,
    >,
) {
    if input.look_delta == Vec2::ZERO {
        return;
    }

    for (player_entity, mut controller, mut player_transform) in &mut players {

        controller.yaw -= input.look_delta.x * MOUSE_SENSITIVITY;

        controller.pitch -= input.look_delta.y * MOUSE_SENSITIVITY;

        controller.pitch = controller.pitch.clamp(
            MIN_PITCH,
            MAX_PITCH,
        );

        player_transform.rotation =
            Quat::from_rotation_y(controller.yaw);

        for (target, mut camera_transform) in &mut cameras {

            if target.0 != player_entity {
                continue;
            }

            camera_transform.translation =
                player_transform.translation
                + Vec3::Y * crate::gameplay::player::PLAYER_EYE_HEIGHT;

            camera_transform.rotation =
                Quat::from_rotation_y(controller.yaw)
                * Quat::from_rotation_x(controller.pitch);
        }
    }
}