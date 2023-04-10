use bevy::prelude::*;

use crate::main_menu::components::MainMenu;

pub fn spawn_main_menu(mut commands: Commands) {
    let _main_menu_entity: Entity = build_main_menu(&mut commands);
}

pub fn despawn_main_menu(mut commands: Commands, main_menu_query: Query<Entity, With<MainMenu>>) {
    if let Ok(main_menu_entity) = main_menu_query.get_single() {
        commands.entity(main_menu_entity).despawn_recursive();
    }
}

fn build_main_menu(commands: &mut Commands) -> Entity {
    let main_menu_entity: Entity = commands
        .spawn((
            NodeBundle {
                style: Style {
                    size: Size::new(Val::Percent(100.0), Val::Percent(100.0)),
                    ..default()
                },
                background_color: Color::hex("374566").unwrap().into(),
                ..default()
            },
            MainMenu {},
        ))
        .id();
    main_menu_entity
}
