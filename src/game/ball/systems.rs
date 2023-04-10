use bevy::prelude::*;
use bevy::window::PrimaryWindow;

use crate::events::GameOver;

use crate::game::score::resources::Score;

use super::components::Ball;
use super::resources::{BallSpawner, Health};

const BALL_MAX_SPEED_H: f32 = 1600.0;
const BALL_MAX_SPEED_V: f32 = 600.0;
const BALL_H_SPEED_MUL_ON_COL: f32 = 0.38;
const GRAVITY_FORCE: f32 = 1500.0;
const GRAVITY_VEC: Vec3 = Vec3 {
    x: 0.0,
    y: -1.0,
    z: 0.0,
};
const BALL_RADIUS: f32 = 26.0;
const BALL_DIAMETER: f32 = 2.0 * BALL_RADIUS;
const PUSH_RADIUS: f32 = 160.0;
const MAX_PUSH_FORCE: f32 = 3600.0;
const KICK_HELP_MUL: f32 = 5.0;

pub fn tick_ball_spawner(mut ball_spawner: ResMut<BallSpawner>, time: Res<Time>) {
    ball_spawner.timer.tick(time.delta());
}

pub fn spawn_ball_overtime(
    health: Res<Health>,
    ball_spawner: Res<BallSpawner>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    if health.points <= 0 {
        return;
    }
    if ball_spawner.timer.finished() {
        spawn_ball(window_query, commands, asset_server, audio);
        println!("Spawn!");
    }
}

pub fn reset_ball_spawner(mut ball_spawner: ResMut<BallSpawner>) {
    *ball_spawner = BallSpawner { ..default() };
}

pub fn restore_health(mut health: ResMut<Health>) {
    *health = Health { ..default() };
}

pub fn spawn_ball(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
) {
    let window: &Window = window_query.get_single().unwrap();
    let spawn_transform: Transform =
        Transform::from_xyz(window.width() / 2.0, window.height() * 1.5, 0.0);
    commands.spawn((
        Ball { ..default() },
        SpriteBundle {
            transform: spawn_transform,
            texture: asset_server.load("sprites/ball.png"),
            ..default()
        },
    ));
    audio.play(asset_server.load("audio/spawn.wav"));
    println!("A ball is spawned!");
}

pub fn despawn_balls(mut commands: Commands, ball_query: Query<Entity, With<Ball>>) {
    for ball in &ball_query {
        commands.entity(ball).despawn();
    }
}

pub fn ball_movement(
    mut ball_query: Query<(&mut Transform, &mut Ball)>,
    time: Res<Time>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut health: ResMut<Health>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    mut game_over_event_writer: EventWriter<GameOver>,
    score: Res<Score>,
) {
    if health.points <= 0 {
        return;
    }
    for (mut transform, mut ball) in &mut ball_query {
        ball.velocity += GRAVITY_FORCE * GRAVITY_VEC * time.delta_seconds();
        ball.velocity.x = ball.velocity.x.clamp(-BALL_MAX_SPEED_H, BALL_MAX_SPEED_H);
        ball.velocity.y = ball.velocity.y.clamp(-BALL_MAX_SPEED_V, BALL_MAX_SPEED_V);
        transform.translation += ball.velocity * time.delta_seconds();
        let window: &Window = window_query.get_single().unwrap();
        let window_h: f32 = window.height();
        let window_w: f32 = window.width();
        if transform.translation.y + BALL_RADIUS < 0.0 {
            transform.translation.y = window_h + BALL_DIAMETER;
            health.points -= 1;
            audio.play(asset_server.load("audio/drop.wav"));
            println!("New HP: {}", health.points);
            if health.points == 0 {
                game_over_event_writer.send(GameOver { score: score.value });
            }
        } else if (transform.translation.x - BALL_RADIUS <= 0.0 && ball.velocity.x < 0.0)
            || (transform.translation.x + BALL_RADIUS >= window_w && ball.velocity.x > 0.0)
        {
            ball.velocity.x = -ball.velocity.x * BALL_H_SPEED_MUL_ON_COL;
        }
    }
}

pub fn kick_ball(
    buttons: Res<Input<MouseButton>>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut ball_query: Query<(&Transform, &mut Ball)>,
    health: Res<Health>,
) {
    if !buttons.just_pressed(MouseButton::Left) {
        return;
    }
    audio.play(asset_server.load("audio/kick.wav"));
    if health.points <= 0 {
        return;
    }
    let window: &Window = window_query.get_single().unwrap();
    let cursor_pos = window.cursor_position().unwrap_or(Vec2::ZERO);
    let force_origin: Vec3 = Vec3 {
        x: cursor_pos.x,
        y: cursor_pos.y,
        z: 0.0,
    };
    for (transform, mut ball) in &mut ball_query {
        let dist: f32 = transform.translation.distance(force_origin);
        if dist > PUSH_RADIUS {
            continue;
        }
        let force_vec_n_x: f32 = (transform.translation - force_origin).normalize().x;
        let force_vec: Vec3 = Vec3 {
            x: force_vec_n_x,
            y: 1.0 * KICK_HELP_MUL,
            z: 0.0,
        }
        .normalize()
            * MAX_PUSH_FORCE
            * (1.0 - dist / PUSH_RADIUS);
        ball.velocity += force_vec;
    }
}
