use bevy::prelude::*;

use crate::GameState;

pub struct InputPlugin;

struct KeyboardEvent(char);

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Actions>()
            .add_system_set(SystemSet::on_update(GameState::Playing)
                                .with_system(process_input),
            )
            .add_event::<KeyboardEvent>();
    }
}

#[derive(Default)]
pub struct Actions {
    pub pressed: Vec<char>,
}

fn process_input(
    mut actions: ResMut<Actions>,
    mut events: EventWriter<KeyboardEvent>,
    keyboard_input: Res<Input<KeyCode>>
) {
    if keyboard_input.just_pressed(KeyCode::A) { events.send(KeyboardEvent('a')); }
    if keyboard_input.just_pressed(KeyCode::B) { events.send(KeyboardEvent('b')); }
    if keyboard_input.just_pressed(KeyCode::C) { events.send(KeyboardEvent('c')); }
    if keyboard_input.just_pressed(KeyCode::D) { events.send(KeyboardEvent('d')); }

    actions.pressed.clear();

    if keyboard_input.pressed(KeyCode::A) { actions.pressed.push('a'); }
    if keyboard_input.pressed(KeyCode::B) { actions.pressed.push('b'); }
    if keyboard_input.pressed(KeyCode::C) { actions.pressed.push('c'); }
    if keyboard_input.pressed(KeyCode::D) { actions.pressed.push('d'); }
}