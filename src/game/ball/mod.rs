use bevy::prelude::*;

use crate::AppState;

use super::SimulationState;

pub mod components;
pub mod resources;
mod systems;

use resources::{BallSpawner, Health};
use systems::{
    ball_movement, despawn_balls, kick_ball, reset_ball_spawner, restore_health, spawn_ball,
    spawn_ball_overtime, tick_ball_spawner,
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BallSpawner>()
            .init_resource::<Health>()
            .add_system(reset_ball_spawner.in_schedule(OnEnter(AppState::Game)))
            .add_system(restore_health.in_schedule(OnEnter(AppState::Game)))
            .add_system(spawn_ball.in_schedule(OnEnter(AppState::Game)))
            .add_systems(
                (
                    ball_movement,
                    kick_ball,
                    spawn_ball_overtime,
                    tick_ball_spawner,
                )
                    .in_set(OnUpdate(AppState::Game))
                    .in_set(OnUpdate(SimulationState::Running)),
            )
            .add_system(despawn_balls.in_schedule(OnExit(AppState::Game)));
    }
}
