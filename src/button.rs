use bevy::prelude::*;
use bevy::sprite::Anchor;
use bevy_tweening::TweenCompleted;

use crate::input::Actions;
use crate::Labels;
use crate::loading::{FontAssets, TextureAssets};
use crate::tween::EV_ALLOW_BUTTON_UPDATE;

pub struct ButtonPlugin;

impl Plugin for ButtonPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_system(update_buttons.label(Labels::UI))
            .add_system(allow_button_update.label(Labels::UI));
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
    hidden: bool,
) -> (Entity, Entity) {
    let child = commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: letter.to_uppercase().to_string(),
                    style: TextStyle {
                        font: fonts.axg.clone(),
                        font_size: 16.0,
                        color: Color::rgba(1., 1., 1., if hidden { 0. } else { 1. }),
                        ..Default::default()
                    },
                }],
                ..Default::default()
            },
            transform: Transform::from_xyz(4., 13., 1.),
            ..Default::default()
        })
        .id();
    let button = commands
        .spawn_bundle(SpriteSheetBundle {
            texture_atlas: textures.buttons.clone(),
            sprite: TextureAtlasSprite {
                anchor: Anchor::BottomLeft,
                color: Color::rgba(1., 1., 1., if hidden { 0. } else { 1. }),
                ..Default::default()
            },
            transform: Transform::from_xyz(position.x, position.y, 2.),
            ..Default::default()
        })
        .add_child(child)
        .insert(Letter { char: letter })
        .id();
    (button, child)
}

#[derive(Component)]
pub struct PreventButtonUpdate;

fn allow_button_update(
    mut commands: Commands,
    mut tween_events: EventReader<TweenCompleted>,
) {
    for &TweenCompleted { entity, user_data } in tween_events.iter() {
        if user_data != EV_ALLOW_BUTTON_UPDATE { continue; }
        commands
            .entity(entity)
            .remove::<PreventButtonUpdate>();
    }
}

fn update_buttons(
    actions: Res<Actions>,
    mut buttons: Query<(&Letter, &mut TextureAtlasSprite, &Children), Without<PreventButtonUpdate>>,
) {
    for (letter, mut sprite, children) in buttons.iter_mut() {
        let pushed = actions.pressed.contains(&letter.char);
        sprite.index = if pushed { 1 } else { 0 };
    }
}
