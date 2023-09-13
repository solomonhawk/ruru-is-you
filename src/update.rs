use bevy::prelude::*;

use crate::{
    collision::collision_system, config::GameConfig, player::movement_system, state::GameState,
};

pub struct UpdatePlugin;

impl Plugin for UpdatePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(
                FixedUpdate,
                (movement_system, collision_system.after(movement_system))
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(Update, bevy::window::close_on_esc);
    }
}

fn setup(mut commands: Commands, config: Res<GameConfig>) {
    commands.insert_resource(FixedTime::new_from_secs(config.frame_time));
}
