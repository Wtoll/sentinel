//! Camera
//! 

use bevy::{camera::Viewport, prelude::*};

use crate::core::{physics::Velocity, player::Player, state::GameState};

/// A plugin for enabling the camera controller
pub struct CameraControllerPlugin;

impl Plugin for CameraControllerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(Update, CameraController::find_point_of_interest.pipe(CameraController::move_towards)
                .run_if(in_state(GameState::Running)
                    .or(in_state(GameState::InGameMenu))));
    }
}

/// Moves the main camera to the position of the player
/// 
/// ### Physics
/// 
/// It's two damped oscillators, one linear and then one angular.
/// 
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

    fn find_point_of_interest(
        player_transforms: Query<&Transform, With<Player>>
    ) -> Vec2 {
        let mut position_sum = Vec2::ZERO;

        for transform in player_transforms {
            position_sum += transform.translation.xy();
        }

        if player_transforms.count() > 0 {
            position_sum /= player_transforms.count() as f32;
        }

        position_sum
    }

    fn move_towards(
        In(point_of_interest): In<Vec2>,
        time: Res<Time>,
        camera_params: Query<(&CameraController, &mut Transform, &mut Velocity), (With<Camera>, Without<Player>)>,
    ) {
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
}