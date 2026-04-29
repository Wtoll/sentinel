//! Drafts of core game plugins

use bevy::app::{PluginGroup, PluginGroupBuilder};

pub mod debug;
pub use debug::DebugPlugin;

pub mod level_manager;
pub use level_manager::LevelManagerPlugin;

pub mod behavior;
pub use behavior::BehaviorPlugin;

pub mod viewport;
pub use viewport::ViewportPlugin;

pub mod assets;

pub mod editor;
pub use editor::EditorPlugin;

pub struct DraftPlugins;

impl PluginGroup for DraftPlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(DebugPlugin)
            .add(LevelManagerPlugin)
            .add(BehaviorPlugin)
            .add(ViewportPlugin)
            .add(EditorPlugin)
    }
}