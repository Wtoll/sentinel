//! Exit conditions
//! 

use std::{marker::PhantomData, sync::Mutex, time::{Duration, Instant}};

use bevy::prelude::*;

/// Exits the app after the exit predicate system is met
pub struct ExitPredicatePlugin<Marker, F>
where
    Marker: Send + Sync + 'static,
    F: ExitPredicate<Marker>
{
    phantom: PhantomData<Marker>,
    system: Mutex<Option<F>>
}

impl<Marker, F> From<F> for ExitPredicatePlugin<Marker, F>
where
    Marker: Send + Sync + 'static,
    F: ExitPredicate<Marker>
{
    fn from(value: F) -> Self {
        Self {
            phantom: PhantomData,
            system: Mutex::new(Some(value))
        }
    }
}

impl<Marker, F> Plugin for ExitPredicatePlugin<Marker, F>
where
    Marker: Send + Sync + 'static,
    F: ExitPredicate<Marker>
{
    fn build(&self, app: &mut App) {
        app.add_systems(
            Last,
            self.system.lock()
                .unwrap()
                .take()
                .unwrap()
                .pipe(then_exit)
            );
    }
}

/// System to pipe into that safely exits if the system before returned an exit code
pub fn then_exit(
    In(should_exit): In<Option<AppExit>>,
    mut writer: MessageWriter<AppExit>
) {
    if let Some(exit_flag) = should_exit {
        writer.write(exit_flag);
    }
}

/// A marker trait for an exit predicate system
pub trait ExitPredicate<Marker>: SystemParamFunction<Marker, In = (), Out = Option<AppExit>> { }

impl<T, Marker> ExitPredicate<Marker> for T
where
    T: SystemParamFunction<Marker, In = (), Out = Option<AppExit>>
{ }

/// A simple exit predicate that exits after a number of cycles
pub fn cycles(count: usize) -> impl FnMut(Local<usize>) -> Option<AppExit> {
    move |mut counter: Local<usize>| -> Option<AppExit> {
        *counter += 1;

        (*counter > count).then_some(AppExit::Success)
    }
}

/// A simple exit predicate that exits after a duration of time
pub fn time(duration: Duration) -> impl FnMut(Local<Option<Instant>>) -> Option<AppExit> {
    move |mut start: Local<Option<Instant>>| -> Option<AppExit> {
        (start.get_or_insert_with(Instant::now).elapsed() > duration)
            .then_some(AppExit::Success)
    }
}
