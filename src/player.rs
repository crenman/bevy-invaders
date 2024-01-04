use std::time::Duration;

use bevy::prelude::*;

use crate::{walls, Player, PlayerBulletFiredEvent, PLAYER_SPEED, PLAYER_WIDTH, PLAYER_Y};

const PLAYER_SPRITE_PATH: &str = "player.png";
const FIRE_RATE: f32 = 0.2;

#[derive(Resource, Debug)]
pub(crate) struct PlayerShootConfig {
    timer: Timer,
}

pub(crate) fn setup(mut commands: Commands) {
    commands.insert_resource(PlayerShootConfig {
        timer: Timer::new(Duration::from_secs_f32(FIRE_RATE), TimerMode::Once),
    });
}

pub(crate) fn spawn_player(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(PLAYER_SPRITE_PATH),
            transform: Transform {
                translation: Vec3::new(0.0, PLAYER_Y, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..default()
        },
        Player,
    ));
}

pub(crate) fn move_player(
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<&mut Transform, With<Player>>,
    time: Res<Time>,
) {
    let mut player_transform = query.single_mut();
    let mut direction = 0.0;

    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction += 1.0;
    }

    let new_player_position =
        player_transform.translation.x + direction * PLAYER_SPEED * time.delta_seconds();

    let left_bound = walls::LEFT_WALL + walls::WALL_THICKNESS / 2.0 + (PLAYER_WIDTH / 2.0);
    let right_bound = walls::RIGHT_WALL - walls::WALL_THICKNESS / 2.0 - (PLAYER_WIDTH / 2.0);

    player_transform.translation.x = new_player_position.clamp(left_bound, right_bound);
}

pub(crate) fn shoot(
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
    mut shoot_config: ResMut<PlayerShootConfig>,
    time: Res<Time>,
    mut player_bullet_fired_event: EventWriter<PlayerBulletFiredEvent>,
) {
    shoot_config.timer.tick(time.delta());

    if !shoot_config.timer.finished() {
        return;
    }

    if keyboard_input.pressed(KeyCode::Space) {
        shoot_config.timer.reset();

        let player_transform = player_query.single();

        player_bullet_fired_event.send(PlayerBulletFiredEvent(
            player_transform.translation + Vec3::new(0.0, 10.0, 0.0),
        ));
    }
}
