use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::level::player;

pub struct Plugin;

impl bevy::app::Plugin for Plugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

fn startup(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {


    player::spawn_player(
        &mut commands,
        meshes.add(Cuboid::new(1.0, 1.0, 1.0)),
        materials.add(Color::hsl(1.0, 1.0, 0.5))
    );


    
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(10.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -1.0, 0.0),
        Collider::cuboid(10.0 / 2.0, 1.0 / 2.0)
    ));
}