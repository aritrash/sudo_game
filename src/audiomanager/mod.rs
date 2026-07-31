//! Audio subsystem for S.U.D.O.
//!
//! This module owns all runtime audio playback.
//!
//! No other subsystem should spawn music, sound effects,
//! or Pong voice audio directly.
//!
//! All audio requests should be performed through events.

pub mod manager;
pub mod music;
pub mod pong;
pub mod settings;
pub mod sfx;

pub use manager::AudioManagerPlugin;
