#![allow(unused)]
use std::f32::consts::PI;
use std::time::Duration;

use bevy::prelude::*;
use bevy::render::render_resource::AddressMode;
use bevy::render::texture::CompressedImageFormats;
use bevy::render::texture::ImageSampler;
use bevy::render::texture::ImageType;
use bevy::sprite::collide_aabb::{collide, Collision};
use bevy::sprite::Anchor;
use bevy::sprite::MaterialMesh2dBundle;
use bevy::window::WindowResolution;
use bevy::{asset::ChangeWatcher, render::render_resource::SamplerDescriptor};

use bevy_inspector_egui::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use bevy_pixel_camera::{PixelCameraBundle, PixelCameraPlugin};

const FRAME_TIME: f32 = 1.0 / 120.0;
const GRID_SIZE: f32 = 16.0;

const SCREEN_WIDTH: i32 = 320;
const SCREEN_HEIGHT: i32 = 180;

#[derive(Reflect, Component)]
struct Player;

#[derive(Component)]
struct Collider;

#[derive(Event, Default)]
struct CollisionEvent;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(
                    // prevent low res texture blurring
                    ImagePlugin::default_nearest(),
                )
                .set(AssetPlugin {
                    // Tell the asset server to watch for asset changes on disk:
                    watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(200)),
                    ..default()
                })
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        mode: bevy::window::WindowMode::Windowed,
                        position: WindowPosition::Centered(MonitorSelection::Primary),
                        ..default()
                    }),
                    ..default()
                }),
            PixelCameraPlugin,
            WorldInspectorPlugin::new(),
        ))
        .register_type::<Player>()
        .add_event::<CollisionEvent>()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(FixedTime::new_from_secs(FRAME_TIME))
        .add_systems(Startup, setup)
        .add_systems(Update, (movement_system, bevy::window::close_on_esc))
        .add_systems(FixedUpdate, collision_system)
        .run();
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut images: ResMut<Assets<Image>>,
    asset_server: ResMut<AssetServer>,
) {
    commands.spawn(PixelCameraBundle::from_resolution(
        SCREEN_WIDTH,
        SCREEN_HEIGHT,
        true,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("player.png"),
            transform: Transform::from_xyz(0., -1. * GRID_SIZE, 0.),
            ..default()
        },
        Player,
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("subjects/ruru.png"),
            transform: Transform::from_xyz(0., 0., 0.),
            ..default()
        },
        Collider,
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("objects/you.png"),
            transform: Transform::from_xyz(-1. * GRID_SIZE, 0., 0.),
            ..default()
        },
        Collider,
    ));
}

fn movement_system(
    keys: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Transform), With<Player>>,
    timer: Res<Time>,
) {
    let mut transform = player_query.single_mut();

    if keys.just_pressed(KeyCode::Left) {
        transform.translation.x -= GRID_SIZE;
        transform.rotation = Quat::from_rotation_z(PI / 2.0);
    }
    if keys.just_pressed(KeyCode::Right) {
        transform.translation.x += GRID_SIZE;
        transform.rotation = Quat::from_rotation_z(PI * 1.5);
    }
    if keys.just_pressed(KeyCode::Up) {
        transform.translation.y += GRID_SIZE;
        transform.rotation = Quat::from_rotation_z(0.0);
    }
    if keys.just_pressed(KeyCode::Down) {
        transform.translation.y -= GRID_SIZE;
        transform.rotation = Quat::from_rotation_z(PI);
    }
}

fn collision_system(
    mut player_query: Query<(Entity, &Transform), With<Player>>,
    collider_query: Query<(Entity, &Transform), (With<Collider>, Without<Player>)>,
    mut collision_events: EventWriter<CollisionEvent>,
) {
    // let (payer_entity, player_transform) = player_query.single();
    // let player_size = player_transform.scale.truncate() * Vec2::splat(GRID_SIZE);

    // for (collider_entity, collider_transform) in &collider_query {
    //     let collision = collide(
    //         player_transform.translation,
    //         player_size,
    //         collider_transform.translation,
    //         collider_transform.scale.truncate() * Vec2::splat(GRID_SIZE),
    //     );

    //     if let Some(collision) = collision {
    //         collision_events.send_default();

    //         match collision {
    //             Collision::Inside => {
    //                 player_entity.
    //             }
    //             _ => ()
    //         }
    //         dbg!(player_size);
    //         dbg!(collision);
    //     }
    // }
}
