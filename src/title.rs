use std::time::Duration;

use bevy::prelude::*;
use bevy_kira_audio::{AudioChannel, AudioControl};
use bevy_tweening::{Animator, EaseFunction, Tween, TweeningType};
use bevy_tweening::lens::TransformPositionLens;

use crate::{GameState, spawn_sprite};
use crate::audio::{BGM, BgmChannel, PlayBgmEvent, SfxChannel};
use crate::button::spawn_button;
use crate::cooking::MadnessMode;
use crate::input::{Actions, KeyboardReleaseEvent};
use crate::loading::{FontAssets, TextureAssets};
use crate::tween::{tween_position, tween_text_opacity, TWEEN_TIME};

pub struct TitlePlugin;

#[derive(Component)]
struct TitleUi;

#[derive(Component)]
struct TitleBurgerIngredient(usize);

struct TitleState {
    burger_open: bool,
}

impl Default for TitleState {
    fn default() -> Self {
        TitleState { burger_open: false }
    }
}

/// This plugin is responsible for the game menu (containing only one button...)
/// The menu is only drawn during the State `GameState::Menu` and is removed when that state is exited
impl Plugin for TitlePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<TitleState>()
            .add_system_set(SystemSet::on_enter(GameState::TitleScreen).with_system(setup_title))
            .add_system_set(SystemSet::on_update(GameState::TitleScreen).with_system(handle_input))
            .add_system_set(SystemSet::on_exit(GameState::TitleScreen).with_system(cleanup_title));
    }
}

fn setup_title(
    mut commands: Commands,
    mut bgm: EventWriter<PlayBgmEvent>,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
    mut state: ResMut<TitleState>,
) {
    bgm.send(PlayBgmEvent(BGM::Title));
    state.burger_open = false;

    spawn_sprite(&mut commands, textures.background.clone(), Vec3::ZERO.clone()).insert(TitleUi);
    spawn_sprite(&mut commands, textures.counter.clone(), Vec3::new(0., 0., 0.5,)).insert(TitleUi);
    spawn_sprite(&mut commands, textures.plate.clone(), Vec3::new(124., 30., 0.75,)).insert(TitleUi);
    spawn_sprite(&mut commands, textures.miam.clone(), Vec3::new(96., 127., 1.)).insert(TitleUi)
        .insert(Animator::new(Tween::new(
            EaseFunction::QuadraticInOut,
            TweeningType::PingPong,
            Duration::from_secs(3),
            TransformPositionLens {
                start: Vec3::new(97., 127., 1.),
                end: Vec3::new(97., 117., 1.),
            },
        )));

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: "Press SPACE".to_string(),
                    style: TextStyle {
                        font: fonts.axg.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                }],
                alignment: TextAlignment::CENTER,
                ..Default::default()
            },
            transform: Transform::from_xyz(170., 16., 1.),
            ..Default::default()
        })
        .insert(TitleUi);

    let (button_entity, _) = spawn_button(&mut commands, Vec2::new(124., 12.), ' ', &textures, &fonts, false);
    commands
        .entity(button_entity)
        .insert(TitleUi);

    let burger: Vec<usize> = vec![0, 2, 4, 1];
    for i in 0..burger.len() {
        commands
            .spawn_bundle(SpriteSheetBundle {
                texture_atlas: textures.ingredients.clone(),
                sprite: TextureAtlasSprite {
                    index: *burger.get(i).unwrap(),
                    ..Default::default()
                },
                transform: Transform::from_xyz(160., 36. + 8. * i as f32, 3.),
                ..Default::default()
            })
            .insert(TitleBurgerIngredient(i))
            .insert(TitleUi);
    }
}

fn handle_input(
    mut commands: Commands,
    mut title_state: ResMut<TitleState>,
    mut state: ResMut<State<GameState>>,
    input: Res<Actions>,
    mut events: EventReader<KeyboardReleaseEvent>,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
    bgm: Res<AudioChannel<BgmChannel>>,
    sfx: Res<AudioChannel<SfxChannel>>,
    ingredients: Query<(Entity, &Transform, &TitleBurgerIngredient)>,
) {
    if !title_state.burger_open && input.pressed.contains(&' ') {
        title_state.burger_open = true;

        // Spread burger
        for (entity, transform, TitleBurgerIngredient(n)) in ingredients.iter() {
            let initial_pos = transform.translation.clone();
            let end_pos = Vec3::new(initial_pos.x, initial_pos.y + *n as f32 * 12., initial_pos.z);
            commands
                .entity(entity)
                .insert(Animator::new(Tween::new(
                    EaseFunction::CubicOut,
                    TweeningType::Once,
                    Duration::from_secs_f32(1.5),
                    TransformPositionLens {
                        start: initial_pos,
                        end: end_pos,
                    },
                )));
        }

        // Spawn options & buttons
        let options = vec![
            ('c', "classic", 86.),
            ('d', "madness", 66.),
            ('u', "music", 46.),
        ];

        for (letter, name, y_pos) in options {
            let y_start = 40.;
            let (button, _) = spawn_button(&mut commands, Vec2::new(160., y_start), letter, &textures, &fonts, false);
            commands
                .entity(button)
                .insert(TitleUi)
                .insert(Animator::new(
                    tween_position(Vec2::new(160., y_start), Vec2::new(118., y_pos), 2., TWEEN_TIME * 3)
                    ));
            commands
                .spawn_bundle(Text2dBundle {
                    text: Text {
                        sections: vec![TextSection {
                            value: name.to_string(),
                            style: TextStyle {
                                font: fonts.axg.clone(),
                                font_size: 16.0,
                                color: Color::rgba(1., 1., 1., 0.),
                            },
                        }],
                        alignment: TextAlignment::CENTER,
                        ..Default::default()
                    },
                    transform: Transform::from_xyz(160., (46. + y_pos) / 2. - 7., 1.),
                    ..Default::default()
                })
                .insert(TitleUi)
                .insert(Animator::new(
                    tween_text_opacity(Color::WHITE, TWEEN_TIME * 3, true)
                ))
                .insert(Animator::new(
                    tween_position(Vec2::new(160., (46. + y_pos) / 2. - 7.), Vec2::new(160., y_pos + 5.), 2., TWEEN_TIME * 3)
                ));
        }
    }

    for KeyboardReleaseEvent(char) in events.iter() {
        match *char {
            'c' => {
                commands.insert_resource(MadnessMode(false));
                state.set(GameState::Cooking).unwrap();
            }
            'd' => {
                commands.insert_resource(MadnessMode(true));
                state.set(GameState::Cooking).unwrap();
            }
            'u' => {
                match bgm.is_playing_sound() {
                    true => { bgm.pause(); sfx.pause(); }
                    false => { bgm.resume(); sfx.resume(); }
                }
            }
            _ => {}
        }
    }
}

fn cleanup_title(
    mut commands: Commands,
    entities: Query<Entity, With<TitleUi>>
) {
    for entity in entities.iter() {
        commands.entity(entity).despawn_recursive();
    }
}
