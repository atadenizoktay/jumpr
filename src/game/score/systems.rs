use bevy::prelude::*;

use crate::events::GameOver;

use crate::game::ball::resources::Health;

use super::resources::{HighScore, Score};

pub fn insert_score(mut commands: Commands) {
    commands.insert_resource(Score::default());
}

pub fn remove_score(mut commands: Commands) {
    commands.remove_resource::<Score>();
}

pub fn tick_score_timer(mut score: ResMut<Score>, time: Res<Time>) {
    score.timer.tick(time.delta());
}

pub fn update_high_score(
    mut game_over_event_reader: EventReader<GameOver>,
    mut high_score: ResMut<HighScore>,
) {
    for event in game_over_event_reader.iter() {
        if event.score > high_score.value {
            high_score.value = event.score;
        }
    }
}

pub fn update_high_score_ui(high_score: Res<HighScore>) {
    if high_score.is_changed() {
        println!("New High Score: {}", high_score.value);
    }
}

pub fn update_score(mut score: ResMut<Score>, health: Res<Health>) {
    if health.points <= 0 {
        return;
    }
    if score.timer.finished() {
        score.value += 1;
        println!("Score: {}", score.value)
    }
}

pub fn update_score_ui(score: Res<Score>) {
    if score.is_changed() {
        return;
    }
}
