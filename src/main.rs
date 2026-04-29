//! # Sentinel
//! 
//! ## Roadmap
//! 
//! 
//! 

use bevy::prelude::*;
use sentinel::{
    core::CorePlugins,
    draft::DraftPlugins
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CorePlugins)
        .add_plugins(DraftPlugins)
        .run();
}
