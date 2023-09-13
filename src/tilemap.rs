use std::io::BufRead;
use std::{fs::File, io::BufReader};

use bevy::prelude::*;

use crate::player::{spawn_player, Player};
use crate::{collision::Collider, config::GameConfig};

pub struct TileMapPlugin;

impl Plugin for TileMapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, create_level);
    }
}

fn create_level(
    mut commands: Commands,
    config: Res<GameConfig>,
    mut asset_server: ResMut<AssetServer>,
    mut camera_query: Query<&mut Transform, (Without<Player>, With<Camera>)>,
) {
    let mut camera = camera_query.single_mut();
    let level_file = File::open("assets/levels/01.txt").expect("Unable to load level file");

    // XXX: ew
    let mut level_height = 0;
    let mut level_width = 0;

    for (y, line) in BufReader::new(level_file).lines().enumerate() {
        level_height = y;

        if let Ok(line) = line {
            for (x, c) in line.chars().enumerate() {
                level_width = x;

                if c == '.' {
                    continue;
                }

                let transform = Transform::from_xyz(
                    x as f32 * config.grid_size,
                    -1.0 * y as f32 * config.grid_size,
                    0.0,
                );

                match c {
                    '#' => spawn_wall_tile(&mut commands, &mut asset_server, transform),
                    '@' => spawn_player(&mut commands, &mut asset_server, transform),
                    'R' => spawn_ruru_tile(&mut commands, &mut asset_server, transform),
                    'Y' => spawn_you_tile(&mut commands, &mut asset_server, transform),
                    '∈' => spawn_is_tile(&mut commands, &mut asset_server, transform),
                    _ => warn!("Unknown map tile char {}", c),
                };
            }
        }
    }

    // XXX: pretty ugly way of "centering" the camera over the level
    camera.translation.x = (level_width / 2) as f32 * config.grid_size;
    camera.translation.y = (level_height / 2) as f32 * -config.grid_size;
}

fn spawn_wall_tile(commands: &mut Commands, asset_server: &mut AssetServer, transform: Transform) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("wall.png"),
            transform,
            ..default()
        },
        Collider,
    ));
}

fn spawn_ruru_tile(commands: &mut Commands, asset_server: &mut AssetServer, transform: Transform) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("subjects/ruru.png"),
            transform,
            ..default()
        },
        Collider,
    ));
}

fn spawn_you_tile(commands: &mut Commands, asset_server: &mut AssetServer, transform: Transform) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("objects/you.png"),
            transform,
            ..default()
        },
        Collider,
    ));
}

fn spawn_is_tile(commands: &mut Commands, asset_server: &mut AssetServer, transform: Transform) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("operators/is.png"),
            transform,
            ..default()
        },
        Collider,
    ));
}
