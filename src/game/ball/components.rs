use bevy::prelude::*;

#[derive(Component)]
pub struct Ball {
    pub velocity: Vec3,
}

impl Default for Ball {
    fn default() -> Self {
        Ball {
            velocity: Vec3 { ..default() },
        }
    }
}
