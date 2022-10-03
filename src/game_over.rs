use bevy::prelude::*;

use crate::{GameState, spawn_sprite};
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
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
) {
    spawn_sprite(&mut commands, textures.background.clone(), Vec3::ZERO.clone()).insert(GameOverUi);
    spawn_sprite(&mut commands, textures.counter.clone(), Vec3::new(0., 0., 0.5)).insert(GameOverUi);
    spawn_sprite(&mut commands, textures.game_over.clone(), Vec3::new(160. - 136., 136. - 23., 1.)).insert(GameOverUi);

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: format!("Your score: {}", score.score),
                        style: TextStyle {
                            font: fonts.axg.clone(),
                            font_size: 16.0,
                            color: Color::WHITE,
                        },
                    },
                ],
                alignment: TextAlignment::CENTER,
            },
            transform: Transform::from_xyz(160.0, 96.0, 1.),
            ..Default::default()
        })
        .insert(GameOverUi);

    let buttons = [
        ('m', Vec2::new(124., 24.)),
        ('i', Vec2::new(124. + 20., 24.)),
        ('a', Vec2::new(124. + 20. * 2., 24.)),
        ('m', Vec2::new(124. + 20. * 3., 24.)),
    ];

    for (c, pos) in buttons {
        let (button, _) = spawn_button(&mut commands, pos, c, &textures, &fonts, false,);
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
