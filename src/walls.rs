use bevy::prelude::*;

use crate::Collider;

pub(crate) const WALL_THICKNESS: f32 = 10.0;
pub(crate) const LEFT_WALL: f32 = -450.;
pub(crate) const RIGHT_WALL: f32 = 450.;
pub(crate) const BOTTOM_WALL: f32 = -300.;
pub(crate) const TOP_WALL: f32 = 300.;

const WALL_COLOR: Color = Color::rgb(0.8, 0.8, 0.8);

#[derive(Bundle)]
struct WallBundle {
    // You can nest bundles inside of other bundles like this
    // Allowing you to compose their functionality
    sprite_bundle: SpriteBundle,
    collider: Collider,
}

/// Which side of the arena is this wall located on?
enum WallLocation {
    Left,
    Right,
    Bottom,
    Top,
}

impl WallLocation {
    fn position(&self) -> Vec3 {
        match self {
            WallLocation::Left => Vec3::new(LEFT_WALL, 0., 0.),
            WallLocation::Right => Vec3::new(RIGHT_WALL, 0., 0.),
            WallLocation::Bottom => Vec3::new(0., BOTTOM_WALL, 0.),
            WallLocation::Top => Vec3::new(0., TOP_WALL, 0.),
        }
    }

    fn size(&self) -> Vec3 {
        let arena_height = TOP_WALL - BOTTOM_WALL;
        let arena_width = RIGHT_WALL - LEFT_WALL;
        // Make sure we haven't messed up our constants
        assert!(arena_height > 0.0);
        assert!(arena_width > 0.0);

        match self {
            WallLocation::Left | WallLocation::Right => {
                Vec3::new(WALL_THICKNESS, arena_height + WALL_THICKNESS, 1.0)
            }
            WallLocation::Bottom | WallLocation::Top => {
                Vec3::new(arena_width + WALL_THICKNESS, WALL_THICKNESS, 1.0)
            }
        }
    }
}

impl WallBundle {
    // This "builder method" allows us to reuse logic across our wall entities,
    // making our code easier to read and less prone to bugs when we change the logic
    fn new(location: WallLocation) -> WallBundle {
        WallBundle {
            sprite_bundle: SpriteBundle {
                transform: Transform {
                    translation: location.position(),
                    scale: location.size(),
                    ..default()
                },
                sprite: Sprite {
                    color: WALL_COLOR,
                    ..default()
                },
                ..default()
            },
            collider: Collider,
        }
    }
}

pub(crate) fn spawn_walls(mut commands: Commands) {
    commands.spawn(WallBundle::new(WallLocation::Left));
    commands.spawn(WallBundle::new(WallLocation::Right));
    commands.spawn(WallBundle::new(WallLocation::Bottom));
    commands.spawn(WallBundle::new(WallLocation::Top));
}
