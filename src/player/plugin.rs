use bevy::prelude::*;

use crate::{
    player::component::{PLAYER_START_POS, Player},
    unit::component::Velocity,
};

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (kill_plane_system,));
    }
}

fn kill_plane_system(mut query: Query<(&mut Transform, &mut Velocity), With<Player>>) {
    for (mut transform, mut velocity) in &mut query {
        if transform.translation.y < -50.0 {
            println!("💀 Player died. Respawning...");
            transform.translation = PLAYER_START_POS;
            velocity.0 = Vec3::ZERO;
        }
    }
}
