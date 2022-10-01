use bevy::prelude::*;

use crate::GameState;

pub struct InputPlugin;

pub struct KeyboardEvent(pub char);

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Actions>()
            .add_system_set(SystemSet::on_update(GameState::Cooking)
                                .with_system(process_input),
            )
            .add_event::<KeyboardEvent>();
    }
}

#[derive(Default)]
pub struct Actions {
    pub pressed: Vec<char>,
}

fn get_char(code: &KeyCode) -> Option<char> {
    match code {
        KeyCode::A => Some('a'),
        KeyCode::B => Some('b'),
        KeyCode::C => Some('c'),
        KeyCode::D => Some('d'),
        KeyCode::E => Some('e'),
        KeyCode::F => Some('f'),
        KeyCode::G => Some('g'),
        KeyCode::H => Some('h'),
        KeyCode::I => Some('i'),
        KeyCode::J => Some('j'),
        KeyCode::K => Some('k'),
        KeyCode::L => Some('l'),
        KeyCode::M => Some('m'),
        KeyCode::N => Some('n'),
        KeyCode::O => Some('o'),
        KeyCode::P => Some('p'),
        KeyCode::Q => Some('q'),
        KeyCode::R => Some('r'),
        KeyCode::S => Some('s'),
        KeyCode::T => Some('t'),
        KeyCode::U => Some('u'),
        KeyCode::V => Some('v'),
        KeyCode::W => Some('w'),
        KeyCode::X => Some('x'),
        KeyCode::Y => Some('y'),
        KeyCode::Z => Some('z'),
        KeyCode::Back => Some('<'),
        KeyCode::Space | KeyCode::Return => Some('>'),
        _ => None,
    }
}

fn process_input(
    mut actions: ResMut<Actions>,
    mut events: EventWriter<KeyboardEvent>,
    keyboard_input: Res<Input<KeyCode>>
) {
    for code in keyboard_input.get_just_pressed() {
        if let Some(char) = get_char(code) {
            events.send(KeyboardEvent(char));
        }
    }

    actions.pressed.clear();

    for code in keyboard_input.get_pressed() {
        if let Some(char) = get_char(code) {
            actions.pressed.push(char);
        }
    }
}