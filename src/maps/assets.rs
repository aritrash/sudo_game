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

////////////////////////////////////////////////////////////////////////////////////////////////////
// Furniture Constants
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const DESK_WIDTH: f32 = 1.60;
pub const DESK_DEPTH: f32 = 0.80;
pub const DESK_HEIGHT: f32 = 0.75;

pub const DESK_TOP_THICKNESS: f32 = 0.05;
pub const DESK_LEG_THICKNESS: f32 = 0.07;

pub const CHAIR_WIDTH: f32 = 0.50;
pub const CHAIR_DEPTH: f32 = 0.50;
pub const CHAIR_HEIGHT: f32 = 0.90;

pub const CHAIR_SEAT_HEIGHT: f32 = CHAIR_HEIGHT * 0.5;
pub const CHAIR_LEG_HEIGHT: f32 = CHAIR_SEAT_HEIGHT;

pub const CHAIR_SEAT_THICKNESS: f32 = 0.06;
pub const CHAIR_BACKREST_HEIGHT: f32 = 0.45;
pub const CHAIR_BACKREST_THICKNESS: f32 = 0.05;
pub const CHAIR_LEG_THICKNESS: f32 = 0.05;

pub const CABINET_WIDTH: f32 = 1.00;
pub const CABINET_DEPTH: f32 = 0.50;
pub const CABINET_HEIGHT: f32 = 1.80;

pub const CABINET_DOOR_THICKNESS: f32 = 0.02;
pub const CABINET_DOOR_WIDTH: f32 = CABINET_WIDTH * 0.92;
pub const CABINET_DOOR_HEIGHT: f32 = CABINET_HEIGHT * 0.46;
pub const CABINET_HANDLE_WIDTH: f32 = 0.12;
pub const CABINET_HANDLE_HEIGHT: f32 = 0.03;
pub const CABINET_HANDLE_DEPTH: f32 = 0.03;

pub const SERVER_RACK_WIDTH: f32 = 0.60;
pub const SERVER_RACK_DEPTH: f32 = 1.00;
pub const SERVER_RACK_HEIGHT: f32 = 2.00;

pub const SERVER_RACK_DOOR_THICKNESS: f32 = 0.02;
pub const SERVER_RACK_RAIL_COUNT: usize = 10;
pub const SERVER_RACK_RAIL_HEIGHT: f32 = 0.03;
pub const SERVER_RACK_RAIL_DEPTH: f32 = 0.02;
pub const SERVER_RACK_LED_SIZE: f32 = 0.025;
pub const SERVER_RACK_LED_COUNT: usize = 5;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spawn Cuboid
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Spawns a single PBR cuboid.
///
/// This helper is used internally by all procedural map assets.
#[inline]
fn spawn_cuboid(
    parent: &mut ChildBuilder,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    size: Vec3,
    position: Vec3,
    material: Handle<StandardMaterial>,
) {
    parent.spawn(PbrBundle {
        mesh: meshes.add(Cuboid::new(
            size.x,
            size.y,
            size.z,
        )),

        material,

        transform: Transform::from_translation(position),

        ..default()
    });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Create Material
////////////////////////////////////////////////////////////////////////////////////////////////////

/// Creates a reusable StandardMaterial.
///
/// Materials are intended to be shared by multiple meshes.
#[inline]
fn create_material(
    materials: &mut Assets<StandardMaterial>,
    color: Color,
    metallic: f32,
    roughness: f32,
) -> Handle<StandardMaterial> {
    materials.add(StandardMaterial {
        base_color: color,
        metallic,
        perceptual_roughness: roughness,
        ..default()
    })
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Desk
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn desk(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    id: FurnitureId,
    position: Vec3,
) {
    let tabletop_material = create_material(
        materials,
        Color::srgb(0.56, 0.43, 0.28),
        0.0,
        0.82,
    );

    let leg_material = create_material(
        materials,
        Color::srgb(0.18, 0.18, 0.18),
        0.35,
        0.65,
    );

    commands
        .spawn((
            FurnitureBundle {
                furniture: Furniture {
                    kind: FurnitureType::Desk,
                },

                morph_state: MorphState::default(),

                velocity: Velocity::default(),

                collider: Collider {
                    size: Vec2::new(
                        DESK_WIDTH,
                        DESK_DEPTH,
                    ),
                },

                transform: Transform::from_translation(position),

                global_transform: default(),

                visibility: default(),

                inherited_visibility: default(),

                view_visibility: default(),
            },

            id,
        ))
        .with_children(|parent| {

            ////////////////////////////////////////////////////////////////////
            // Table Top
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    DESK_WIDTH,
                    DESK_TOP_THICKNESS,
                    DESK_DEPTH,
                ),
                Vec3::new(
                    0.0,
                    DESK_HEIGHT - DESK_TOP_THICKNESS * 0.5,
                    0.0,
                ),
                tabletop_material.clone(),
            );

            ////////////////////////////////////////////////////////////////////
            // Legs
            ////////////////////////////////////////////////////////////////////

            let leg_height = DESK_HEIGHT - DESK_TOP_THICKNESS;

            let leg_size = Vec3::new(
                DESK_LEG_THICKNESS,
                leg_height,
                DESK_LEG_THICKNESS,
            );

            let x = DESK_WIDTH * 0.5 - DESK_LEG_THICKNESS;
            let z = DESK_DEPTH * 0.5 - DESK_LEG_THICKNESS;
            let y = leg_height * 0.5;

            let leg_positions = [

                Vec3::new( x, y,  z),

                Vec3::new(-x, y,  z),

                Vec3::new( x, y, -z),

                Vec3::new(-x, y, -z),
            ];

            for leg_position in leg_positions {

                spawn_cuboid(
                    parent,
                    meshes,
                    materials,
                    leg_size,
                    leg_position,
                    leg_material.clone(),
                );
            }
        });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Chair
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn chair(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    id: FurnitureId,
    position: Vec3,
) {
    let seat_material = create_material(
        materials,
        Color::srgb(0.18, 0.18, 0.20),
        0.0,
        0.72,
    );

    let frame_material = create_material(
        materials,
        Color::srgb(0.30, 0.30, 0.32),
        0.45,
        0.45,
    );

    commands
        .spawn((
            FurnitureBundle {

                furniture: Furniture {
                    kind: FurnitureType::Chair,
                },

                morph_state: MorphState::default(),

                velocity: Velocity::default(),

                collider: Collider {
                    size: Vec2::new(
                        CHAIR_WIDTH,
                        CHAIR_DEPTH,
                    ),
                },

                transform: Transform::from_translation(position),

                global_transform: default(),

                visibility: default(),

                inherited_visibility: default(),

                view_visibility: default(),
            },

            id,
        ))
        .with_children(|parent| {

            ////////////////////////////////////////////////////////////////////
            // Seat
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    CHAIR_WIDTH,
                    CHAIR_SEAT_THICKNESS,
                    CHAIR_DEPTH,
                ),
                Vec3::new(
                    0.0,
                    CHAIR_SEAT_HEIGHT,
                    0.0,
                ),
                seat_material.clone(),
            );

            ////////////////////////////////////////////////////////////////////
            // Backrest
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    CHAIR_WIDTH,
                    CHAIR_BACKREST_HEIGHT,
                    CHAIR_BACKREST_THICKNESS,
                ),
                Vec3::new(
                    0.0,
                    0.45 + CHAIR_BACKREST_HEIGHT * 0.5,
                    -(CHAIR_DEPTH * 0.5) + CHAIR_BACKREST_THICKNESS * 0.5,
                ),
                seat_material.clone(),
            );

            ////////////////////////////////////////////////////////////////////
            // Legs
            ////////////////////////////////////////////////////////////////////

            let leg_height = CHAIR_LEG_HEIGHT;

            let leg_size = Vec3::new(
                CHAIR_LEG_THICKNESS,
                leg_height,
                CHAIR_LEG_THICKNESS,
            );

            let x = CHAIR_WIDTH * 0.5 - CHAIR_LEG_THICKNESS;
            let z = CHAIR_DEPTH * 0.5 - CHAIR_LEG_THICKNESS;
            let y = leg_height * 0.5;

            let leg_positions = [

                Vec3::new( x, y,  z),

                Vec3::new(-x, y,  z),

                Vec3::new( x, y, -z),

                Vec3::new(-x, y, -z),
            ];

            for leg_position in leg_positions {

                spawn_cuboid(
                    parent,
                    meshes,
                    materials,
                    leg_size,
                    leg_position,
                    frame_material.clone(),
                );
            }
        });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Cabinet
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn cabinet(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    id: FurnitureId,
    position: Vec3,
) {
    let body_material = create_material(
        materials,
        Color::srgb(0.72, 0.74, 0.77),
        0.45,
        0.38,
    );

    let handle_material = create_material(
        materials,
        Color::srgb(0.18, 0.18, 0.18),
        0.70,
        0.20,
    );

    commands
        .spawn((
            FurnitureBundle {

                furniture: Furniture {
                    kind: FurnitureType::Cabinet,
                },

                morph_state: MorphState::default(),

                velocity: Velocity::default(),

                collider: Collider {
                    size: Vec2::new(
                        CABINET_WIDTH,
                        CABINET_DEPTH,
                    ),
                },

                transform: Transform::from_translation(position),

                global_transform: default(),

                visibility: default(),

                inherited_visibility: default(),

                view_visibility: default(),
            },

            id,
        ))
        .with_children(|parent| {

            ////////////////////////////////////////////////////////////////////
            // Cabinet Body
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    CABINET_WIDTH,
                    CABINET_HEIGHT,
                    CABINET_DEPTH,
                ),
                Vec3::new(
                    0.0,
                    CABINET_HEIGHT * 0.5,
                    0.0,
                ),
                body_material.clone(),
            );

            ////////////////////////////////////////////////////////////////////
            // Upper Door
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    CABINET_WIDTH * 0.92,
                    CABINET_HEIGHT * 0.46,
                    CABINET_DOOR_THICKNESS,
                ),
                Vec3::new(
                    0.0,
                    CABINET_HEIGHT * 0.73,
                    CABINET_DEPTH * 0.5 + CABINET_DOOR_THICKNESS * 0.5,
                ),
                body_material.clone(),
            );

            ////////////////////////////////////////////////////////////////////
            // Lower Door
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    CABINET_WIDTH * 0.92,
                    CABINET_HEIGHT * 0.46,
                    CABINET_DOOR_THICKNESS,
                ),
                Vec3::new(
                    0.0,
                    CABINET_HEIGHT * 0.25,
                    CABINET_DEPTH * 0.5 + CABINET_DOOR_THICKNESS * 0.5,
                ),
                body_material.clone(),
            );

            ////////////////////////////////////////////////////////////////////
            // Upper Handle
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    0.12,
                    0.03,
                    0.03,
                ),
                Vec3::new(
                    0.0,
                    CABINET_HEIGHT * 0.73,
                    CABINET_DEPTH * 0.5 + CABINET_DOOR_THICKNESS * 0.5 + 0.03,
                ),
                handle_material.clone(),
            );

            ////////////////////////////////////////////////////////////////////
            // Lower Handle
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    0.12,
                    0.03,
                    0.03,
                ),
                Vec3::new(
                    0.0,
                    CABINET_HEIGHT * 0.25,
                    CABINET_DEPTH * 0.5 + 0.03,
                ),
                handle_material,
            );
        });
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Server Rack
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn server_rack(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    id: FurnitureId,
    position: Vec3,
) {
    let body_material = create_material(
        materials,
        Color::srgb(0.08, 0.08, 0.09),
        0.55,
        0.42,
    );

    let rail_material = create_material(
        materials,
        Color::srgb(0.22, 0.24, 0.26),
        0.65,
        0.28,
    );

    let led_material = create_material(
        materials,
        Color::srgb(0.10, 0.80, 0.18),
        0.0,
        0.15,
    );

    commands
        .spawn((
            FurnitureBundle {

                furniture: Furniture {
                    kind: FurnitureType::ServerRack,
                },

                morph_state: MorphState::default(),

                velocity: Velocity::default(),

                collider: Collider {
                    size: Vec2::new(
                        SERVER_RACK_WIDTH,
                        SERVER_RACK_DEPTH,
                    ),
                },

                transform: Transform::from_translation(position),

                global_transform: default(),

                visibility: default(),

                inherited_visibility: default(),

                view_visibility: default(),
            },

            id,
        ))
        .with_children(|parent| {

            ////////////////////////////////////////////////////////////////////
            // Rack Body
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    SERVER_RACK_WIDTH,
                    SERVER_RACK_HEIGHT,
                    SERVER_RACK_DEPTH,
                ),
                Vec3::new(
                    0.0,
                    SERVER_RACK_HEIGHT * 0.5,
                    0.0,
                ),
                body_material.clone(),
            );

            ////////////////////////////////////////////////////////////////////
            // Front Door
            ////////////////////////////////////////////////////////////////////

            spawn_cuboid(
                parent,
                meshes,
                materials,
                Vec3::new(
                    SERVER_RACK_WIDTH * 0.94,
                    SERVER_RACK_HEIGHT * 0.96,
                    SERVER_RACK_DOOR_THICKNESS,
                ),
                Vec3::new(
                    0.0,
                    SERVER_RACK_HEIGHT * 0.5,
                    SERVER_RACK_DEPTH * 0.5 + SERVER_RACK_DOOR_THICKNESS * 0.5,
                ),
                rail_material.clone(),
            );

            ////////////////////////////////////////////////////////////////////
            // Rack Rails
            ////////////////////////////////////////////////////////////////////

            for i in 0..SERVER_RACK_RAIL_COUNT {

                let y = 0.20 + i as f32 * 0.17;

                spawn_cuboid(
                    parent,
                    meshes,
                    materials,
                    Vec3::new(
                        SERVER_RACK_WIDTH * 0.86,
                        SERVER_RACK_RAIL_HEIGHT,
                        SERVER_RACK_RAIL_DEPTH,
                    ),
                    Vec3::new(
                        0.0,
                        y,
                        SERVER_RACK_DEPTH * 0.5 + SERVER_RACK_DOOR_THICKNESS * 0.5 + 0.03,
                    ),
                    rail_material.clone(),
                );
            }

            ////////////////////////////////////////////////////////////////////
            // Status LEDs
            ////////////////////////////////////////////////////////////////////

            for i in 0..SERVER_RACK_LED_COUNT {

                let y = SERVER_RACK_HEIGHT - 0.25 - i as f32 * 0.07;

                spawn_cuboid(
                    parent,
                    meshes,
                    materials,
                    Vec3::new(
                        SERVER_RACK_LED_SIZE,
                        SERVER_RACK_LED_SIZE,
                        SERVER_RACK_LED_SIZE,
                    ),
                    Vec3::new(
                        SERVER_RACK_WIDTH * 0.32,
                        y,
                        SERVER_RACK_DEPTH * 0.5 + SERVER_RACK_DOOR_THICKNESS * 0.5 + 0.04,
                    ),
                    led_material.clone(),
                );
            }
        });
}