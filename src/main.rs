use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

pub mod bullets;
pub mod collider;
pub mod gamestate;
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
        // .add_plugins(WorldInspectorPlugin::new())
        .add_state::<GameState>()
        .add_event::<InvaderBulletFiredEvent>()
        .add_event::<PlayerBulletFiredEvent>()
        .add_event::<InvadersReachedBottomEvent>()
        .add_event::<InvaderBulletHitPlayerEvent>()
        .add_event::<PlayerKilledAllInvadersEvent>()
        .add_systems(
            Startup,
            (
                spawn_camera,
                setup_sound,
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
                    bullets::player_bullet_sound,
                    gamestate::check_player_killed_all_invaders,
                    gamestate::invader_bullet_hit_player,
                    gamestate::invaders_hit_player,
                    gamestate::player_killed_all_invaders,
                )
                    .chain(),
                bevy::window::close_on_esc,
            ),
        )
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

fn setup_sound(mut commands: Commands, asset_server: Res<AssetServer>) {
    let ambience: Handle<AudioSource> = asset_server.load("space-invaders-drum.wav");

    commands.spawn(AudioBundle {
        source: ambience,
        settings: PlaybackSettings::LOOP,
    });

    let shoot_sound: Handle<AudioSource> = asset_server.load("laser.wav");
    commands.insert_resource(ShootSound(shoot_sound));
}
