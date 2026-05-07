//! Creating and interacting with scenes
//! 

use bevy::prelude::*;

use std::{error::Error, fmt::Display};


mod generators;
pub use generators::SceneWriter;

use crate::core::scene::generators::SceneWriterError;

/// Plugin for scene management
pub struct ScenePlugin;

impl Plugin for ScenePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, generators::run_generators);
    }
}

/// A problem that may occur in the process of writing a scene to a file
#[derive(Debug)]
pub enum FileSceneWriterError {
    /// An invalid file path was given
    InvalidPath,
    /// An error occured in the scene writer
    SceneWriterError(SceneWriterError)
}

impl Display for FileSceneWriterError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FileSceneWriterError::InvalidPath => {
                write!(f, "Invalid path for scene serialization")
            },
            FileSceneWriterError::SceneWriterError(error) => error.fmt(f)
        }
    }
}

impl Error for FileSceneWriterError {}

impl<E> From<E> for FileSceneWriterError where SceneWriterError: From<E> {
    fn from(value: E) -> Self {
        Self::SceneWriterError(value.into())
    }
}

