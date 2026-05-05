//! # Sentinel
//! 
//! ## Roadmap
//! - [ ] Minimap
//! - [ ] Level serialization and deserialization
//! - [ ] Level editor
//! - [ ] Main Menu
//! - [ ] Options Menu
//! - [ ] Control Mappings
//! - [ ] Physics
//! - [ ] 
//! 
//! 

use std::env;

use bevy::prelude::*;
use leafwing_input_manager::prelude::*;
use sentinel::{
    core::{CorePlugins, input::GameAction},
    draft::DraftPlugins
};

fn main() {
    // SAFETY: The application is not yet multithreaded
    unsafe {
        env::set_var("RUST_LOG", "info,sentinel=debug");
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(InputManagerPlugin::<GameAction>::default())
        .add_plugins(CorePlugins)
        .add_plugins(DraftPlugins)
        .run();
}
