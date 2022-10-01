use bevy::prelude::*;

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>();
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
            lives: 3
        }
    }
}

impl Score {
    pub fn compute_on_failure (&mut self) {
        self.streak = 0;
        self.lives -=1;
    }

    pub fn compute_on_success(&mut self, time: f64, difficulty: usize) {
        self.streak += 1;
        // TODO pb : streak should not inflate your score this time if it negative
        self.score += self.streak * (5 * (difficulty as i64 - 1) - (time.round() as i64));
        println!("Updated score: {} (with current streak: {})", self.score, self.streak);
    }
}