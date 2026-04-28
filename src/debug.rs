use std::ops::DerefMut;

use bevy::{color, math::VectorSpace, prelude::*};
use bevy_rapier2d::prelude::*;

use crate::level::player::Player;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, draw_gizmos)
            // .add_systems(Update, print_player)
            .add_systems(Last, respawner);
    }
}

fn draw_gizmos(mut gizmos: Gizmos) {
    gizmos.arrow(Vec3::ZERO, Vec3::X, color::palettes::basic::RED);
    gizmos.arrow(Vec3::ZERO, Vec3::Y, color::palettes::basic::GREEN);
    gizmos.arrow(Vec3::ZERO, Vec3::Z, color::palettes::basic::BLUE);
}

fn respawner(mut query: Query<(&mut Transform, Option<&mut Velocity>), With<Player>>) {
    for (mut transform, velocity) in query {
        if transform.translation.y < -10.0 {
            *transform = Transform::from_translation(Vec3::Y * 0.5);

            if let Some(mut velocity) = velocity {
                *velocity = Velocity::zero();
            }
        }
    }
}