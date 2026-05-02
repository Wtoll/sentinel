//! Level manager

use std::{fs::File, io::Write, path::PathBuf};

use bevy::{asset::AssetPath, prelude::*, scene::SceneInstanceReady, tasks::{IoTaskPool, Task, futures::check_ready}};

use crate::core::{AppState, GameState, player::PlayerManager};

use rayon::prelude::*;

/// Plugin for enabling the level manager
pub struct LevelManagerPlugin;

impl Plugin for LevelManagerPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_systems(OnEnter(GameState::Entering), queue_scene_loader)
            .add_observer(scene_loaded)
            .add_systems(OnEnter(GameState::Leaving), queue_serializers)
            .add_systems(Update, leave_game.run_if(in_state(GameState::Leaving)));
    }
}

fn queue_scene_loader(
    mut commands: Commands, 
    mut meshes: ResMut<Assets<Mesh>>, 
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>
) {
    info!("Entering the game");

    commands.spawn((
        DynamicSceneRoot(asset_server.load("scenes/test-load.scn.ron"))
    ));

    commands.queue(PlayerManager::spawn_player());

    commands.spawn((
        DespawnOnExit(AppState::InGame),
        Mesh3d(meshes.add(Cuboid::new(10.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -1.0, 0.0)
    ));
}

fn scene_loaded(
    event: On<SceneInstanceReady>
) {
    info!("Scene Loaded");
}

#[derive(Component)]
struct SerializerTask(pub Task<Result<usize, SceneSerializationError>>);

/// The error conditions that may occur when attempting to serialize a scene to a file
#[derive(Debug)]
pub enum SceneSerializationError {
    /// A serialization error
    RonError(ron::error::Error),
    /// A file system error
    IoError(std::io::Error)
}

impl From<ron::error::Error> for SceneSerializationError {
    fn from(value: ron::error::Error) -> Self {
        Self::RonError(value)
    }
}

impl From<std::io::Error> for SceneSerializationError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}

fn queue_serializers(
    world: &World,
    mut commands: Commands,
    query: Query<(Entity, &DynamicSceneRoot, Option<&Children>)>
) {
    // TODO: Evaluate how necessary the weird parallelism stuff is 

    info!("Leaving the game");

    let (entities, query_params): (Vec<Entity>, Vec<(&AssetPath, Option<&Children>)>) = query.iter()
        .filter_map(|(entity, scene_root, scene_children)| {
            scene_root.0.path()
                .map(|path| {
                    debug!("Attempting to serialize scene root {} to path {}", entity, path);
                    (entity, (path, scene_children))
                })
        }).unzip();

    let type_registry = world.resource::<AppTypeRegistry>().clone();

    let scenes: Vec<(PathBuf, DynamicScene)> = query_params.into_par_iter()
        .map(|(path, scene_children)| {

            let mut path_buf = PathBuf::from("assets");
            path_buf.push(path.path());

            let mut scene_builder = DynamicSceneBuilder::from_world(world)
                .deny_component::<ChildOf>();

            if scene_children.is_some() {
                scene_builder = scene_builder.extract_entities(scene_children.unwrap().iter());
            }

            (path_buf, scene_builder.build())

        }).collect();

    let task_pool = IoTaskPool::get();

    let tasks: Vec<SerializerTask> = scenes.into_iter()
        .map(|(path, scene)| {
            let type_registry = type_registry.clone();

            SerializerTask(task_pool.spawn(async move {
                scene.serialize(&type_registry.read())
                    .map_err(SceneSerializationError::from)
                    .and_then(|serialized| {
                        File::create(path)
                            .and_then(|mut file| file.write(serialized.as_bytes()))
                            .map_err(SceneSerializationError::from)
                    })
            }))
        }).collect();

    for entity in entities {
        info!("Despawning entity {}", entity);
        commands.entity(entity).despawn();
    }

    commands.spawn_batch(tasks);
}

fn leave_game(
    mut commands: Commands,
    mut serializers: Query<(Entity, &mut SerializerTask)>,
    mut next_state: ResMut<NextState<AppState>>
) {

    info!("Waiting on futures");

    if serializers.count() > 0 {
        for (entity, mut task) in &mut serializers {
            match check_ready(&mut task.0) {
                Some(res) => {
                    commands.entity(entity).despawn();
                    match res {
                        Ok(_) => info!("Successfully serialized a scene"),
                        Err(e) => {
                            info!("There was an error serializing a scene {:?}", e);
                        },
                    }
                },
                _ => {},
            }
        }
    } else {
        info!("All serializing futures have been completed!");

        next_state.set(AppState::MainMenu);

    }
}

