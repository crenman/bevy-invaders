use bevy::prelude::*;

use crate::{walls, Invader, PLAYER_Y};

const INVADER_SPRITE_PATH: &str = "green.png";

const INVADER_SIZE: Vec2 = Vec2::new(100., 30.);
// These values are exact
const GAP_BETWEEN_PADDLE_AND_INVADERS: f32 = 300.0;
const GAP_BETWEEN_INVADERS: f32 = 40.0;
// These values are lower bounds, as the number of INVADERS is computed
const GAP_BETWEEN_INVADERS_AND_CEILING: f32 = 20.0;
const GAP_BETWEEN_INVADERS_AND_SIDES: f32 = 20.0;

pub(crate) fn spawn_invaders(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load(INVADER_SPRITE_PATH),
            transform: Transform {
                translation: Vec3::new(0.0, 0.0, 0.0),
                rotation: Quat::IDENTITY,
                scale: Vec3::ONE,
            },
            ..default()
        },
        Invader,
    ));

    // invaders
    let total_width_of_invaders =
        (walls::RIGHT_WALL - walls::LEFT_WALL) - 2. * GAP_BETWEEN_INVADERS_AND_SIDES;
    let bottom_edge_of_invaders = PLAYER_Y + GAP_BETWEEN_PADDLE_AND_INVADERS;
    let total_height_of_invaders =
        walls::TOP_WALL - bottom_edge_of_invaders - GAP_BETWEEN_INVADERS_AND_CEILING;

    assert!(total_width_of_invaders > 0.0);
    assert!(total_height_of_invaders > 0.0);

    // Given the space available, compute how many rows and columns of invaders we can fit
    let n_columns =
        (total_width_of_invaders / (INVADER_SIZE.x + GAP_BETWEEN_INVADERS)).floor() as usize;
    let n_rows =
        (total_height_of_invaders / (INVADER_SIZE.y + GAP_BETWEEN_INVADERS)).floor() as usize;
    let n_vertical_gaps = n_columns - 1;

    // Because we need to round the number of columns,
    // the space on the top and sides of the invaders only captures a lower bound, not an exact value
    let center_of_invaders = (walls::LEFT_WALL + walls::RIGHT_WALL) / 2.0;
    let left_edge_of_invaders = center_of_invaders
        // Space taken up by the invaders
        - (n_columns as f32 / 2.0 * INVADER_SIZE.x)
        // Space taken up by the gaps
        - n_vertical_gaps as f32 / 2.0 * GAP_BETWEEN_INVADERS;

    // In Bevy, the `translation` of an entity describes the center point,
    // not its bottom-left corner
    let offset_x = left_edge_of_invaders + INVADER_SIZE.x / 2.;
    let offset_y = bottom_edge_of_invaders + INVADER_SIZE.y / 2.;

    for row in 0..n_rows {
        for column in 0..n_columns {
            let invader_position = Vec2::new(
                offset_x + column as f32 * (INVADER_SIZE.x + GAP_BETWEEN_INVADERS),
                offset_y + row as f32 * (INVADER_SIZE.y + GAP_BETWEEN_INVADERS),
            );

            commands.spawn((
                SpriteBundle {
                    texture: asset_server.load(INVADER_SPRITE_PATH),
                    transform: Transform {
                        translation: invader_position.extend(0.0),
                        scale: Vec3::ONE,
                        ..default()
                    },
                    ..default()
                },
                Invader,
            ));
        }
    }
}
