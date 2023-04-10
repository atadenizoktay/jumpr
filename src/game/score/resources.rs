use bevy::prelude::*;

#[derive(Resource)]
pub struct HighScore {
    pub value: u64,
}

impl Default for HighScore {
    fn default() -> Self {
        HighScore { value: 0 }
    }
}

#[derive(Resource)]
pub struct Score {
    pub timer: Timer,
    pub value: u64,
}

impl Default for Score {
    fn default() -> Self {
        Score {
            timer: Timer::from_seconds(1.0, TimerMode::Repeating),
            value: 0,
        }
    }
}
