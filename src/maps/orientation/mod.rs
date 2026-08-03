pub mod map;

pub mod spawn_lobby;
pub mod movement_hall;
pub mod observation_room;
pub mod authentication_room;
pub mod packet_storage;
pub mod uplink_room;
pub mod quarantine_room;
pub mod freeze_chamber;
pub mod meeting_room;
pub mod graduation_hall;

pub use map::{
    build_orientation_facility,
    despawn_orientation_facility,
};