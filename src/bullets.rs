use crate::{invaders, Bullet, Collider, Invader, Velocity};
use bevy::prelude::*;

const BULLET_SPRITE_PATH: &str = "bullet.png";
const PLAYER_BULLET_SPEED: f32 = 500.0;

#[derive(Bundle)]
pub(crate) struct BulletBundle {
    sprite_bundle: SpriteBundle,
    bullet: Bullet,
    velocity: Velocity,
}

pub(crate) fn spawn_player_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    bullet_transform: &Transform,
) {
    commands.spawn(BulletBundle {
        sprite_bundle: SpriteBundle {
            texture: asset_server.load(BULLET_SPRITE_PATH),
            transform: Transform {
                translation: bullet_transform.translation,
                ..default()
            },
            ..default()
        },
        bullet: Bullet,
        velocity: Velocity(Vec3::new(0.0, PLAYER_BULLET_SPEED, 0.0)),
    });
}

pub(crate) fn move_bullets(
    mut bullet_query: Query<(&mut Transform, &Velocity), With<Bullet>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in bullet_query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

pub(crate) fn check_bullet_collider_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    collider_query: Query<&Transform, With<Collider>>,
) {
    for (bullet, bullet_transform) in bullet_query.iter() {
        for collider_transform in collider_query.iter() {
            if bevy::sprite::collide_aabb::collide(
                bullet_transform.translation,
                bullet_transform.scale.truncate(),
                collider_transform.translation,
                collider_transform.scale.truncate(),
            )
            .is_some()
            {
                commands.entity(bullet).despawn();
            }
        }
    }
}

pub(crate) fn check_bullet_invader_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
    invader_query: Query<(Entity, &Transform), With<Invader>>,
) {
    for (bullet, bullet_transform) in bullet_query.iter() {
        for (invader, invader_transform) in invader_query.iter() {
            if bullet_transform
                .translation
                .distance(invader_transform.translation)
                < invaders::INVADER_SIZE
            {
                commands.entity(bullet).despawn();
                commands.entity(invader).despawn();
            }
        }
    }
}
