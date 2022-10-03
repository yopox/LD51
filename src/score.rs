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
            lives: 5,
        }
    }
}

pub const TIME_PER_INGREDIENT: f64 = 1.5;
pub const EXTRA_TIME_PER_BURGER: f64 = 3.;
const SCORING_SLOPE_POS: f64 = 1.;

impl Score {
    pub fn compute_on_failure(&mut self) {
        self.streak = 0;
        self.lives -= 1;
    }

    pub fn compute_on_success(&mut self, time: f64, difficulty: usize) {
        self.streak += 1;
        let time_performance =
            EXTRA_TIME_PER_BURGER + difficulty as f64 * TIME_PER_INGREDIENT - time;
        let score = SCORING_SLOPE_POS * self.streak as f64 * time_performance;
        self.score += score.round() as i64;
    }

    fn to_display_text(&self) -> String {
        format!("{}", self.score)
    }
}

#[derive(Component)]
struct ScoreUI;

#[derive(Component)]
pub struct LifeIcon(pub i32);

fn init_score(
    mut score: ResMut<Score>,
    mut commands: Commands,
    textures: Res<TextureAssets>,
    fonts: Res<FontAssets>,
) {
    let default_score = Score::default();
    score.score = default_score.score;
    score.streak = default_score.streak;
    score.lives = default_score.lives;

    commands
        .spawn_bundle(Text2dBundle {
            text: Text {
                sections: vec![TextSection {
                    value: score.to_display_text(),
                    style: TextStyle {
                        font: fonts.axg.clone(),
                        font_size: 16.0,
                        color: Color::BLACK,
                    },
                }],
                alignment: TextAlignment::BOTTOM_LEFT,
                ..Default::default()
            },
            transform: Transform::from_xyz(320. - 56., 156., 5.),
            ..Default::default()
        })
        .insert(ScoreUI);

    for i in 0..score.lives {
        commands
            .spawn_bundle(SpriteSheetBundle {
                sprite: Default::default(),
                texture_atlas: textures.life.clone(),
                transform: Transform::from_xyz(320. - 54. + 9. * i as f32, 156. - 19., 5.),
                ..Default::default()
            })
            .insert(LifeIcon(i))
            .insert(ScoreUI);
    }
}

fn update_score(score: Res<Score>, mut query: Query<&mut Text, With<ScoreUI>>) {
    for mut text in &mut query {
        text.sections[0].value = score.to_display_text();
    }
}

fn clean_score(mut commands: Commands, spawned_ui_components: Query<Entity, With<ScoreUI>>) {
    for e in spawned_ui_components.iter() {
        commands.entity(e).despawn_recursive()
    }
}
