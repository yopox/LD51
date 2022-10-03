use bevy::app::App;
use bevy::ecs::system::EntityCommands;
use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_kira_audio::AudioPlugin;
use bevy_tweening::component_animator_system;

use crate::audio::InternalAudioPlugin;
use crate::button::ButtonPlugin;
use crate::chef::ChefPlugin;
use crate::cooking::CookingPlugin;
use crate::customer::CustomerPlugin;
use crate::game_over::GameOverPlugin;
use crate::input::InputPlugin;
use crate::loading::LoadingPlugin;
use crate::order::OrderPlugin;
use crate::restaurant::RestaurantPlugin;
use crate::score::ScorePlugin;
use crate::title::TitlePlugin;
use crate::tween::TweenPlugin;

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
mod customer;
mod tween;
mod chef;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
enum GameState {
    Loading,
    Cooking,
    TitleScreen,
    GameOver,
}

#[derive(SystemLabel)]
pub enum Labels {
    LogicSender,
    LogicReceiver,
    UI,
}

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state(GameState::Loading)
            .add_plugin(LoadingPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(TitlePlugin)
            .add_plugin(InputPlugin)
            .add_plugin(InternalAudioPlugin)
            .add_plugin(TweenPlugin)
            .add_plugin(ButtonPlugin)
            .add_plugin(OrderPlugin)
            .add_plugin(CookingPlugin)
            .add_plugin(RestaurantPlugin)
            .add_plugin(ChefPlugin)
            .add_plugin(CustomerPlugin)
            .add_plugin(ScorePlugin)
            .add_plugin(GameOverPlugin)
            .add_system(component_animator_system::<TextureAtlasSprite>);
    }
}

pub fn spawn_sprite<'w, 's, 'a>(
    commands: &'a mut Commands<'w, 's>,
    handle: Handle<Image>,
    position: Vec3,
) -> EntityCommands<'w, 's, 'a> {
    commands
        .spawn_bundle(SpriteBundle {
            texture: handle,
            sprite: Sprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform::from_translation(position),
            ..Default::default()
        })
}