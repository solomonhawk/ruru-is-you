use bevy::prelude::*;

#[derive(Resource, Reflect)]
pub struct GameConfig {
    pub frame_time: f32,
    pub grid_size: f32,
    pub screen_width: i32,
    pub screen_height: i32,
}

pub struct GameConfigPlugin;

impl Plugin for GameConfigPlugin {
    fn build(&self, app: &mut App) {
        app.register_type::<GameConfig>()
            .insert_resource(GameConfig {
                frame_time: 1.0 / 120.0,
                grid_size: 16.0,
                screen_width: 320,
                screen_height: 180,
            });
    }
}
