//! Application viewport

use bevy::{camera::Viewport, color, math::{VectorSpace, ops::sqrt}, prelude::*};

use crate::core::{GameState, Player, physics::Velocity};

/// Plugin for enabling the game's viewport
pub struct ViewportPlugin;

impl Plugin for ViewportPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Startup, startup)
            .add_systems(Update, camera_control
                .run_if(in_state(GameState::Running)
                    .or(in_state(GameState::InGameMenu))));
    }
}

fn startup(mut commands: Commands) {
    commands.spawn((
        Name::new("Viewport"),
        Camera3d::default(),
        Camera2d::default(),
        Transform::from_translation(32.0 * Vec3::Z),
        CameraController::at_distance(32.0)
    ));

    commands.spawn((
        Name::new("Minimap"),
        Camera {
            viewport: Some(Viewport {
                physical_position: UVec2::new(10, 30),
                physical_size: UVec2::new(200, 100),
                ..default()
            }),
            order: 1,
            ..default()
        },
        Camera3d::default(),
        Camera2d::default(),
        Transform::from_translation(Vec3::Y * 16.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
    ));
}

/// Moves the main camera to the position of the player
#[derive(Component)]
#[require(Transform, Velocity)]
pub struct CameraController {
    distance: f32
}

impl CameraController {

    /// Construct a new camera controller at the given distance
    pub fn at_distance(distance: f32) -> Self {
        Self {
            distance,
        }
    }

}




/// 
/// 
/// 
/// ## Physics
/// 
/// The camera controller is modeled as a semi-rigid, prismatic beam with one
/// "root" endpoint coplanar to the game, and the other projected a known
/// distance normal to that plane. It's subject to three forces:
///  1. A coplanar acceleration, applied at the root, towards a point of
///     interest located at the mean position of all players but constrained by
///     local viewbounds.
///  2. An acceleration parallel to the beam's axis proportional to the
///     difference between the current distance and target distance
///  3. An angular spring restoring force, applied at the root, proportional to
///     the angle of deflection from the game plane's normal
/// 
fn camera_control(
    mut gizmos: Gizmos,
    time: Res<Time>,
    player_transforms: Query<&Transform, With<Player>>,
    camera_params: Query<(&CameraController, &mut Transform, &mut Velocity), (With<Camera>, Without<Player>)>
) {
    
    let point_of_interest = {
        let mut position_sum = Vec2::ZERO;

        for transform in player_transforms {
            position_sum += transform.translation.xy();
        }

        if player_transforms.count() > 0 {
            position_sum /= player_transforms.count() as f32;
        }

        position_sum
    };

    for (controller, mut transform, mut velocity) in camera_params {

        let root_target = Vec3::new(point_of_interest.x, point_of_interest.y, 0.0);

        let projection = transform.forward() * controller.distance;
        let root_projected = transform.translation + projection;

        let angular_transfer_ratio = 0.25;

        let linear_undamped_frequency = 5.0;
        let linear_damping_ratio = 2.0;

        let angular_undamped_frequency = 30.0;
        let angular_damping_ratio = 2.0;

        let linear_acceleration = (-linear_undamped_frequency) * ((linear_undamped_frequency * (root_projected - root_target)) + (linear_damping_ratio * velocity.linear));
        
        let angular_restoring = (-angular_undamped_frequency) * ((angular_undamped_frequency * transform.rotation.to_scaled_axis()) + (angular_damping_ratio * velocity.angular));

        let angular_acceleration = (angular_transfer_ratio * linear_acceleration.cross(-projection)) + angular_restoring;

        let dt = time.delta_secs();

        velocity.linear += linear_acceleration * dt;
        velocity.angular += angular_acceleration * dt;

        transform.rotate_around(root_projected, Quat::from_scaled_axis(velocity.angular * dt));
        transform.translation += velocity.linear * dt;

    }

}