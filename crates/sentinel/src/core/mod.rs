//! Sentinel core libraries
//! 

pub mod util;
use util::task::TaskPlugin;

pub mod state;
use state::StatePlugin;

pub mod input;

pub mod player;
use player::PlayerManagerPlugin;

pub mod world;
use world::WorldManagerPlugin;

use bevy::app::{PluginGroup, PluginGroupBuilder};

/// Plugin group for the game's core plugins
pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(StatePlugin)
            .add(input::plugin)
            .add(PlayerManagerPlugin)
            .add(WorldManagerPlugin)
            .add(TaskPlugin)
    }
}