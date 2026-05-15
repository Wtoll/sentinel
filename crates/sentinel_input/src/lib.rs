#![forbid(missing_docs)]
//! Input management system
//! 
//! # Modules
//! 
//! This crate is organized into modules roughly corresponding to how raw input is transformed into expressive actions.
//! 
//! |                   |                                                                                                  |
//! |-------------------|--------------------------------------------------------------------------------------------------|
//! | [`src::raw`]      | Converts input events into stores of raw input data as components on entities.                   |
//! | [`src::hal`]      | Provides user-definable mappings for interpreting raw input data as conceptual input primitives. |
//! | [`src`]           | Exposes an interface for input sinks to read input primitives.                                   |
//! | [`sink`]          | Tools for assigning sinks and reading input primitives from sources.                             |
//! | [`sink::profile`] | Provides user-definable profiles for remapping the virtual input layout.                         |
//! | [`sink::combo`]   | Defines mappings for interpreting sequences and combinations of inputs as discrete actions.      |
//! 
//! ## [`src::raw`]
//! 
//! ## [`src::hal`]
//! 
//! ## [`src`]
//! 
//! ## [`sink`]
//! 
//! ## [`sink::profile`]
//! 
//! ## [`sink::combo`]
//! 

use bevy::{input::keyboard::keyboard_input_system, prelude::*};

pub mod src;
pub mod sink;

/// The plugin for enabling input management features.
pub struct Plugin;

impl bevy::prelude::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, src::raw::keyboard::spawn_virtual_keyboard)
            .add_systems(PreUpdate, src::raw::keyboard::update_virtual_keyboards
                .after(keyboard_input_system));
    }
}

