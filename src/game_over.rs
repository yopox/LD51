use bevy::prelude::*;
use bevy_pkv::PkvStore;

use crate::{GameState, spawn_sprite};
use crate::audio::{BGM, PlayBgmEvent};
use crate::button::spawn_button;
use crate::input::{KeyboardReleaseEvent, process_input};
use crate::loading::{FontAssets, TextureAssets};
use crate::score::Score;

pub struct GameOverPlugin;

#[derive(Component)]
struct GameOverUi;

impl Plugin for GameOverPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(SystemSet::on_enter(GameState::GameOver).with_system(init_game_over))
            .add_system_set(
                SystemSet::on_update(GameState::GameOver)
                    .with_system(update_game_over)
                    .with_system(process_input),
            )
            .add_system_set(SystemSet::on_exit(GameState::GameOver).with_system(clean_game_over));
    }
}

fn init_game_over(
    score: Res<Score>,
    mut commands: Commands,
    mut bgm: EventWriter<PlayBgmEvent>,
    pkv: Res<PkvStore>,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
) {
    bgm.send(PlayBgmEvent(BGM::GameOver));

    spawn_sprite(&mut commands, textures.background.clone(), Vec3::ZERO.clone()).insert(GameOverUi);
    spawn_sprite(&mut commands, textures.counter.clone(), Vec3::new(0., 0., 0.5)).insert(GameOverUi);
    spawn_sprite(&mut commands, textures.game_over.clone(), Vec3::new(160. - 136., 136. - 23., 1.)).insert(GameOverUi);
    spawn_sprite(&mut commands, textures.end_bill.clone(), Vec3::new(160. - 64., -24., 2.)).insert(GameOverUi);

    let texts = [
        (format!("YOUR SCORE:"), Vec2::new(108.0, 77.0)),
        (score.score.to_string(), Vec2::new(108.0, 77.0 - 8. * 1.)),
        (format!("ALL-TIME BEST: (CLASSIC)"), Vec2::new(108.0, 77.0 - 8. * 3.)),
        (pkv.get::<String>("classic").unwrap_or("0".to_string()), Vec2::new(108.0, 77.0 - 8. * 4.)),
        (format!("ALL-TIME BEST: (MADNESS)"), Vec2::new(108.0, 77.0 - 8. * 6.)),
        (pkv.get::<String>("madness").unwrap_or("0".to_string()), Vec2::new(108.0, 77.0 - 8. * 7.)),
    ];

    for (text, pos) in texts {
        commands
            .spawn_bundle(Text2dBundle {
                text: Text {
                    sections: vec![
                        TextSection {
                            value: text,
                            style: TextStyle {
                                font: fonts.axg.clone(),
                                font_size: 16.0,
                                color: Color::BLACK,
                            },
                        },
                    ],
                    alignment: TextAlignment::BOTTOM_LEFT,
                },
                transform: Transform::from_xyz(pos.x, pos.y, 3.),
                ..Default::default()
            })
            .insert(GameOverUi);
    }

    let buttons = [
        ('m', Vec2::new(124., 8.)),
        ('i', Vec2::new(124. + 20., 8.)),
        ('a', Vec2::new(124. + 20. * 2., 8.)),
        ('m', Vec2::new(124. + 20. * 3., 8.)),
    ];

    for (c, pos) in buttons {
        let (button, _) = spawn_button(&mut commands, pos, c, &textures, &fonts, false);
        commands.entity(button).insert(GameOverUi);
    }
}

fn update_game_over(
    mut events: EventReader<KeyboardReleaseEvent>,
    mut state: ResMut<State<GameState>>,
) {
    for KeyboardReleaseEvent(l) in events.iter() {
        if *l == 'm' || *l == 'i' || *l == 'a' {
            state.set(GameState::TitleScreen).unwrap_or_default();
        }
    }
}

fn clean_game_over(
    mut commands: Commands,
    spawned_ui_elements: Query<Entity, With<GameOverUi>>,
) {
    for e in &spawned_ui_elements {
        commands.entity(e).despawn_recursive();
    }
}
