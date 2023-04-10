use bevy::prelude::*;

use crate::AppState;

use crate::events::GameOver;

mod ball;
mod score;
use ball::BallPlugin;
use score::ScorePlugin;

mod systems;
use systems::toggle_simulation;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<SimulationState>()
            .add_event::<GameOver>()
            .add_plugin(BallPlugin)
            .add_plugin(ScorePlugin)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    #[default]
    Running,
    Paused,
}
