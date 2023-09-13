mod camera;
mod collision;
mod config;
mod debug;
mod player;
mod setup;
mod update;

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins((
            config::GameConfigPlugin,
            setup::SetupPlugin,
            camera::CameraPlugin,
            debug::DebugPlugin,
            player::PlayerPlugin,
            collision::CollisionPlugin,
            update::UpdatePlugin,
        ))
        .run();
}
