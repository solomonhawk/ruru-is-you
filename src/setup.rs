use bevy::{asset::ChangeWatcher, prelude::*};
use std::time::Duration;

use crate::{collision::Collider, config::GameConfig};

pub struct SetupPlugin;

impl Plugin for SetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(
            DefaultPlugins
                .set(
                    // prevent low res texture blurring
                    ImagePlugin::default_nearest(),
                )
                .set(AssetPlugin {
                    // Tell the asset server to watch for asset changes on disk:
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                }),
        )
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, config: Res<GameConfig>, asset_server: ResMut<AssetServer>) {
    // TODO: remove when map loading exists
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("subjects/ruru.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Collider,
    ));

    // TODO: remove when map loading exists
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("objects/you.png"),
            transform: Transform::from_xyz(-1. * config.grid_size, 0., 0.),
            ..default()
        },
        Collider,
    ));
}
