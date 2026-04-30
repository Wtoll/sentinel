//! The core plugins 

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod physics;
pub use physics::PhysicsPlugin;

pub mod input;
pub use input::GameInputPlugin;

pub mod player;
pub use player::Player;

pub mod app_state;
pub use app_state::{AppStatePlugin, AppState, PauseState};

/// A plugin group for enabling the core plugins
pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(PhysicsPlugin)
            .add(GameInputPlugin)
            .add(AppStatePlugin)
    }
}