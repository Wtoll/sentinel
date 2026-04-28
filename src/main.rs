//!
//! 
//! ### System Orders
//! 
//! Systems execute in the following order:
//! * On first run:
//!     * `PreStartup`
//!     * `Startup`
//!     * `PostStartup`
//! * Then:
//!     * `First`
//!     * `PreUpdate`
//!         * Gamepad entities are updated
//!     * `StateTransition`
//!     * `RunFixedMainLoop`
//!         * `FixedMain` (zero or more times, proportional to the time difference)
//!     * `Update`
//!     * `PostUpdate`
//!     * `Last`
//! 

use bevy::{app::MainScheduleOrder, prelude::*};

use crate::{input::ProcessGameInput, movement_controllers::ProcessMovementControllers};

pub mod viewport;
pub mod level;
pub mod input;
pub mod debug;
pub mod movement_controllers;
pub mod brains;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins)
        .add_plugins((
            level::manager::Plugin,
            viewport::Plugin,
            input::GameInputPlugin,
            debug::Plugin,
            movement_controllers::MovementControllerPlugin,
            brains::BrainsPlugin
        ));

    let mut schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();

    schedule_order.insert_after(PreUpdate, ProcessGameInput);
    schedule_order.insert_after(Update, ProcessMovementControllers);

    app.run();
}
