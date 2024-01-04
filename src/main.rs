use std::process::exit;

use bevy::{
    prelude::*,
    sprite::collide_aabb::{collide, Collision},
    sprite::MaterialMesh2dBundle,
};
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod bullets;
pub mod collider;
pub mod invaders;
pub mod player;
pub mod walls;

const PLAYER_SPEED: f32 = 500.0;
const PLAYER_WIDTH: f32 = 60.0;
const PLAYER_HEIGHT: f32 = 30.0;
const INVADER_SIZE: f32 = 20.0;
const GAP_BETWEEN_PLAYER_AND_FLOOR: f32 = 40.0;

const PLAYER_Y: f32 = walls::BOTTOM_WALL + GAP_BETWEEN_PLAYER_AND_FLOOR;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(WorldInspectorPlugin::new())
        .add_state::<GameState>()
        .add_event::<InvaderBulletFiredEvent>()
        .add_event::<PlayerBulletFiredEvent>()
        .add_systems(
            Startup,
            (
                spawn_camera,
                walls::spawn_walls,
                player::setup,
                player::spawn_player,
                invaders::setup,
                invaders::spawn_invaders,
            ),
        )
        .add_systems(
            Update,
            (
                (
                    player::move_player,
                    player::shoot,
                    invaders::move_invaders,
                    invaders::maybe_shoot,
                    invaders::check_invader_wall_collision,
                    invaders::maybe_move_invaders_down,
                    invaders::check_invaders_reached_bottom,
                    bullets::spawn_player_bullet,
                    bullets::spawn_invader_bullet,
                    bullets::move_bullets,
                    bullets::check_bullet_wall_collision,
                    bullets::check_player_bullet_invader_collision,
                    bullets::check_invader_bullet_player_collision,
                )
                    .chain(),
                bevy::window::close_on_esc,
            ),
        )
        .run();
}

#[derive(Component)]
struct Player;

enum InvaderDifficulty {
    Easy,
    Medium,
    Hard,
}

#[derive(Component)]
struct Invader {
    difficulty: InvaderDifficulty,
}

#[derive(Component)]
struct Collider;

#[derive(Component)]
struct CommonBullet;

#[derive(Component)]
struct PlayerBullet;

#[derive(Event)]
struct PlayerBulletFiredEvent(Vec3);

#[derive(Component)]
struct InvaderBullet;

#[derive(Event)]
struct InvaderBulletFiredEvent(Vec3);

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum GameState {
    #[default]
    MainMenu,
    Game,
    PostGame,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn on_invaders_hit_player() {
    exit(0);
}

fn on_invader_bullet_hit_player() {
    exit(0);
}
