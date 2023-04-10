use bevy::prelude::*;

use crate::AppState;

mod systems;

use systems::{despawn_main_menu, spawn_main_menu};

mod components;

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_system(spawn_main_menu.in_schedule(OnEnter(AppState::MainMenu)))
            .add_system(despawn_main_menu.in_schedule(OnExit(AppState::MainMenu)));
    }
}
