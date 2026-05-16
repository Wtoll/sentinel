//! Keyboard
//! 

use bevy::prelude::*;

/// A component marking an entity as a virtual keyboard.
#[derive(Component)]
pub struct Keyboard {
    key_codes: ButtonInput<KeyCode>
}

pub(crate) fn spawn_virtual_keyboard(
    mut commands: Commands
) {
    commands.spawn((
        Name::new("Virtual Keyboard"),
        Keyboard {
            key_codes: Default::default()
        }
    ));
}

pub(crate) fn update_virtual_keyboards(
    mut keyboards: Query<&mut Keyboard>,
    key_codes: Res<ButtonInput<KeyCode>>
) {
    for mut keyboard in keyboards.iter_mut() {
        for key in key_codes.get_just_pressed() {
            keyboard.key_codes.press(*key);
        }

        for key in key_codes.get_just_released() {
            keyboard.key_codes.release(*key);
        }
    }
}