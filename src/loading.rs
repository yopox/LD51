use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

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
    #[asset(path = "fonts/Axones Gold.ttf")]
    pub axones_gold: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    // #[asset(path = "audio/flying.ogg")]
    // pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 12., tile_size_y = 16., columns = 2, rows = 1, padding_x = 0., padding_y = 0.))]
    #[asset(path = "buttons.png")]
    pub buttons: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 8., columns = 1, rows = 6, padding_x = 0., padding_y = 0.))]
    #[asset(path = "ingredients.png")]
    pub ingredients: Handle<TextureAtlas>,

    #[asset(path = "miam.png")]
    pub miam: Handle<Image>,
    #[asset(path = "title.png")]
    pub title: Handle<Image>,

    #[asset(path = "restaurant.png")]
    pub restaurant: Handle<Image>,
    #[asset(path = "bubble.png")]
    pub bubble: Handle<Image>,
    #[asset(path = "arrow.png")]
    pub arrow: Handle<Image>,
}
