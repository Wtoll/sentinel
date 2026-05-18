//! Saving the game
//! 

use std::{fs::create_dir_all, path::PathBuf};

use bevy::prelude::*;

/// Plugin for enabling the save manager
pub struct SavePlugin {
    /// A path to the save directory
    pub save_dir: PathBuf
}

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {

        create_dir_all(self.save_dir.clone()).unwrap(); // TODO: Remove unwrap



    }
}