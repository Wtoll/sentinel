//! File generators for dynamic scenes
//! 
//! 


use std::{fmt::Display, error::Error, fs::File, io::Write, path::PathBuf, sync::{Arc, LazyLock, RwLock}};

use bevy::{prelude::*, reflect::{TypeRegistry, TypeRegistryArc}};

use crate::core::scene::FileSceneWriterError;

static TYPE_REGISTRY: LazyLock<TypeRegistryArc> = LazyLock::new(|| {
    let mut registry = TypeRegistry::empty();

    registry.register_derived_types();

    TypeRegistryArc { internal: Arc::new(RwLock::new(registry)) }
});

pub fn global_type_registry() -> AppTypeRegistry {
    AppTypeRegistry(TYPE_REGISTRY.clone())
}






pub fn run_generators() {
    start_room::generate();
}




pub trait InteractiveScene {
    fn world(&self) -> &World;

    fn world_mut(&mut self) -> &mut World;

    fn apply_world<F: FnOnce(&mut World)>(&mut self, f: F) -> &mut Self {
        f(self.world_mut());
        self
    }
}







/// Helper for writing scenes to writers
pub struct SceneWriter(World);

impl SceneWriter {
    /// Writes the scene to the given [`Writer`](std::fmt::Writer)
    pub fn write_to<W: Write>(&self, writer: &mut W) -> Result<(), SceneWriterError> {
        writer.write_all(DynamicScene::from_world(self.world())
            .serialize(&self.world().resource::<AppTypeRegistry>().read())?.as_bytes())?;

        Ok(())
    }
}

impl InteractiveScene for SceneWriter {
    fn world_mut(&mut self) -> &mut World {
        &mut self.0
    }
    
    fn world(&self) -> &World {
        &self.0
    }
}

impl Default for SceneWriter {
    fn default() -> Self {
        let mut world = World::new();

        world.insert_resource(global_type_registry());

        Self(world)
    }
}

#[derive(Debug)]
/// An error that occurs in the process of writing a scene to a writer
pub enum SceneWriterError {
    SerializationError(ron::Error),
    IoError(std::io::Error)
}

impl Display for SceneWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SceneWriterError::SerializationError(error) => error.fmt(f),
            SceneWriterError::IoError(error) => error.fmt(f),
        }
    }
}

impl Error for SceneWriterError {}

impl From<ron::Error> for SceneWriterError {
    fn from(value: ron::Error) -> Self {
        Self::SerializationError(value)
    }
}

impl From<std::io::Error> for SceneWriterError {
    fn from(value: std::io::Error) -> Self {
        Self::IoError(value)
    }
}




/// Helper for writing scenes to files
pub struct FileSceneWriter {
    path: PathBuf,
    writer: SceneWriter
}

impl InteractiveScene for FileSceneWriter {
    fn world(&self) -> &World {
        self.writer.world()
    }

    fn world_mut(&mut self) -> &mut World {
        self.writer.world_mut()
    }
}

impl FileSceneWriter {
    pub fn new(path: PathBuf) -> Self {
        Self {
            path,
            writer: Default::default()
        }
    }

    pub fn write(&self) -> Result<(), FileSceneWriterError> {
        self.writer.write_to(&mut File::create(&self.path)?)?;

        Ok(())
    }
}

impl<P> From<P> for FileSceneWriter where PathBuf: From<P> {
    fn from(value: P) -> Self {
        Self::new(PathBuf::from(value))
    }
}






mod start_room;
