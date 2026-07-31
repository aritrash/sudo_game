//! ================================================================================================
//! S.U.D.O.
//! Maps Asset Library
//!
//! This module defines the reusable building blocks used to construct maps.
//!
//! Assets are divided into four categories:
//!
//!     • Architecture
//!     • Infrastructure
//!     • Furniture
//!     • Interactables
//!
//! Every map should be built exclusively using the functions and bundles
//! provided here.
//! ================================================================================================

use bevy::prelude::*;

use crate::gameplay::physics::{
    Collider,
    Velocity,
};

////////////////////////////////////////////////////////////////////////////////////////////////////
// Marker Components
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Marks an architectural object.
///
/// Architecture never moves during gameplay.
#[derive(Component)]
pub struct StaticEnvironment;

/// Marks infrastructure objects.
///
/// Examples:
///     • Ceiling lights
///     • Security cameras
///     • Air-conditioning vents
#[derive(Component)]
pub struct Infrastructure;

/// Marks an interactable object.
#[derive(Component)]
pub struct Interactable {
    pub kind: InteractableType,
}

/// Marks a furniture object.
///
/// Furniture may be moved by players and may be possessed
/// by a Rootkit.
#[derive(Component)]
pub struct Furniture {
    pub kind: FurnitureType,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Furniture
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum FurnitureType {
    Desk,
    Chair,
    Cabinet,
    ServerRack,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Interactables
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InteractableType {
    Terminal,
    Keypad,
    UplinkTerminal,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Infrastructure
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfrastructureType {
    CeilingLight,
    Vent,
    SecurityCamera,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Morph State
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Current Rootkit occupation state.
#[derive(Component)]
pub enum MorphState {

    /// Furniture is free.
    Empty,

    /// Occupied by a Rootkit.
    Occupied(Entity),

    /// Recently vacated.
    Cooldown(Timer),
}

impl Default for MorphState {
    fn default() -> Self {
        Self::Empty
    }
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Furniture Identity
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Unique furniture identifier.
///
/// Used by future Rootkit and anomaly systems.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct FurnitureId(pub u32);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Bundles
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Bundle)]
pub struct FurnitureBundle {
    pub furniture: Furniture,
    pub morph_state: MorphState,
    pub velocity: Velocity,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

#[derive(Bundle)]
pub struct StaticBundle {
    pub collider: Collider,
    pub environment: StaticEnvironment,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Architecture Constants
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const WALL_HEIGHT: f32 = 3.0;
pub const WALL_THICKNESS: f32 = 0.25;

pub const FLOOR_THICKNESS: f32 = 0.10;
pub const CEILING_THICKNESS: f32 = 0.10;

pub const DOOR_WIDTH: f32 = 1.25;
pub const DOOR_HEIGHT: f32 = 2.25;
pub const DOOR_THICKNESS: f32 = WALL_THICKNESS;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Floor
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn floor(
    commands: &mut Commands,
    position: Vec3,
    size: Vec2,
) {
    commands.spawn(StaticBundle {

        collider: Collider {
            size,
        },
        environment: StaticEnvironment,
        transform: Transform::from_translation(position),
        global_transform: default(),
        visibility: default(),
        inherited_visibility: default(),
        view_visibility: default(),
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ceiling
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn ceiling(
    commands: &mut Commands,
    position: Vec3,
    size: Vec2,
) {
    commands.spawn(StaticBundle {

        collider: Collider {
            size,
        },
        environment: StaticEnvironment,
        transform: Transform::from_translation(position),
        global_transform: default(),
        visibility: default(),
        inherited_visibility: default(),
        view_visibility: default(),
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Wall
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn wall(
    commands: &mut Commands,
    position: Vec3,
    size: Vec2,
) {
    commands.spawn(StaticBundle {

        collider: Collider {
            size,
        },

        environment: StaticEnvironment,
        transform: Transform::from_translation(position),
        global_transform: default(),
        visibility: default(),
        inherited_visibility: default(),
        view_visibility: default(),
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Door
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn door(
    commands: &mut Commands,
    position: Vec3,
    size: Vec2,
) {
    commands.spawn(StaticBundle {

        collider: Collider {
            size,
        },

        environment: StaticEnvironment,
        transform: Transform::from_translation(position),
        global_transform: default(),
        visibility: default(),
        inherited_visibility: default(),
        view_visibility: default(),
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Infrastructure
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum InfrastructureType {
    CeilingLight,
    Vent,
    SecurityCamera,
}

/// Marks a piece of infrastructure.
///
/// Infrastructure is static and cannot be interacted with or
/// possessed by a Rootkit.
#[derive(Component)]
pub struct Infrastructure {
    pub kind: InfrastructureType,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Infrastructure Constants
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const CEILING_LIGHT_HEIGHT: f32 = WALL_HEIGHT - 0.15;
pub const VENT_HEIGHT: f32 = WALL_HEIGHT - 0.05;
pub const CAMERA_HEIGHT: f32 = WALL_HEIGHT - 0.35;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Ceiling Light
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn ceiling_light(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        Infrastructure {
            kind: InfrastructureType::CeilingLight,
        },

        PbrBundle {
            mesh: meshes.add(Cuboid::new(0.40, 0.08, 0.40)),
            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.95, 0.95, 0.98),
                metallic: 0.0,
                perceptual_roughness: 0.85,
                ..default()
            }),
            transform: Transform::from_translation(position),
            ..default()
        },

        PointLightBundle {
            point_light: PointLight {
                intensity: 2200.0,
                range: 14.0,
                radius: 0.15,

                color: Color::srgb(1.0, 0.98, 0.93),

                shadows_enabled: true,

                ..default()
            },

            transform: Transform::from_translation(
                position - Vec3::new(0.0, 0.08, 0.0),
            ),

            ..default()
        },
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Air Conditioning Vent
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn vent(
    commands: &mut Commands,
    position: Vec3,
) {
    commands.spawn((
        Infrastructure {
            kind: InfrastructureType::Vent,
        },

        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Security Camera
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn security_camera(
    commands: &mut Commands,
    position: Vec3,
) {
    commands.spawn((
        Infrastructure {
            kind: InfrastructureType::SecurityCamera,
        },

        Transform::from_translation(position),
        GlobalTransform::default(),
        Visibility::default(),
        InheritedVisibility::default(),
        ViewVisibility::default(),
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Interactable Bundle
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Bundle)]
pub struct InteractableBundle {
    pub interactable: Interactable,
    pub collider: Collider,
    pub transform: Transform,
    pub global_transform: GlobalTransform,
    pub visibility: Visibility,
    pub inherited_visibility: InheritedVisibility,
    pub view_visibility: ViewVisibility,
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Interactable Constants
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const TERMINAL_WIDTH: f32 = 0.80;
pub const TERMINAL_DEPTH: f32 = 0.60;
pub const TERMINAL_HEIGHT: f32 = 1.40;

pub const KEYPAD_WIDTH: f32 = 0.30;
pub const KEYPAD_HEIGHT: f32 = 0.40;

pub const UPLINK_WIDTH: f32 = 1.20;
pub const UPLINK_DEPTH: f32 = 0.80;
pub const UPLINK_HEIGHT: f32 = 1.60;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Terminal
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn terminal(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        InteractableBundle {

            interactable: Interactable {
                kind: InteractableType::Terminal,
            },

            collider: Collider {
                size: Vec2::new(
                    TERMINAL_WIDTH,
                    TERMINAL_DEPTH,
                ),
            },

            transform: Transform::from_translation(position),

            global_transform: default(),

            visibility: default(),

            inherited_visibility: default(),

            view_visibility: default(),
        },

        PbrBundle {
            mesh: meshes.add(Cuboid::new(
                TERMINAL_WIDTH,
                TERMINAL_HEIGHT,
                TERMINAL_DEPTH,
            )),

            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.18, 0.18, 0.20),
                metallic: 0.30,
                perceptual_roughness: 0.60,
                ..default()
            }),

            transform: Transform::from_translation(position),

            ..default()
        },
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Keypad
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn keypad(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        InteractableBundle {

            interactable: Interactable {
                kind: InteractableType::Keypad,
            },

            collider: Collider {
                size: Vec2::new(
                    KEYPAD_WIDTH,
                    0.10,
                ),
            },

            transform: Transform::from_translation(position),

            global_transform: default(),

            visibility: default(),

            inherited_visibility: default(),

            view_visibility: default(),
        },

        PbrBundle {

            mesh: meshes.add(Cuboid::new(
                KEYPAD_WIDTH,
                KEYPAD_HEIGHT,
                0.08,
            )),

            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.12, 0.12, 0.13),
                metallic: 0.40,
                perceptual_roughness: 0.50,
                ..default()
            }),

            transform: Transform::from_translation(position),

            ..default()
        },
    ));
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Uplink Terminal
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn uplink_terminal(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    position: Vec3,
) {
    commands.spawn((
        InteractableBundle {

            interactable: Interactable {
                kind: InteractableType::UplinkTerminal,
            },

            collider: Collider {
                size: Vec2::new(
                    UPLINK_WIDTH,
                    UPLINK_DEPTH,
                ),
            },

            transform: Transform::from_translation(position),

            global_transform: default(),

            visibility: default(),

            inherited_visibility: default(),

            view_visibility: default(),
        },

        PbrBundle {

            mesh: meshes.add(Cuboid::new(
                UPLINK_WIDTH,
                UPLINK_HEIGHT,
                UPLINK_DEPTH,
            )),

            material: materials.add(StandardMaterial {
                base_color: Color::srgb(0.22, 0.23, 0.25),
                metallic: 0.45,
                perceptual_roughness: 0.55,
                ..default()
            }),

            transform: Transform::from_translation(position),

            ..default()
        },
    ));
}