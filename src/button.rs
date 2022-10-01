use bevy::prelude::*;

use crate::GameState;
use crate::input::Actions;
use crate::loading::TextureAssets;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system_set(SystemSet::on_enter(GameState::Playing)
                    .with_system(spawn_button)
            )
            .add_system(update_buttons);
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
            texture_atlas: textures.buttons.clone(),
            transform: Transform {
                translation: Vec3::new(-32., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Letter { char: 'l' });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.buttons.clone(),
            transform: Transform {
                translation: Vec3::new(-8., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Letter { char: 'u' });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.buttons.clone(),
            transform: Transform {
                translation: Vec3::new(16., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Letter { char: 'f' });

    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.buttons.clone(),
            transform: Transform {
                translation: Vec3::new(40., 0., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(Letter { char: 'z' });
}

fn update_buttons(
    actions: Res<Actions>,
    mut buttons: Query<(&Letter, &mut TextureAtlasSprite)>,
) {
    let a_code = 'a' as usize;
    for (letter, mut sprite) in buttons.iter_mut() {
        let pushed = actions.pressed.contains(&letter.char);
        sprite.index = (letter.char as usize - a_code) * 2;
        if pushed { sprite.index += 1; }
    }
}