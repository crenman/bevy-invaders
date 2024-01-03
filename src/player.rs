use bevy::prelude::*;

use crate::{bullets, walls, Player, PLAYER_SPEED, PLAYER_WIDTH, PLAYER_Y};

const PLAYER_SPRITE_PATH: &str = "player.png";

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

    if keyboard_input.pressed(KeyCode::Left) {
        direction -= 1.0;
    }

    if keyboard_input.pressed(KeyCode::Right) {
        direction += 1.0;
    }

    // Calculate the new horizontal paddle position based on player input
    let new_player_position =
        player_transform.translation.x + direction * PLAYER_SPEED * time.delta_seconds();

    // Update the paddle position, making sure it doesn't cause the paddle to leave the arena
    let left_bound = walls::LEFT_WALL + walls::WALL_THICKNESS / 2.0 + (PLAYER_WIDTH / 2.0);
    let right_bound = walls::RIGHT_WALL - walls::WALL_THICKNESS / 2.0 - (PLAYER_WIDTH / 2.0);

    player_transform.translation.x = new_player_position.clamp(left_bound, right_bound);
}

pub(crate) fn shoot(
    commands: Commands,
    asset_server: Res<AssetServer>,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
) {
    let player_transform = player_query.single();

    if keyboard_input.pressed(KeyCode::Space) {
        let mut bullet_transform = *player_transform;
        bullet_transform.translation.y += 10.0;

        bullets::spawn_player_bullet(commands, asset_server, &bullet_transform);
    }
}
