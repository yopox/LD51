use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use crate::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .continue_to_state(GameState::TitleScreen),
        );
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/axg.ttf")]
    pub axg: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    #[asset(path = "audio/TITLE.ogg")]
    pub title: Handle<AudioSource>,
    #[asset(path = "audio/JEU.ogg")]
    pub classic: Handle<AudioSource>,
    #[asset(path = "audio/MADNESS.ogg")]
    pub madness: Handle<AudioSource>,
    #[asset(path = "audio/game over.ogg")]
    pub game_over: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 12., tile_size_y = 16., columns = 2, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "buttons.png")]
    pub buttons: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 16., columns = 1, rows = 16, padding_x = 0., padding_y = 0.))]
    #[asset(path = "ingredients.png")]
    pub ingredients: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 72., tile_size_y = 80., columns = 1, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "characters.png")]
    pub characters: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 8., tile_size_y = 8., columns = 2, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "life.png")]
    pub life: Handle<TextureAtlas>,

    #[asset(path = "background.png")]
    pub background: Handle<Image>,
    #[asset(path = "counter.png")]
    pub counter: Handle<Image>,
    #[asset(path = "plate.png")]
    pub plate: Handle<Image>,
    #[asset(path = "menu.png")]
    pub menu: Handle<Image>,
    #[asset(path = "bill.png")]
    pub bill: Handle<Image>,

    #[asset(path = "miam.png")]
    pub miam: Handle<Image>,

    #[asset(path = "bubble.png")]
    pub bubble: Handle<Image>,
    #[asset(path = "arrow.png")]
    pub arrow: Handle<Image>,
    #[asset(path = "chef.png")]
    pub chef: Handle<Image>,

    #[asset(path = "game_over.png")]
    pub game_over: Handle<Image>,
    #[asset(path = "bill2.png")]
    pub end_bill: Handle<Image>,
}
