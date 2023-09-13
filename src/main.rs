#[macro_use]
extern crate lazy_static;

mod camera;
mod collision;
mod config;
mod debug;
mod menu;
mod player;
mod setup;
mod state;
mod tilemap;
mod update;

use bevy::prelude::*;

use camera::CameraPlugin;
use collision::CollisionPlugin;
use config::GameConfigPlugin;
use debug::DebugPlugin;
use menu::MenuPlugin;
use player::PlayerPlugin;
use setup::SetupPlugin;
use state::GameStatePlugin;
use tilemap::TileMapPlugin;
use update::UpdatePlugin;

fn main() {
    App::new()
        .add_plugins((
            GameConfigPlugin,
            GameStatePlugin,
            MenuPlugin,
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
