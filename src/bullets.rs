use bevy::prelude::*;

use crate::{Bullet, Collider, Velocity};

const BULLET_SPRITE_PATH: &str = "bullet.png";
const PLAYER_BULLET_SPEED: f32 = 500.0;

#[derive(Bundle)]
pub(crate) struct BulletBundle {
    sprite_bundle: SpriteBundle,
    collider: Collider,
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
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..default()
        },
        bullet: Bullet,
        velocity: Velocity(Vec3::new(0.0, PLAYER_BULLET_SPEED, 0.0)),
        collider: Collider,
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
