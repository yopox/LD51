use bevy::prelude::*;

pub struct InputPlugin;

pub struct KeyboardEvent(pub char);
pub struct KeyboardReleaseEvent(pub char);

// This plugin listens for keyboard input and converts the input into Actions
// Actions can then be used as a resource in other systems to act on the player input.
impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<Actions>()
            .add_system(process_input)
            .add_event::<KeyboardEvent>()
            .add_event::<KeyboardReleaseEvent>();
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
        KeyCode::Space | KeyCode::Return => Some(' '),
        _ => None,
    }
}

pub fn process_input(
    mut actions: ResMut<Actions>,
    mut events: EventWriter<KeyboardEvent>,
    mut released_events: EventWriter<KeyboardReleaseEvent>,
    keyboard_input: Res<Input<KeyCode>>
) {
    for code in keyboard_input.get_just_pressed() {
        if let Some(char) = get_char(code) {
            events.send(KeyboardEvent(char));
        }
    }
    for code in keyboard_input.get_just_released() {
        if let Some(char) = get_char(code) {
            released_events.send(KeyboardReleaseEvent(char));
        }
    }

    actions.pressed.clear();

    for code in keyboard_input.get_pressed() {
        if let Some(char) = get_char(code) {
            actions.pressed.push(char);
        }
    }
}