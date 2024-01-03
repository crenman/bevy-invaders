use std::time::Duration;

use bevy::prelude::*;

use crate::{walls, Collider, Invader};

const INVADER_SPRITE_PATH: &str = "green.png";
const MOVEMENT_RATE: f32 = 0.1;
const MOVEMENT: f32 = 5.0;
pub(crate) const INVADER_SIZE: f32 = 20.0;
const INVADER_WALL_PADDING: f32 = 20.0;

#[derive(Resource)]
pub(crate) struct InvaderConfig {
    movement_timer: Timer,
    direction: f32,
    wall_collision_timer: Timer,
}

pub(crate) fn setup(mut commands: Commands) {
    commands.insert_resource(InvaderConfig {
        movement_timer: Timer::new(Duration::from_secs_f32(MOVEMENT_RATE), TimerMode::Repeating),
        wall_collision_timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
        direction: 1.0,
    });
}

pub(crate) fn spawn_invaders(mut commands: Commands, asset_server: Res<AssetServer>) {
    let n_rows = 5;
    let n_cols = 11;

    let height = 300.0;
    let max_height = walls::TOP_WALL - 50.0;
    let width = (-walls::LEFT_WALL + walls::RIGHT_WALL) / 1.5;
    let horizontal_spacing = width / n_cols as f32;
    let vertical_spacing = height / n_rows as f32;
    let starting_x = -(width / 2.0);
    let starting_position = Vec3::new(starting_x, max_height, 0.0);

    let mut invader_position = starting_position;

    for _row in 0..n_rows {
        for _column in 0..n_cols {
            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(INVADER_SPRITE_PATH),
                    transform: Transform {
                        translation: invader_position,
                        ..default()
                    },
                    ..default()
                },
                Invader,
            ));
            invader_position.x += horizontal_spacing;
        }
        invader_position.x = starting_x;
        invader_position.y -= vertical_spacing;
    }
}

pub(crate) fn move_invaders(
    mut invader_query: Query<&mut Transform, With<Invader>>,
    mut invader_config: ResMut<InvaderConfig>,
    time: Res<Time>,
) {
    invader_config.movement_timer.tick(time.delta());

    if !invader_config.movement_timer.just_finished() {
        return;
    }

    for mut invader_transform in invader_query.iter_mut() {
        invader_transform.translation.x += MOVEMENT * invader_config.direction;
    }
}

pub(crate) fn check_invader_wall_collision(
    invader_query: Query<&Transform, With<Invader>>,
    collider_query: Query<&Transform, With<Collider>>,
    mut invader_config: ResMut<InvaderConfig>,
    time: Res<Time>,
) {
    invader_config.wall_collision_timer.tick(time.delta());

    if !invader_config.wall_collision_timer.finished() {
        return;
    }

    for invader_transform in invader_query.iter() {
        for collider_transform in collider_query.iter() {
            if bevy::sprite::collide_aabb::collide(
                invader_transform.translation,
                Vec2::splat(INVADER_SIZE + INVADER_WALL_PADDING),
                collider_transform.translation,
                collider_transform.scale.truncate(),
            )
            .is_some()
            {
                invader_config.direction *= -1.0;
                invader_config.wall_collision_timer.reset();
                dbg!("Collision!");

                // send event

                // for mut invader_transform in invader_query.iter_mut() {
                //     invader_transform.translation.y -= 10.0;
                // }
                return;
            }
        }
    }
}
