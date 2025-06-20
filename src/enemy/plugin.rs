use bevy::prelude::*;

use crate::enemy::{spawn::spawn_jester_in_room, fsm::update_enemy_fsm};

use super::{assets::load_jester_animations, sync_anim_state::sync_anim_state};

pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, load_jester_animations)
            .add_systems(Startup, spawn_jester_in_room)
            .add_systems(Update, update_enemy_fsm)
            .add_systems(Update, sync_anim_state);
    }
}
