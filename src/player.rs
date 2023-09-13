use bevy::prelude::*;
use std::f32::consts::PI;

use crate::{collision::Collider, config::GameConfig};

#[derive(Reflect, Component)]
pub struct Player;

#[derive(Debug, Component)]
pub struct Dir(pub FacingDirection);

#[derive(Debug)]
pub enum FacingDirection {
    Up,
    Right,
    Down,
    Left,
}

#[derive(Resource)]
pub struct MoveTimer {
    timer: Timer,
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(MoveTimer {
            timer: Timer::from_seconds(0.2, TimerMode::Once),
        });
    }
}

pub fn spawn_player(commands: &mut Commands, asset_server: &mut AssetServer, transform: Transform) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform,
            ..default()
        },
        Player,
        Dir(FacingDirection::Up), // XXX: this seems like a hack...
        Collider,
    ));
}

pub fn movement_system(
    keys: Res<Input<KeyCode>>,
    mut move_timer: ResMut<MoveTimer>,
    mut player_query: Query<(&mut Transform, &mut Dir), With<Player>>,
    time: Res<Time>,
    config: Res<GameConfig>,
) {
    if move_timer.timer.finished() {
        let (mut transform, mut dir) = player_query.single_mut();

        if keys.pressed(KeyCode::Left) {
            transform.translation.x -= config.grid_size;
            transform.rotation = Quat::from_rotation_z(PI / 2.0);
            dir.0 = FacingDirection::Left;
        }
        if keys.pressed(KeyCode::Right) {
            transform.translation.x += config.grid_size;
            transform.rotation = Quat::from_rotation_z(PI * 1.5);
            dir.0 = FacingDirection::Right;
        }
        if keys.pressed(KeyCode::Up) {
            transform.translation.y += config.grid_size;
            transform.rotation = Quat::from_rotation_z(0.0);
            dir.0 = FacingDirection::Up;
        }
        if keys.pressed(KeyCode::Down) {
            transform.translation.y -= config.grid_size;
            transform.rotation = Quat::from_rotation_z(PI);
            dir.0 = FacingDirection::Down;
        }
        move_timer.timer.reset();
    } else {
        move_timer.timer.tick(time.delta());
    }
}
