//! Libraries for debugging sentinel
//! 

use bevy::app::{PluginGroup, PluginGroupBuilder};
use sentinel_inspector::InspectorPlugin;

pub mod sandbox;
use sandbox::SandboxPlugin;

mod state;

/// Plugins for debugging sentinel
pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(InspectorPlugin)
            .add(SandboxPlugin)
            .add(state::plugin)
    }
}