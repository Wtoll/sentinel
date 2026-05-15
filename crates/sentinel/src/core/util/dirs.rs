//! Helpers for interacting with program files across platforms
//! 

use std::{path::PathBuf, sync::LazyLock};

use directories::ProjectDirs;

static DIRS: LazyLock<ProjectDirs> = LazyLock::new(|| {
    ProjectDirs::from("com", "wtoll", "Sentinel")
        .expect("Could not locate a directory for program files")
});

/// Returns the saves directory for the program
pub fn save_directory() -> PathBuf {
    DIRS.data_local_dir().join("saves")
}