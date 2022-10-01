use bevy::prelude::*;

use crate::GameState;
use crate::loading::{FontAssets, TextureAssets};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_system_set(SystemSet::on_enter(GameState::Cooking).with_system(init_score))
            .add_system_set(SystemSet::on_update(GameState::Cooking).with_system(update_score))
            .add_system_set(SystemSet::on_exit(GameState::Cooking).with_system(clean_score));
    }
}

#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Score {
    pub score: i64,
    pub streak: i64,
    pub lives: i32,
}

impl Default for Score {
    fn default() -> Self {
        Self {
            score: 0,
            streak: 0,
            lives: 3,
        }
    }
}

impl Score {
    pub fn compute_on_failure(&mut self) {
        self.streak = 0;
        self.lives -= 1;
    }

    pub fn compute_on_success(&mut self, time: f64, difficulty: usize) {
        self.streak += 1;
        // TODO pb : streak should not inflate your score this time if it negative
        self.score += self.streak * (5 * (difficulty as i64 - 1) - (time.round() as i64));
    }

    fn to_display_text(&self) -> String {
        format!("Score {}", self.score)
    }
}

#[derive(Component)]
struct ScoreUI;

fn init_score(score: Res<Score>, mut commands: Commands, fonts: Res<FontAssets>) {
    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: score.to_display_text(),
                    style: TextStyle {
                        font: fonts.axones_gold.clone(),
                        font_size: 16.0,
                        color: Color::WHITE,
                    },
                }],
                ..Default::default()
            },
            transform: Transform {
                translation: Vec3::new(256., 16., 1.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(ScoreUI);
}

fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreUI>>) {
    for mut text in &mut query {
        text.sections[0].value = score.to_display_text();
    }
}

fn clean_score(
    mut commands: Commands,
    spawned_ui_components: Query<Entity, With<ScoreUI>>,
) {
    for e in spawned_ui_components.iter() {
        commands.entity(e).despawn_recursive()
    }
}