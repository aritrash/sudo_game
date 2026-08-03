//! ================================================================================================
//! Orientation Facility Map
//!
//! This module assembles the complete Orientation Facility by placing
//! each room at its corresponding world-space origin.
//!
//! Individual rooms are responsible for constructing their own
//! geometry and contents.
//! ================================================================================================

use bevy::prelude::*;

use super::spawn_lobby::spawn_lobby;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Room Origins
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const SPAWN_LOBBY_ORIGIN: Vec3 = Vec3::ZERO;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Facility Dimensions
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const FACILITY_WIDTH: f32 = 120.0;
pub const FACILITY_DEPTH: f32 = 18.0;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Camera
////////////////////////////////////////////////////////////////////////////////////////////////////

pub const CAMERA_POSITION: Vec3 =
    Vec3::new(0.0, 1.7, 6.0);

pub const CAMERA_TARGET: Vec3 =
    Vec3::new(0.0, 1.2, 0.0);

////////////////////////////////////////////////////////////////////////////////////////////////////
// Components
////////////////////////////////////////////////////////////////////////////////////////////////////

#[derive(Component)]
pub struct OrientationFacility;

#[derive(Component)]
pub struct OrientationCamera;

////////////////////////////////////////////////////////////////////////////////////////////////////
// Build Orientation Facility
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn build_orientation_facility(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    println!("Orientation: building facility");
    let root = commands
        .spawn((
            Name::new("Orientation Facility"),
            OrientationFacility,
            SpatialBundle::default(),
        ))
        .id();

    commands.entity(root).with_children(|_| {
        // Reserved for future room hierarchy.
    });

    spawn_lobby(
        &mut commands,
        &asset_server,
        &mut meshes,
        &mut materials,
        SPAWN_LOBBY_ORIGIN,
    );
}

////////////////////////////////////////////////////////////////////////////////////////////////////
// Despawn Orientation Facility
////////////////////////////////////////////////////////////////////////////////////////////////////

pub fn despawn_orientation_facility(
    mut commands: Commands,
    facility: Query<Entity, With<OrientationFacility>>,
) {
    for entity in &facility {
        commands.entity(entity).despawn_recursive();
    }
}