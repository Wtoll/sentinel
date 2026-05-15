//! App Things
//! 

use bevy::{ecs::system::RunSystemOnce, prelude::*};
use std::{error::Error, marker::PhantomData, result::Result};

/// An alias for the result of a call to step
pub type StepResult<E> = Result<Option<AppExit>, E>;

/// An app that can be stepped through
pub struct SteppableApp<E: 'static = ()> {
    phantom: PhantomData<E>,
    exit: Option<AppExit>,
    app: App
}

impl Default for SteppableApp {
    fn default() -> Self {
        let mut app = App::new();

        app.add_plugins(MinimalPlugins);

        Self {
            phantom: Default::default(),
            exit: Default::default(),
            app
        }
    }
}

impl SteppableApp {
    /// Runs a system, then updates the app once
    pub fn step<F, Marker>(mut self, f: F) -> Self
    where
        F: IntoSystem<(), (), Marker>,
        Marker: Send + Sync + 'static
    {
        if self.exit.is_none() {
            self.app.world_mut()
                .run_system_once(f)
                .unwrap();
            self.app.update();
        }
        self
    }

    /// Exits the app
    fn finish(self) -> AppExit {
        self.exit.unwrap_or_default()
    }
}

impl<E: 'static> SteppableApp<E> {
    /// Runs a system with error handling, then updates the app once
    pub fn step_res<F, Marker, T>(mut self, f: F) -> Result<Self, E>
    where
        F: IntoSystem<(), StepResult<T>, Marker>,
        Marker: Send + Sync + 'static,
        T: Into<E>
    {
        if self.exit.is_none() {
            self.app.world_mut()
                .run_system_once(f)
                .unwrap()
                .map_err(Into::into)
                .map(|exit| {
                    match exit {
                        Some(exit) => self.exit = Some(exit),
                        None => self.app.update(),
                    };
                    self
                })
        } else {
            Ok(self)
        }
    }
}

impl<E: 'static> From<App> for SteppableApp<E> {
    fn from(mut app: App) -> Self {

        app.finish();
        app.cleanup();

        Self {
            phantom: PhantomData,
            exit: None,
            app,
        }
    }
}

#[cfg(test)]
mod test {

    use bevy::ecs::entity::SpawnError;
    use bevy::prelude::*;

    use crate::prelude::*;

    use crate::app::{StepResult, SteppableApp};

    #[test]
    fn stepped() -> Result<(), ()> {

        SteppableApp::default()
            .step(|world: &mut World| {
                
            })
            .step_res(|world: &mut World| -> StepResult<()> {
                Ok(None)
            })?
            .finish();

        Ok(())

    }



}