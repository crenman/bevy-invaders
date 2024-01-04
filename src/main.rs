use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod bullets;
pub mod game;
pub mod gamestate;
pub mod invaders;
pub mod player;
pub mod shields;
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
        .add_plugins(game::GamePlugin)
        // .add_plugins(WorldInspectorPlugin::new())
        .run();
}

#[derive(Component)]
struct Player;

#[derive(Clone)]
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
struct Shield(i32);

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
struct InvaderBulletFiredEvent {
    position: Vec3,
    invader_difficulty: InvaderDifficulty,
}

#[derive(Event)]
struct InvadersReachedBottomEvent;

#[derive(Event)]
struct InvaderBulletHitPlayerEvent;

#[derive(Event)]
struct PlayerKilledAllInvadersEvent;

#[derive(Component)]
struct Velocity(Vec3);

#[derive(Resource)]
struct ShootSound(Handle<AudioSource>);
