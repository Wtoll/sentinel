use std::{cell::OnceCell, collections::HashMap, marker::PhantomData};

use bevy::prelude::*;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, initialize_assets);
    }
}








fn initialize_assets(
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>
) {

}