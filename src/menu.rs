use bevy::{
    input::{keyboard::KeyboardInput, ButtonState},
    prelude::*,
};

use crate::state::GameState;

pub struct MenuPlugin;

#[derive(Component)]
struct StartTitle;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup)
            .add_systems(Update, keyboard_events)
            .add_systems(OnEnter(GameState::Menu), show_start_screen)
            .add_systems(OnExit(GameState::Menu), hide_start_screen);
    }
}

fn setup(mut commands: Commands, mut asset_server: ResMut<AssetServer>) {
    let font = asset_server.load("fonts/stay-pixel-font/StayPixelRegular-EaOxl.ttf");

    commands
        .spawn(NodeBundle {
            style: Style {
                // fill the entire window
                height: Val::Percent(100.),
                width: Val::Percent(100.),
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|builder| {
            builder.spawn(
                TextBundle::from_section(
                    "Press any key to start",
                    TextStyle {
                        font: font.clone(),
                        font_size: 30.0,
                        color: Color::rgba(1.0, 1.0, 1.0, 0.0),
                    },
                )
                .with_text_alignment(TextAlignment::Center),
            );
        });
}

fn show_start_screen(mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].style.color.set_a(1.0);
}

fn hide_start_screen(mut query: Query<&mut Text>) {
    let mut text = query.single_mut();
    text.sections[0].style.color.set_a(0.0);
}

fn keyboard_events(
    mut game_state: ResMut<NextState<GameState>>,
    mut key_evr: EventReader<KeyboardInput>,
) {
    for ev in key_evr.iter() {
        match ev.state {
            ButtonState::Released => {
                game_state.set(GameState::Playing);
            }
            _ => (),
        }
    }
}
