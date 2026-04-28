use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::movement_controllers::MovementController;

#[derive(Component)]
#[require(Transform, RigidBody::Dynamic, MovementController, Collider)]
pub struct Player;

pub fn spawn_player<M: Material>(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<M>
) {
    commands.spawn((
        Player,
        Mesh3d(mesh),
        MeshMaterial3d(material),
        Collider::cuboid(0.5, 0.5)
    ));
}