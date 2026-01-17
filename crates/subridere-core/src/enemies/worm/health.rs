use super::components::{Worm, WormHead, WormSegment};
use crate::{enemies::worm::death::spawn_corpse_on_death, stats::health::component::Health};
use bevy::prelude::*;

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
