use std::time::Duration;

use bevy::asset::ChangeWatcher;
use bevy::prelude::*;

const DELTA_TIME: f32 = 1.0 / 24.0;
const GRID_SIZE: f32 = 16.0;
const SCALE_FACTOR: f32 = 3.0;

#[derive(Component)]
struct MovementController;

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("subjects/ruru.png"),
            transform: Transform::from_xyz(0., 0., 0.).with_scale(Vec3::splat(SCALE_FACTOR)),
            ..default()
        },
        MovementController,
    ));
}

fn main() {
    App::new()
        .add_plugins(
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
        .insert_resource(FixedTime::new_from_secs(DELTA_TIME))
        .add_systems(Startup, setup)
        .add_systems(Update, (bevy::window::close_on_esc))
        .add_systems(FixedUpdate, (movement_system))
        .run();
}

fn movement_system(
    keys: Res<Input<KeyCode>>,
    mut player: Query<&mut Transform, With<MovementController>>,
) {
    let mut p = player.single_mut();
    let dist = SCALE_FACTOR * GRID_SIZE;

    if keys.pressed(KeyCode::Left) {
        p.translation.x -= dist;
    }
    if keys.pressed(KeyCode::Right) {
        p.translation.x += dist;
    }
    if keys.pressed(KeyCode::Up) {
        p.translation.y += dist;
    }
    if keys.pressed(KeyCode::Down) {
        p.translation.y -= dist;
    }
}
