use crate::{
    Collider, CommonBullet, Invader, InvaderBullet, InvaderBulletFiredEvent,
    InvaderBulletHitPlayerEvent, Player, PlayerBullet, PlayerBulletFiredEvent, ShootSound,
    Velocity, INVADER_SIZE, PLAYER_HEIGHT,
};
use bevy::prelude::*;
use rand::Rng;

const BULLET_SPRITE_PATH: &str = "player-bullet.png";
const PLAYER_BULLET_SPEED: f32 = 500.0;

#[derive(Bundle)]
pub(crate) struct PlayerBulletBundle {
    sprite_bundle: SpriteBundle,
    common_bullet: CommonBullet,
    bullet: PlayerBullet,
    velocity: Velocity,
}

#[derive(Bundle)]
pub(crate) struct InvaderBulletBundle {
    sprite_bundle: SpriteBundle,
    common_bullet: CommonBullet,
    bullet: InvaderBullet,
    velocity: Velocity,
}

pub(crate) fn spawn_player_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut player_bullet_fired_event: EventReader<PlayerBulletFiredEvent>,
) {
    for event in player_bullet_fired_event.read() {
        commands.spawn(PlayerBulletBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(BULLET_SPRITE_PATH),
                transform: Transform {
                    translation: event.0,
                    ..default()
                },
                ..default()
            },
            bullet: PlayerBullet,
            common_bullet: CommonBullet,
            velocity: Velocity(Vec3::new(0.0, PLAYER_BULLET_SPEED, 0.0)),
        });
    }
}

pub(crate) fn spawn_invader_bullet(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut invader_bullet_fired_event: EventReader<InvaderBulletFiredEvent>,
) {
    for event in invader_bullet_fired_event.read() {
        commands.spawn(InvaderBulletBundle {
            sprite_bundle: SpriteBundle {
                texture: asset_server.load(event.invader_difficulty.get_bullet_sprite_path()),
                transform: Transform {
                    translation: event.position,
                    ..default()
                },
                ..default()
            },
            bullet: InvaderBullet,
            common_bullet: CommonBullet,
            velocity: Velocity(Vec3::new(
                0.0,
                -event.invader_difficulty.get_bullet_speed(),
                0.0,
            )),
        });
    }
}

pub(crate) fn player_bullet_sound(
    mut commands: Commands,
    shoot_sound: Res<ShootSound>,
    mut player_bullet_fired_event: EventReader<PlayerBulletFiredEvent>,
) {
    for _event in player_bullet_fired_event.read() {
        commands.spawn(AudioBundle {
            source: shoot_sound.0.clone(),
            settings: PlaybackSettings {
                mode: bevy::audio::PlaybackMode::Despawn,
                speed: 2.0,
                volume: bevy::audio::Volume::new_relative(
                    0.1 + rand::thread_rng().gen_range(0.1..0.3),
                ),
                ..default()
            },
        });
    }
}

pub(crate) fn move_bullets(
    mut bullet_query: Query<(&mut Transform, &Velocity), With<CommonBullet>>,
    time: Res<Time>,
) {
    for (mut transform, velocity) in bullet_query.iter_mut() {
        transform.translation += velocity.0 * time.delta_seconds();
    }
}

pub(crate) fn check_bullet_wall_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<CommonBullet>>,
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

pub(crate) fn check_player_bullet_invader_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<PlayerBullet>>,
    invader_query: Query<(Entity, &Transform), With<Invader>>,
) {
    for (bullet, bullet_transform) in bullet_query.iter() {
        for (invader, invader_transform) in invader_query.iter() {
            if bullet_transform
                .translation
                .distance(invader_transform.translation)
                < INVADER_SIZE
            {
                commands.entity(bullet).despawn();
                commands.entity(invader).despawn();
            }
        }
    }
}

pub(crate) fn check_invader_bullet_player_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<InvaderBullet>>,
    player_query: Query<&Transform, With<Player>>,
    mut invader_hit_player_event: EventWriter<InvaderBulletHitPlayerEvent>,
) {
    let player_transform = player_query.single();

    for (bullet, bullet_transform) in bullet_query.iter() {
        if bullet_transform
            .translation
            .distance(player_transform.translation)
            < PLAYER_HEIGHT
        {
            commands.entity(bullet).despawn();
            invader_hit_player_event.send(InvaderBulletHitPlayerEvent);
            return;
        }
    }
}
