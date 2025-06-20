use super::system::despawn_expired_projectiles;
use bevy::prelude::*;

pub struct ProjectilePlugin;

impl Plugin for ProjectilePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (despawn_expired_projectiles,));
    }
}
