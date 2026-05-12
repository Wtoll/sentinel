use crate::core::{level::room::{Room, RoomVolume}, scene::generators::{FileSceneWriter, InteractiveScene}};

use bevy::prelude::*;

pub fn generate() {
    FileSceneWriter::from("assets/scenes/new-test.scn.ron")
        .apply_world(|world| {

            world
                .spawn(Room {
                    volume: RoomVolume::simple(Vec3::new(10.0, 3.0, 1.0))
                })
                .with_child(Name::new("Object In Room"));

        })
        .write()
        .expect("Problem writing to file");
}