use bevy::prelude::*;

use crate::{
    bullets, gamestate, invaders, player, shields, walls, InvaderBulletFiredEvent,
    InvaderBulletHitPlayerEvent, InvadersReachedBottomEvent, PlayerBulletFiredEvent,
    PlayerKilledAllInvadersEvent, ShootSound,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<GameState>()
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
                    shields::spawn_shields,
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
                        shields::check_bullet_shield_collision,
                        gamestate::check_player_killed_all_invaders,
                        gamestate::invader_bullet_hit_player,
                        gamestate::invaders_hit_player,
                        gamestate::player_killed_all_invaders,
                    )
                        .chain(),
                    bevy::window::close_on_esc,
                ),
            );
    }
}

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
