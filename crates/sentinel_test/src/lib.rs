#![forbid(missing_docs)]
#![allow(unused)]
//! Utilities for writing tests for Bevy
//! 

use std::time::Duration;

use bevy::prelude::*;

/// Helpful imports for test modules
pub mod prelude {
    pub use crate::{
        test_app,
        exit::{
            cycles,
            time,
            then_exit
        },
        app::{
            SteppableApp,
            StepResult
        }
    };
}

pub mod exit;

pub mod app;

/// Runs a quick test app, allowing it to be configured using the closure, and
/// ensuring sensible defaults are set if not done explicitly.
pub fn test_app<F: FnOnce(&mut App)>(f: F) {
    let mut app = App::new();
    f(&mut app);
    ensure_minimal(&mut app);
    app.run();
}

fn ensure_minimal(app: &mut App) {
    if !app.is_plugin_added::<bevy::app::TaskPoolPlugin>() {
        app.add_plugins(bevy::app::TaskPoolPlugin {
            ..Default::default()
        });
    }

    if !app.is_plugin_added::<bevy::diagnostic::FrameCountPlugin>() {
        app.add_plugins(bevy::diagnostic::FrameCountPlugin);
    }

    if !app.is_plugin_added::<bevy::time::TimePlugin>() {
        app.add_plugins(bevy::time::TimePlugin);
    }

    if !app.is_plugin_added::<bevy::app::ScheduleRunnerPlugin>() {
        app.add_plugins(bevy::app::ScheduleRunnerPlugin::run_loop(Duration::ZERO));
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use bevy::{app::{App, Startup, Update}, ecs::system::IntoSystem};

    use super::{exit::{cycles, then_exit, time}, test_app};

    #[test]
    fn test_cycles() {
        test_app(|app: &mut App| {
            app
                .add_systems(Startup, || {
                    println!("Startup");
                })
                .add_systems(Update, || {
                    println!("Update");
                })
                .add_systems(Update, cycles(100)
                    .pipe(then_exit));
        });
    }

    #[test]
    fn test_time() {
        test_app(|app: &mut App| {
            app
                .add_systems(Startup, || {
                    println!("Startup");
                })
                .add_systems(Update, || {
                    println!("Update");
                })
                .add_systems(Update, time(Duration::from_secs(5))
                    .pipe(then_exit));
        });
    }
}
