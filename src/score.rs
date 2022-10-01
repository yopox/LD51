use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Default)]
pub struct Score {
    score: i64,
    streak: i64,
}

impl Score {
    pub fn compute_on_failure (&mut self) {
        self.streak = 0
    }

    pub fn compute_on_success(&mut self, time: f64, difficulty: usize) {
        self.streak += 1;
        self.score += self.streak * (10 * (difficulty as i64 - 1) - (time.round() as i64));
        println!("Updated score: {} (with current streak: {})", self.score, self.streak);
    }
}