use bevy::app::App;
use bevy::prelude::*;
use bevy_tweening::Lens;

use crate::audio::InternalAudioPlugin;
use crate::button::ButtonPlugin;
use crate::cooking::CookingPlugin;
use crate::input::InputPlugin;
use crate::loading::LoadingPlugin;
use crate::order::OrderPlugin;
use crate::restaurant::RestaurantPlugin;
use crate::score::ScorePlugin;
use crate::title::TitlePlugin;
use crate::game_over::GameOverPlugin;

mod input;
mod audio;
mod loading;
mod title;
mod button;
mod ingredients;
mod cooking;
mod order;
mod restaurant;
mod score;
mod game_over;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Cooking,
    TitleScreen,
    GameOver,
}

#[derive(SystemLabel)]
pub enum Labels {
    Logic,
    UI,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(TitlePlugin)
            .add_plugin(InputPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(ButtonPlugin)
            .add_plugin(OrderPlugin)
            .add_plugin(CookingPlugin)
            .add_plugin(RestaurantPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(GameOverPlugin);
    }
}