use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::GameState;
use crate::input::Actions;
use crate::loading::{FontAssets, TextureAssets};

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(update_buttons);
    }
}

#[derive(Component)]
pub struct Letter {
    pub char: char,
}

pub fn spawn_button(
    commands: &mut Commands,
    position: Vec2,
    letter: char,
    textures: &Res<TextureAssets>,
    fonts: &Res<FontAssets>,
) -> Entity {
    commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.buttons.clone(),
            sprite: TextureAtlasSprite {
                anchor: Anchor::BottomLeft,
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(position.x, position.y, 2.),
                ..Default::default()
            },
            ..Default::default()
        })
        .with_children(|parent| {
            parent.spawn_bundle(Text2dBundle {
                text: Text {
                    sections: vec![TextSection {
                        value: letter.to_uppercase().to_string(),
                        style: TextStyle {
                            font: fonts.axones_gold.clone(),
                            font_size: 16.0,
                            ..Default::default()
                        },
                    }],
                    ..Default::default()
                },
                transform: Transform {
                    translation: Vec3::new(4., 13., 1.),
                    ..Default::default()
                },
                ..Default::default()
            });
        })
        .insert(Letter { char: letter })
        .id()
}

fn update_buttons(
    actions: Res<Actions>,
    mut buttons: Query<(&Letter, &mut TextureAtlasSprite, &Children)>,
    mut text: Query<&mut Text>,
) {
    for (letter, mut sprite, children) in buttons.iter_mut() {
        let pushed = actions.pressed.contains(&letter.char);
        let color: Color;
        match pushed {
            true => {
                sprite.index = 1;
                color = Color::rgb(182. / 255., 182. / 255., 182. / 255.);
            }
            false => {
                sprite.index = 0;
                color = Color::rgb(58. / 255., 58. / 255., 58. / 255.);
            }
        }
        let mut child_text = text.get_mut(*children.get(0).unwrap()).unwrap();
        child_text.sections.get_mut(0).unwrap().style.color = color;
    }
}
