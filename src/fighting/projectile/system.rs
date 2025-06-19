use super::component::Projectile;
use bevy::prelude::*;

pub fn despawn_expired_projectiles(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut Projectile)>,
) {
    for (entity, mut projectile) in &mut query {
        projectile.lifetime -= time.delta_secs();
        if projectile.lifetime <= 0.0 {
            commands.entity(entity).despawn();
            println!("Projectile {:?} despawned", entity);
        }
    }
}
