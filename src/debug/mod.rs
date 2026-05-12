//! Plugins for debugging the game
//! 

use bevy::{app::PluginGroupBuilder, prelude::*};

mod component_inspector;

/// A plugin group for adding debug plugins
pub struct DebugPlugins;

impl PluginGroup for DebugPlugins {
    fn build(self) -> bevy::app::PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(component_inspector::ComponentInspectorPlugin)
    }
}
