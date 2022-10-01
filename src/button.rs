use std::any::{Any, TypeId};
use bevy::prelude::*;
use crate::GameState;
use crate::loading::TextureAssets;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_button)
            );
    }
}

#[derive(Component)]
pub struct Letter {
    pub char: char
}

fn spawn_button(
    mut commands: Commands,
    textures: Res<TextureAssets>
) {
    commands
        .spawn_bundle(SpriteSheetBundle {
            sprite: TextureAtlasSprite {
                index: 0,
                ..Default::default()
            },
            texture_atlas: textures.buttons.clone(),
            transform: Default::default(),
            ..Default::default()
        })
        .insert(Letter { char: 'a' });
}