//! Sentinel libraries

#![forbid(missing_docs)] // Swap from "warn" to "forbid" as needed

pub mod core;
pub mod draft;

/// Combined preludes for main crates
pub mod prelude {
    pub use bevy::prelude::*;
    pub use leafwing_input_manager::prelude::*;
}