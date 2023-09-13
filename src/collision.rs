use bevy::{prelude::*, sprite::collide_aabb::collide};

use crate::{
    config::GameConfig,
    player::{Dir, FacingDirection, Player},
};

#[derive(Component)]
pub struct Collider;

#[derive(Event, Default)]
pub struct CollisionEvent;

pub struct CollisionPlugin;

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<CollisionEvent>();
    }
}

pub fn collision_system(
    mut player_query: Query<(Entity, &mut Transform, &Dir), With<Player>>,
    collider_query: Query<(Entity, &Transform), (With<Collider>, Without<Player>)>,
    mut collision_events: EventWriter<CollisionEvent>,
    config: Res<GameConfig>,
) {
    let (_payer_entity, mut player_transform, dir) = player_query.single_mut();
    let player_size = player_transform.scale.truncate() * Vec2::splat(config.grid_size);

    for (_collider_entity, collider_transform) in &collider_query {
        let collision = collide(
            player_transform.translation,
            player_size,
            collider_transform.translation,
            collider_transform.scale.truncate() * Vec2::splat(config.grid_size),
        );

        if let Some(_collision) = collision {
            collision_events.send_default();

            let offset: Vec3 = match dir.0 {
                FacingDirection::Up => [0.0, -1.0, 0.0].into(),
                FacingDirection::Right => [-1.0, 0.0, 0.0].into(),
                FacingDirection::Down => [0.0, 1.0, 0.0].into(),
                FacingDirection::Left => [1.0, 0.0, 0.0].into(),
            };

            // XXX: is this bad? it feels kind of bad
            // undo the movement that would have caused the collision
            player_transform.translation += offset * Vec3::splat(config.grid_size);
        }
    }
}
