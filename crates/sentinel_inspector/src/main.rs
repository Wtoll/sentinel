use bevy::prelude::*;
use sentinel_inspector::InspectorPlugin;

fn main() {
    App::new()
        .add_plugins(InspectorPlugin)
        .run();
}
