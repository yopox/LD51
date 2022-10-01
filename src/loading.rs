use crate::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_loading_state(
            LoadingState::new(GameState::Loading)
                .with_collection::<FontAssets>()
                .with_collection::<AudioAssets>()
                .with_collection::<TextureAssets>()
                .continue_to_state(GameState::Playing),
        );
    }
}

#[derive(AssetCollection)]
pub struct FontAssets {
    #[asset(path = "fonts/FiraSans-Bold.ttf")]
    pub fira_sans: Handle<Font>,
}

#[derive(AssetCollection)]
pub struct AudioAssets {
    // #[asset(path = "audio/flying.ogg")]
    // pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection)]
pub struct TextureAssets {
    #[asset(texture_atlas(tile_size_x = 24., tile_size_y = 16., columns = 2, rows = 4, padding_x = 0., padding_y = 0.))]
    #[asset(path = "buttons.png")]
    pub buttons: Handle<TextureAtlas>,
    #[asset(texture_atlas(tile_size_x = 48., tile_size_y = 8., columns = 1, rows = 6, padding_x = 0., padding_y = 0.))]
    #[asset(path = "ingredients.png")]
    pub ingredients: Handle<TextureAtlas>,
}
