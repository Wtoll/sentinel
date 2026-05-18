//! Sentinel core libraries
//! 

use std::sync::{Arc, Mutex};

use bevy::{app::{PluginGroup, PluginGroupBuilder}, camera::Viewport, prelude::*, window::{PrimaryWindow, RawHandleWrapperHolder}};

pub mod util;
use util::task::TaskPlugin;

pub mod state;
use state::StatePlugin;

pub mod input;

pub mod player;
use player::PlayerManagerPlugin;

pub mod world;
use world::WorldManagerPlugin;

pub mod render;
use render::camera::CameraControllerPlugin;

pub mod physics;

pub mod progression;

pub mod save;
use save::SavePlugin;

use crate::core::{player::PlayerManager, render::camera::CameraController, state::{AppState, GameState}};

/// Plugin group for the game's core plugins
pub struct CorePlugins;

impl PluginGroup for CorePlugins {
    fn build(self) -> PluginGroupBuilder {
        PluginGroupBuilder::start::<Self>()
            .add(plugin)
            .add(StatePlugin)
            .add(input::plugin)
            .add(PlayerManagerPlugin)
            .add(WorldManagerPlugin)
            .add(TaskPlugin)
            .add(CameraControllerPlugin)
            .add(progression::plugin)
            .add(SavePlugin {
                save_dir: util::dirs::save_directory()
            })
    }
}

fn plugin(app: &mut App) {
    app
        .add_systems(OnEnter(GameState::Running), game_enter);


    app.world_mut().spawn((
        Name::new("Primary Window"),
        PrimaryWindow,
        Window::default(),
        RawHandleWrapperHolder(Arc::new(Mutex::new(None)))
    ));
}

fn game_enter(
    mut commands: Commands,
    mut player_manager: ResMut<PlayerManager>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>
) {
    commands.spawn((
        Name::new("Main Camera"),
        Camera3d::default(),
        Camera2d,
        Transform::from_translation(32.0 * Vec3::Z),
        CameraController::at_distance(32.0)
    ));

    commands.spawn((
        Name::new("Minimap Camera"),
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
        Camera2d,
        Transform::from_translation(Vec3::Y * 16.0).looking_at(Vec3::ZERO, Vec3::NEG_Z),
    ));

    let player = player_manager
        .spawn_player(&mut commands)
        .insert(Transform::from_xyz(0.0, -4.0, 0.0))
        .id();

    commands.spawn((
        Name::new("Ground"),
        DespawnOnExit(AppState::Game),
        Mesh3d(meshes.add(Cuboid::new(10.0, 1.0, 1.0))),
        MeshMaterial3d(materials.add(Color::WHITE)),
        Transform::from_xyz(0.0, -5.0, 0.0)
    ));
}