use bevy::prelude::*;

fn setup(mut commands: Commands, asset_server: ResMut<AssetServer>) {
    commands.spawn(Camera2dBundle::default());

    commands.spawn(SpriteBundle {
        texture: asset_server.load("subjects/ruru.png"),
        transform: Transform::from_xyz(100., 0., 0.),
        ..default()
    });
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            // prevent low res texture blurring
            ImagePlugin::default_nearest(),
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, bevy::window::close_on_esc)
        .run();
}
