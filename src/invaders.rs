use bevy::prelude::*;

use crate::{walls, Invader};

const INVADER_SPRITE_PATH: &str = "green.png";

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
