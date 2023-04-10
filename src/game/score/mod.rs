use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

pub mod resources;
mod systems;

use resources::HighScore;
use systems::{
    insert_score, remove_score, tick_score_timer, update_high_score, update_high_score_ui,
    update_score, update_score_ui,
};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HighScore>()
            .add_system(insert_score.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    update_high_score,
                    tick_score_timer,
                    update_score,
                    update_high_score_ui,
                    update_score_ui,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(remove_score.in_schedule(OnExit(AppState::Game)));
    }
}
