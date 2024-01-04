use std::time::Duration;

use bevy::prelude::*;
use rand::prelude::*;

use crate::{
    on_invaders_hit_player, walls, Collider, Invader, InvaderBulletFiredEvent, InvaderDifficulty,
    INVADER_SIZE, PLAYER_Y,
};

const MOVEMENT_RATE: f32 = 0.1;
const MOVEMENT: f32 = 5.0;
const INVADER_WALL_PADDING: f32 = 20.0;
const MOVE_DOWN_AMOUNT: f32 = 15.0;
const PLAYER_COLLISION_Y: f32 = PLAYER_Y + 20.0;

impl InvaderDifficulty {
    fn from_i32(value: i32) -> Self {
        match value {
            0 => InvaderDifficulty::Easy,
            1 => InvaderDifficulty::Medium,
            2 => InvaderDifficulty::Hard,
            3 => InvaderDifficulty::Medium,
            _ => InvaderDifficulty::Easy,
        }
    }

    fn get_sprite_path(&self) -> String {
        String::from(match self {
            InvaderDifficulty::Easy => "green.png",
            InvaderDifficulty::Medium => "yellow.png",
            InvaderDifficulty::Hard => "red.png",
        })
    }

    pub(crate) fn get_bullet_sprite_path(&self) -> String {
        String::from(match self {
            InvaderDifficulty::Easy => "green-bullet.png",
            InvaderDifficulty::Medium => "yellow-bullet.png",
            InvaderDifficulty::Hard => "red-bullet.png",
        })
    }

    pub(crate) fn get_bullet_speed(&self) -> f32 {
        match self {
            InvaderDifficulty::Easy => 200.0,
            InvaderDifficulty::Medium => 350.0,
            InvaderDifficulty::Hard => 500.0,
        }
    }
}

#[derive(Resource)]
pub(crate) struct InvaderConfig {
    movement_timer: Timer,
    direction: f32,
    wall_collision_timer: Timer,
    move_down: bool,
}

pub(crate) fn setup(mut commands: Commands) {
    commands.insert_resource(InvaderConfig {
        movement_timer: Timer::new(Duration::from_secs_f32(MOVEMENT_RATE), TimerMode::Repeating),
        wall_collision_timer: Timer::new(Duration::from_secs_f32(1.0), TimerMode::Once),
        direction: 1.0,
        move_down: false,
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

    (0..n_rows).for_each(|row| {
        (0..n_cols).for_each(|_column| {
            let invader: Invader = Invader {
                difficulty: InvaderDifficulty::from_i32(row),
            };
            let invader_sprite_path = invader.difficulty.get_sprite_path();
            commands.spawn((
                invader,
                SpriteBundle {
                    texture: asset_server.load(invader_sprite_path),
                    transform: Transform {
                        translation: invader_position,
                        ..default()
                    },
                    ..default()
                },
            ));
            invader_position.x += horizontal_spacing;
        });
        invader_position.x = starting_x;
        invader_position.y -= vertical_spacing;
    });
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

    invader_query.iter_mut().for_each(|mut invader_transform| {
        invader_transform.translation.x += MOVEMENT * invader_config.direction;
    });
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
                collider_transform.scale.truncate() + walls::WALL_THICKNESS,
            )
            .is_some()
            {
                invader_config.direction *= -1.0;
                invader_config.wall_collision_timer.reset();
                invader_config.move_down = true;
                return;
            }
        }
    }
}

pub(crate) fn maybe_move_invaders_down(
    mut invader_query: Query<&mut Transform, With<Invader>>,
    mut invader_config: ResMut<InvaderConfig>,
) {
    if !invader_config.move_down {
        return;
    }

    invader_config.move_down = false;

    invader_query.iter_mut().for_each(|mut invader_transform| {
        invader_transform.translation.y -= MOVE_DOWN_AMOUNT;
    });
}

pub(crate) fn check_invaders_reached_bottom(invader_query: Query<&Transform, With<Invader>>) {
    for invader_transform in invader_query.iter() {
        if invader_transform.translation.y <= PLAYER_COLLISION_Y {
            on_invaders_hit_player();
            return;
        }
    }
}

pub(crate) fn maybe_shoot(
    invader_query: Query<(&Invader, &Transform), With<Invader>>,
    mut invader_bullet_fired_event: EventWriter<InvaderBulletFiredEvent>,
) {
    for (invader, invader_transform) in invader_query.iter() {
        let rng = rand::thread_rng().gen::<f32>();
        if rng < 0.999 {
            continue;
        }

        invader_bullet_fired_event.send(InvaderBulletFiredEvent {
            position: invader_transform.translation + Vec3::new(0.0, -10.0, 0.0),
            invader_difficulty: invader.difficulty.clone(),
        });
    }
}
