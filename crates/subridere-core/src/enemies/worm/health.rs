use super::components::{Worm, WormHead, WormSegment};
use crate::audio::worm::events::WormHurtEvent;
use crate::{
    enemies::worm::{death::spawn_corpse_on_death, particles::spawn_blood_splatter},
    fighting::projectile::weapons::DamageProjectile,
    stats::{
        damage::component::{Damage, DamageType},
        health::component::Health,
    },
};
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Detects projectile hits on worm and applies damage
pub fn worm_projectile_damage_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,                // ✅ ДОБАВИТЬ
    mut materials: ResMut<Assets<StandardMaterial>>, // ✅ ДОБАВИТЬ
    mut collision_events: EventReader<CollisionEvent>,
    mut hurt_event: EventWriter<WormHurtEvent>,
    projectiles: Query<(&Transform, &Velocity), With<DamageProjectile>>, // ✅ Добавить Transform и Velocity
    worm_segments: Query<(&Transform, &WormHead)>,                       // ✅ Добавить Transform
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
            if let Ok((_, projectile_velocity)) = projectiles.get(projectile_entity) {
                // Get worm head info
                if let Ok((head_transform, head)) = worm_segments.get(worm_head_entity) {
                    let worm_root = head.worm_root;

                    // ✅ Spawn blood splatter at hit position
                    spawn_blood_splatter(
                        &mut commands,
                        &mut meshes,
                        &mut materials,
                        head_transform.translation, // Hit position
                        projectile_velocity.linvel, // Hit direction
                    );

                    // Apply damage to worm root
                    commands.entity(worm_root).insert(Damage {
                        amount: 10.0,
                        damage_type: DamageType::Magical,
                    });

                    hurt_event.write(WormHurtEvent);

                    // Despawn projectile
                    commands.entity(projectile_entity).despawn();

                    info!("💥 Projectile hit worm! 10 damage");
                }
            }
        }
    }
}

/// Despawns dead worms and spawns corpse
pub fn worm_death_system(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    worms: Query<(Entity, &Health), With<Worm>>,
    worm_heads: Query<(
        Entity,
        &Transform,
        &WormHead,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    worm_segments: Query<(
        Entity,
        &Transform,
        &WormSegment,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    // ❌ УБРАТЬ: existing_materials: Res<Assets<StandardMaterial>>,
) {
    for (worm_entity, health) in &worms {
        if health.current <= 0.0 {
            info!("💀 Worm died!");

            let mut segment_data = Vec::new();

            // Collect head data
            for (_seg_entity, transform, head, material_handle) in &worm_heads {
                if head.worm_root == worm_entity {
                    // ✅ Используем materials (ResMut) вместо existing_materials
                    let color = if let Some(mat) = materials.get(&material_handle.0) {
                        mat.base_color
                    } else {
                        Color::srgba(0.8, 0.2, 0.2, 1.0)
                    };

                    segment_data.push((
                        transform.translation,
                        transform.rotation,
                        Vec3::splat(0.8),
                        color,
                    ));
                }
            }

            // Collect segment data
            for (_seg_entity, transform, segment, material_handle) in &worm_segments {
                if segment.worm_root == worm_entity {
                    // ✅ Используем materials (ResMut)
                    let color = if let Some(mat) = materials.get(&material_handle.0) {
                        mat.base_color
                    } else {
                        Color::srgba(0.2, 0.7, 0.3, 1.0)
                    };

                    segment_data.push((
                        transform.translation,
                        transform.rotation,
                        Vec3::splat(0.8),
                        color,
                    ));
                }
            }

            // Spawn corpse
            spawn_corpse_on_death(&mut commands, &mut meshes, &mut materials, segment_data);

            // Despawn all segments
            for (seg_entity, _transform, head, _) in &worm_heads {
                if head.worm_root == worm_entity {
                    commands.entity(seg_entity).despawn();
                }
            }

            for (seg_entity, _transform, segment, _) in &worm_segments {
                if segment.worm_root == worm_entity {
                    commands.entity(seg_entity).despawn();
                }
            }

            commands.entity(worm_entity).despawn();
        }
    }
}
