use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod bullets;
pub mod collider;
pub mod invaders;
pub mod player;
pub mod walls;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_WIDTH: f32 = 60.0;
const GAP_BETWEEN_PLAYER_AND_FLOOR: f32 = 40.0;

const PLAYER_Y: f32 = walls::BOTTOM_WALL + GAP_BETWEEN_PLAYER_AND_FLOOR;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_systems(
            Startup,
            (
                spawn_camera,
                walls::spawn_walls,
                player::setup,
                player::spawn_player,
                invaders::spawn_invaders,
            ),
        )
        .add_systems(
            Update,
            (
                (
                    player::move_player,
                    player::shoot,
                    bullets::move_bullets,
                    bullets::check_bullet_collider_collision,
                    bullets::check_bullet_invader_collision,
                )
                    .chain(),
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Invader;

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct Bullet;

#[derive(Component)]
struct Velocity(Vec3);

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
