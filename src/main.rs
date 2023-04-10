use bevy::prelude::*;
use bevy::window::{PresentMode, WindowResolution};

mod game;
mod main_menu;

use game::GamePlugin;
use main_menu::MainMenuPlugin;

mod events;
mod systems;

use systems::{
    exit_game, handle_game_over, spawn_camera, transition_to_game_state,
    transition_to_main_menu_state,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(default_window_plugin()))
        .add_state::<AppState>()
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .insert_resource(ClearColor(Color::hex("111323").unwrap()))
        .add_startup_system(spawn_camera)
        .add_system(exit_game)
        .add_system(handle_game_over)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .run();
}

fn default_window_plugin() -> WindowPlugin {
    let window: Window = Window {
        present_mode: PresentMode::AutoVsync,
        position: WindowPosition::Centered(MonitorSelection::Primary),
        resolution: WindowResolution::new(450.0, 800.0),
        title: "Jumpr".to_string(),
        resizable: false,
        ..default()
    };
    let window_plugin: WindowPlugin = WindowPlugin {
        primary_window: Option::Some(window),
        ..default()
    };
    window_plugin
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}
