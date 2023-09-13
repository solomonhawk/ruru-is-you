mod camera;
mod collision;
mod config;
mod debug;
mod player;
mod setup;
mod tilemap;
mod update;

use bevy::prelude::*;

use camera::CameraPlugin;
use collision::CollisionPlugin;
use config::GameConfigPlugin;
use debug::DebugPlugin;
use player::PlayerPlugin;
use setup::SetupPlugin;
use tilemap::TileMapPlugin;
use update::UpdatePlugin;

fn main() {
    App::new()
        .add_plugins((
            GameConfigPlugin,
            SetupPlugin,
            CameraPlugin,
            DebugPlugin,
            PlayerPlugin,
            TileMapPlugin,
            CollisionPlugin,
            UpdatePlugin,
        ))
        .run();
}
