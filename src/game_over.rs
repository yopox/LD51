use bevy::prelude::*;

use crate::button::{Letter, spawn_button};
use crate::GameState;
use crate::input::{KeyboardReleaseEvent, process_input};
use crate::loading::{FontAssets, TextureAssets};
use crate::score::Score;

pub struct GameOverPlugin;

#[derive(Component)]
struct GameOverUI;

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
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![
                    TextSection {
                        value: "Game Over\n".to_string(),
                        style: TextStyle {
                            font: fonts.axg.clone(),
                            font_size: 64.0,
                            color: Color::WHITE,
                        },
                    },
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
            transform: Transform::from_xyz(160.0, 132.0, 1.),
            ..Default::default()
        })
        .insert(GameOverUI);

    spawn_button(
        &mut commands,
        Vec2::new(154.0, 16.0),
        'm',
        &textures,
        &fonts,
    );
}

fn update_game_over(
    mut events: EventReader<KeyboardReleaseEvent>,
    mut state: ResMut<State<GameState>>,
) {
    for KeyboardReleaseEvent(l) in events.iter() {
        if *l == 'm' {
            state.set(GameState::TitleScreen).unwrap_or_default();
        }
    }
}

fn clean_game_over(
    mut commands: Commands,
    spawned_ui_elements: Query<Entity, With<GameOverUI>>,
    buttons: Query<Entity, With<Letter>>,
) {
    for e in &spawned_ui_elements {
        commands.entity(e).despawn_recursive();
    }
    for e in buttons.iter() {
        commands.entity(e).despawn_recursive();
    }
}
