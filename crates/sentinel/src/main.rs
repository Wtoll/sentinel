//! Sentinel

use bevy::{app::PluginGroupBuilder, prelude::*};
use sentinel::{core::CorePlugins, debug::DebugPlugins};

fn main() {
    // SAFETY: The application is not yet multithreaded
    #[cfg(feature = "debug")] unsafe {
        std::env::set_var("RUST_LOG", "info,sentinel=debug");
    }

    let mut app = App::new();

    app
        .add_plugins(BevyPlugins)
        .add_plugins(CorePlugins);

    #[cfg(feature = "debug")]
    app.add_plugins(DebugPlugins);

    app.run();
}

/// Reimplements the bevy default plugins with some additional configuration
struct BevyPlugins;

impl PluginGroup for BevyPlugins {
    fn build(self) -> PluginGroupBuilder {
        let mut builder = PluginGroupBuilder::start::<Self>()
            .add(bevy::log::LogPlugin {
                ..default()
            })
            .add(bevy::app::TaskPoolPlugin {
                ..default()
            })
            .add(bevy::diagnostic::FrameCountPlugin)
            .add(bevy::time::TimePlugin)
            .add(bevy::transform::TransformPlugin)
            .add(bevy::input::InputPlugin)
            .add(bevy::window::WindowPlugin {
                ..default()
            })
            .add(bevy::a11y::AccessibilityPlugin);

        #[cfg(any(all(unix, not(target_os = "horizon")), windows))] {
            builder = builder.add(bevy::app::TerminalCtrlCHandlerPlugin);
        }

        builder = builder
            .add(bevy::asset::AssetPlugin {
                ..default()
            })
            .add(bevy::scene::ScenePlugin)
            // NOTE: After [`AssetPlugin`] to enable custom cursors
            .add(bevy::winit::WinitPlugin {
                ..default()
            })
            .add(bevy::render::RenderPlugin {
                ..default()
            })
            .add(bevy::image::ImagePlugin {
                ..default()
            })
            .add(bevy::mesh::MeshPlugin)
            .add(bevy::camera::CameraPlugin)
            .add(bevy::light::LightPlugin);

        #[cfg(not(target_arch = "wasm32"))] {
            builder = builder.add(bevy::render::pipelined_rendering::PipelinedRenderingPlugin);
        }

        builder = builder
            .add(bevy::core_pipeline::CorePipelinePlugin)
            .add(bevy::post_process::PostProcessPlugin)
            .add(bevy::anti_alias::AntiAliasPlugin)
            .add(bevy::sprite::SpritePlugin)
            .add(bevy::sprite_render::SpriteRenderPlugin)
            .add(bevy::text::TextPlugin)
            .add(bevy::ui::UiPlugin)
            .add(bevy::ui_render::UiRenderPlugin)
            .add(bevy::pbr::PbrPlugin {
                ..default()
            })
            .add(bevy::gltf::GltfPlugin {
                ..default()
            })
            .add(bevy::audio::AudioPlugin {
                ..default()
            })
            .add(bevy::gilrs::GilrsPlugin)
            .add(bevy::animation::AnimationPlugin)
            .add(bevy::gizmos::GizmoPlugin)
            .add(bevy::gizmos_render::GizmoRenderPlugin)
            .add(bevy::state::app::StatesPlugin)
            .add_group(bevy::picking::DefaultPickingPlugins);

        builder
    }
}
