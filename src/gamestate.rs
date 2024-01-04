use std::process::exit;

use bevy::prelude::*;

use crate::{
    Invader, InvaderBulletHitPlayerEvent, InvadersReachedBottomEvent, Player,
    PlayerKilledAllInvadersEvent,
};

pub(crate) fn invaders_hit_player(
    mut invaders_reached_bottom_event: EventReader<InvadersReachedBottomEvent>,
) {
    if let Some(_event) = invaders_reached_bottom_event.read().next() {
        exit(0);
    }
}

pub(crate) fn invader_bullet_hit_player(
    mut invader_bullet_hit_player_event: EventReader<InvaderBulletHitPlayerEvent>,
) {
    if let Some(_event) = invader_bullet_hit_player_event.read().next() {
        exit(0);
    }
}

pub(crate) fn player_killed_all_invaders(
    mut player_killed_all_invaders_event: EventReader<PlayerKilledAllInvadersEvent>,
) {
    if let Some(_event) = player_killed_all_invaders_event.read().next() {
        exit(0);
    }
}

pub(crate) fn check_player_killed_all_invaders(
    invader_query: Query<With<Invader>>,
    mut player_killed_all_invaders_event: EventWriter<PlayerKilledAllInvadersEvent>,
) {
    if invader_query.iter().count() == 0 {
        player_killed_all_invaders_event.send(PlayerKilledAllInvadersEvent);
    }
}
