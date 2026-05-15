//! Drafts of sentinel core libraries
//! 

use bevy::app::{PluginGroup, PluginGroupBuilder};

/// Plugin group for drafts of the game's core plugins
pub struct DraftPlugins;

impl PluginGroup for DraftPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
    }
}