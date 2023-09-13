use bevy::prelude::*;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

use crate::config::GameConfig;

pub struct CameraPlugin;

impl Plugin for CameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PixelCameraPlugin)
            .add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, config: Res<GameConfig>) {
    commands.spawn(PixelCameraBundle::from_resolution(
        config.screen_width,
        config.screen_height,
        true,
    ));
}
