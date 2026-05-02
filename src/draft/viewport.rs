//! Application viewport

use bevy::prelude::*;

/// Plugin for enabling the game's viewport
pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup);
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Name::new("Viewport"),
        Camera3d::default(),
        Camera2d::default(),
        Transform::from_translation(15.0 * Vec3::Z).looking_to(Vec3::NEG_Z, Vec3::Y),
    ));
}