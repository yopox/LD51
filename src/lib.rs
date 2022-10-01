use bevy::app::App;
use bevy::prelude::*;

use crate::audio::InternalAudioPlugin;
use crate::button::ButtonPlugin;
use crate::input::InputPlugin;
use crate::loading::LoadingPlugin;
use crate::menu::MenuPlugin;

mod input;
mod audio;
mod loading;
mod menu;
mod button;
mod data;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Playing,
    Menu,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(MenuPlugin)
            .add_plugin(InputPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(ButtonPlugin);
    }
}
