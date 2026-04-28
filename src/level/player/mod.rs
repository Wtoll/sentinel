use bevy::prelude::*;

use crate::movement_controllers::MovementController;

#[derive(Component)]
#[require(Transform, MovementController)]
pub struct Player;

pub fn spawn_player<M: Material>(
    commands: &mut Commands,
    mesh: Handle<Mesh>,
    material: Handle<M>
) {
    commands.spawn((
        Player,
        Mesh3d(mesh),
        MeshMaterial3d(material)
    ));
}