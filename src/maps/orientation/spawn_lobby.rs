//! ================================================================================================
//! Spawn Lobby
//!
//! The Spawn Lobby is the player's first room inside the S.U.D.O.
//! Orientation Facility.
//!
//! This room serves only to establish the environment and introduce
//! the player to the Orientation Facility before movement and gameplay
//! mechanics are taught.
//!
//! Responsibilities
//! ----------------
//! • Construct the room geometry.
//! • Place decorative infrastructure.
//! • Place furniture.
//! • Place interactable objects.
//! • Display the S.U.D.O. floor logo.
//!
//! This module intentionally contains no gameplay, tutorial,
//! progression or dialogue logic.
//! ================================================================================================

use bevy::prelude::*;

use crate::maps::assets::*;

pub const PLAYER_SPAWN_POSITION: Vec3 =
    Vec3::new(
        0.0,
        0.0,
        -2.5,
    );

////////////////////////////////////////////////////////////////////////////////////////////////////
// Room Dimensions
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ROOM_WIDTH: f32 = 14.0;
pub const ROOM_DEPTH: f32 = 10.0;
pub const ROOM_HEIGHT: f32 = WALL_HEIGHT;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Furniture IDs
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const DESK_ID: FurnitureId = FurnitureId(1);

pub const CHAIR_ID: FurnitureId = FurnitureId(2);

pub const CABINET_ID: FurnitureId = FurnitureId(3);

pub const SERVER_RACK_ID: FurnitureId = FurnitureId(4);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Architecture
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const FLOOR_POSITION: Vec3 =
    Vec3::new(0.0, 0.0, 0.0);

pub const CEILING_POSITION: Vec3 =
    Vec3::new(0.0, 0.0, ROOM_HEIGHT);

pub const NORTH_WALL_POSITION: Vec3 =
    Vec3::new(0.0, ROOM_DEPTH * 0.5, ROOM_HEIGHT * 0.5);

pub const SOUTH_WALL_POSITION: Vec3 =
    Vec3::new(0.0, -ROOM_DEPTH * 0.5, ROOM_HEIGHT * 0.5);

pub const EAST_WALL_POSITION: Vec3 =
    Vec3::new(ROOM_WIDTH * 0.5, 0.0, ROOM_HEIGHT * 0.5);

pub const WEST_WALL_POSITION: Vec3 =
    Vec3::new(-ROOM_WIDTH * 0.5, 0.0, ROOM_HEIGHT * 0.5);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Doors
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const ENTRANCE_DOOR_POSITION: Vec3 =
    Vec3::new(-ROOM_WIDTH * 0.5, 0.0, DOOR_HEIGHT * 0.5);

pub const EXIT_DOOR_POSITION: Vec3 =
    Vec3::new(ROOM_WIDTH * 0.5, 0.0, DOOR_HEIGHT * 0.5);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Lighting
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const LIGHT_POSITIONS: [Vec3; 9] = [
    Vec3::new(-4.5,  3.0, CEILING_LIGHT_HEIGHT),
    Vec3::new( 0.0,  3.0, CEILING_LIGHT_HEIGHT),
    Vec3::new( 4.5,  3.0, CEILING_LIGHT_HEIGHT),

    Vec3::new(-4.5,  0.0, CEILING_LIGHT_HEIGHT),
    Vec3::new( 0.0,  0.0, CEILING_LIGHT_HEIGHT),
    Vec3::new( 4.5,  0.0, CEILING_LIGHT_HEIGHT),

    Vec3::new(-4.5, -3.0, CEILING_LIGHT_HEIGHT),
    Vec3::new( 0.0, -3.0, CEILING_LIGHT_HEIGHT),
    Vec3::new( 4.5, -3.0, CEILING_LIGHT_HEIGHT),
];

////////////////////////////////////////////////////////////////////////////////////////////////////
// Infrastructure
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const LEFT_VENT_POSITION: Vec3 =
    Vec3::new(-1.5, 3.0, VENT_HEIGHT);

pub const RIGHT_VENT_POSITION: Vec3 =
    Vec3::new(1.5, 3.0, VENT_HEIGHT);

pub const SECURITY_CAMERA_POSITION: Vec3 =
    Vec3::new(
        ROOM_WIDTH * 0.5 - 0.25,
        ROOM_DEPTH * 0.5 - 0.25,
        CAMERA_HEIGHT,
    );

////////////////////////////////////////////////////////////////////////////////////////////////////
// Furniture
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const DESK_POSITION: Vec3 =
    Vec3::new(-2.2, -2.8, 0.0);

pub const CHAIR_POSITION: Vec3 =
    Vec3::new(-2.2, -1.8, 0.0);

pub const TERMINAL_POSITION: Vec3 =
    Vec3::new(4.2, 2.2, 0.0);

pub const SERVER_RACK_POSITION: Vec3 =
    Vec3::new(-6.0, 3.6, 0.0);

pub const CABINET_POSITION: Vec3 =
    Vec3::new(6.0, 3.6, 0.0);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Decoration
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const SUDO_LOGO_POSITION: Vec3 =
    Vec3::new(
        2.0,
        -0.5,
        FLOOR_THICKNESS + 0.002,
    );

pub const SUDO_LOGO_SIZE: Vec2 =
    Vec2::new(1.75, 1.75);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Spawn Lobby
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn spawn_lobby(
    commands: &mut Commands,
    asset_server: &AssetServer,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    origin: Vec3,
) {

    ////////////////////////////////////////////////////////////////////
    // Architecture
    ////////////////////////////////////////////////////////////////////

    floor(
        commands,
        origin + FLOOR_POSITION,
        Vec2::new(
            ROOM_WIDTH,
            ROOM_DEPTH,
        ),
    );

    ceiling(
        commands,
        origin + CEILING_POSITION,
        Vec2::new(
            ROOM_WIDTH,
            ROOM_DEPTH,
        ),
    );

    wall(
        commands,
        origin + NORTH_WALL_POSITION,
        Vec2::new(
            ROOM_WIDTH,
            WALL_THICKNESS,
        ),
    );

    wall(
        commands,
        origin + SOUTH_WALL_POSITION,
        Vec2::new(
            ROOM_WIDTH,
            WALL_THICKNESS,
        ),
    );

    wall(
        commands,
        origin + EAST_WALL_POSITION,
        Vec2::new(
            WALL_THICKNESS,
            ROOM_DEPTH,
        ),
    );

    wall(
        commands,
        origin + WEST_WALL_POSITION,
        Vec2::new(
            WALL_THICKNESS,
            ROOM_DEPTH,
        ),
    );

    door(
        commands,
        origin + ENTRANCE_DOOR_POSITION,
        Vec2::new(
            DOOR_WIDTH,
            DOOR_THICKNESS,
        ),
    );

    door(
        commands,
        origin + EXIT_DOOR_POSITION,
        Vec2::new(
            DOOR_WIDTH,
            DOOR_THICKNESS,
        ),
    );

    ////////////////////////////////////////////////////////////////////
    // Lighting
    ////////////////////////////////////////////////////////////////////

    for position in LIGHT_POSITIONS {

        ceiling_light(
            commands,
            meshes,
            materials,
            origin + position,
        );
    }

    ////////////////////////////////////////////////////////////////////
    // Infrastructure
    ////////////////////////////////////////////////////////////////////

    vent(
        commands,
        origin + LEFT_VENT_POSITION,
    );

    vent(
        commands,
        origin + RIGHT_VENT_POSITION,
    );

    security_camera(
        commands,
        origin + SECURITY_CAMERA_POSITION,
    );

    ////////////////////////////////////////////////////////////////////
    // Decoration
    ////////////////////////////////////////////////////////////////////

    decal(
        commands,
        asset_server,
        meshes,
        materials,
        "textures/sudo_logo.png",
        origin + SUDO_LOGO_POSITION,
        SUDO_LOGO_SIZE,
        Quat::from_rotation_x(
            -std::f32::consts::FRAC_PI_2,
        ),
    );

    ////////////////////////////////////////////////////////////////////
    // Furniture
    ////////////////////////////////////////////////////////////////////

    desk(
        commands,
        meshes,
        materials,
        DESK_ID,
        origin + DESK_POSITION,
    );

    chair(
        commands,
        meshes,
        materials,
        CHAIR_ID,
        origin + CHAIR_POSITION,
    );

    cabinet(
        commands,
        meshes,
        materials,
        CABINET_ID,
        origin + CABINET_POSITION,
    );

    server_rack(
        commands,
        meshes,
        materials,
        SERVER_RACK_ID,
        origin + SERVER_RACK_POSITION,
    );

    ////////////////////////////////////////////////////////////////////
    // Interactables
    ////////////////////////////////////////////////////////////////////

    terminal(
        commands,
        meshes,
        materials,
        origin + TERMINAL_POSITION,
    );
}