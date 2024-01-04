use bevy::prelude::*;

use crate::{walls, CommonBullet, Shield};

const SHIELD_SIZE: f32 = 30.0;

impl Shield {
    fn get_sprite_path(&self) -> String {
        String::from(match self.0 {
            3 => "shield-0.png",
            2 => "shield-1.png",
            1 => "shield-2.png",
            0 => "shield-3.png",
            _ => "shield-0.png",
        })
    }
}

pub(crate) fn spawn_shields(mut commands: Commands, asset_server: Res<AssetServer>) {
    let n_cols = 4;
    let height = walls::BOTTOM_WALL + 100.0;
    let width = (-walls::LEFT_WALL + walls::RIGHT_WALL) / 1.5;
    let horizontal_spacing = width / n_cols as f32;
    let starting_x = -width / (n_cols - 1) as f32;
    let starting_position = Vec3::new(starting_x, height, 0.0);

    let mut shield_position = starting_position;

    (0..n_cols).for_each(|_column| {
        let shield: Shield = Shield(3);
        let shield_sprite_path = shield.get_sprite_path();
        commands.spawn((
            shield,
            SpriteBundle {
                texture: asset_server.load(shield_sprite_path),
                transform: Transform {
                    translation: shield_position,
                    ..default()
                },
                ..default()
            },
        ));
        shield_position.x += horizontal_spacing;
    });
}

pub(crate) fn check_bullet_shield_collision(
    mut commands: Commands,
    bullet_query: Query<(Entity, &Transform), With<CommonBullet>>,
    shield_query: Query<(Entity, &Transform, &mut Shield), With<Shield>>,
    asset_server: Res<AssetServer>,
) {
    for (bullet, bullet_transform) in bullet_query.iter() {
        for (shield_entity, shield_transform, shield) in shield_query.iter() {
            if bullet_transform
                .translation
                .distance(shield_transform.translation)
                < SHIELD_SIZE
            {
                commands.entity(bullet).despawn();
                commands.entity(shield_entity).despawn();

                let new_shield_level = shield.0 - 1;

                if new_shield_level < 0 {
                    continue;
                }

                let shield: Shield = Shield(shield.0 - 1);
                let shield_sprite_path = shield.get_sprite_path();
                commands.spawn((
                    shield,
                    SpriteBundle {
                        texture: asset_server.load(shield_sprite_path),
                        transform: Transform {
                            translation: shield_transform.translation,
                            ..default()
                        },
                        ..default()
                    },
                ));
            }
        }
    }
}
