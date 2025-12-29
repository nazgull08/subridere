use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::{enemies::worm::particles::spawn_blood_splatter, fighting::projectile::weapons::DamageProjectile, stats::{
    damage::component::{Damage, DamageType}, health::component::Health
}};
use super::components::{Worm, WormSegment, WormHead};
use crate::audio::worm::events::WormHurtEvent;

/// Detects projectile hits on worm and applies damage
pub fn worm_projectile_damage_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,  // ✅ ДОБАВИТЬ
    mut materials: ResMut<Assets<StandardMaterial>>,  // ✅ ДОБАВИТЬ
    mut collision_events: EventReader<CollisionEvent>,
    mut hurt_event: EventWriter<WormHurtEvent>,
    projectiles: Query<(&Transform, &Velocity), With<DamageProjectile>>,  // ✅ Добавить Transform и Velocity
    worm_segments: Query<(&Transform, &WormHead)>,  // ✅ Добавить Transform
    worms: Query<Entity, With<Worm>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            // Check if collision involves projectile and worm head
            let (projectile_entity, worm_head_entity) = 
                if projectiles.get(*e1).is_ok() && worm_segments.get(*e2).is_ok() {
                    (*e1, *e2)
                } else if projectiles.get(*e2).is_ok() && worm_segments.get(*e1).is_ok() {
                    (*e2, *e1)
                } else {
                    continue;
                };

            // Get projectile info
            if let Ok((projectile_transform, projectile_velocity)) = projectiles.get(projectile_entity) {
                // Get worm head info
                if let Ok((head_transform, head)) = worm_segments.get(worm_head_entity) {
                    let worm_root = head.worm_root;

                    // ✅ Spawn blood splatter at hit position
                    spawn_blood_splatter(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        head_transform.translation,  // Hit position
                        projectile_velocity.linvel,  // Hit direction
                    );

                    // Apply damage to worm root
                    commands.entity(worm_root).insert(Damage {
                        amount: 10.0,
                        damage_type: DamageType::Magical,
                    });

                    hurt_event.send(WormHurtEvent);

                    // Despawn projectile
                    commands.entity(projectile_entity).despawn();

                    info!("💥 Projectile hit worm! 10 damage");
                }
            }
        }
    }
}

/// Despawns dead worms
pub fn worm_death_system(
    mut commands: Commands,
    worms: Query<(Entity, &Health), With<Worm>>,
    worm_heads: Query<(Entity, &WormHead)>,  // ✅ Добавить Entity
    worm_segments: Query<(Entity, &WormSegment)>,  // ✅ Добавить Entity
) {
    for (worm_entity, health) in &worms {
        if health.current <= 0.0 {
            info!("💀 Worm died!");

            // Find and despawn head
            for (segment_entity, head) in worm_heads.iter() {
                if head.worm_root == worm_entity {
                    commands.entity(segment_entity).despawn();
                }
            }
            
            // Find and despawn body segments
            for (segment_entity, segment) in worm_segments.iter() {
                if segment.worm_root == worm_entity {
                    commands.entity(segment_entity).despawn();
                }
            }

            // Despawn worm root
            commands.entity(worm_entity).despawn();
        }
    }
}
