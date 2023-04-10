use bevy::prelude::*;

const NEW_BALL_SPAWN_TIME: f32 = 5.0;

#[derive(Resource)]
pub struct BallSpawner {
    pub timer: Timer,
}

impl Default for BallSpawner {
    fn default() -> Self {
        BallSpawner {
            timer: Timer::from_seconds(NEW_BALL_SPAWN_TIME, TimerMode::Repeating),
        }
    }
}

#[derive(Resource)]
pub struct Health {
    pub points: u64,
}

impl Default for Health {
    fn default() -> Self {
        Health { points: 3 }
    }
}
