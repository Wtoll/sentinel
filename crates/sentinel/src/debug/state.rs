//! Debugging of the global state system
//! 

use bevy::prelude::*;

use strum::IntoEnumIterator;

use crate::core::state::{AppState, GameState};

pub fn plugin(app: &mut App) {
    AppState::iter()
        .for_each(|state| {
            app
                .add_systems(OnEnter(state), enter_state(state))
                .add_systems(OnExit(state), exit_state(state));
        });

    GameState::iter()
        .for_each(|state| {
            app
                .add_systems(OnEnter(state), enter_state(state))
                .add_systems(OnExit(state), exit_state(state));
        });
}

fn enter_state<S: States>(s: S) -> impl Fn() {
    move || {
        info!("Entered state {:?}", s);
    }
}

fn exit_state<S: States>(s: S) -> impl Fn() {
    move || {
        info!("Exited state {:?}", s);
    }
}