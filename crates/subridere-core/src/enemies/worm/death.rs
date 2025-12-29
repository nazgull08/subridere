use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use super::components::{Worm, WormHead, WormSegment};

/// Marker for corpse segment
#[derive(Component)]
pub struct CorpseSegment {
    pub fade_timer: Timer,
}

/// Marker for blood pool
#[derive(Component)]
pub struct BloodPool {
    pub fade_timer: Timer,
}

/// Spawns corpse on worm death
pub fn spawn_corpse_on_death(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    segment_data: Vec<(Vec3, Quat, Vec3, Color)>,  // ✅ position, rotation, size, color
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();
    
    let mut death_position = Vec3::ZERO;
    
    for (position, rotation, size, color) in &segment_data {
        death_position += *position;
        
        // Use original segment size
        let mesh = meshes.add(Cuboid::new(size.x, size.y, size.z));
        let material = materials.add(StandardMaterial {
            base_color: *color,  // ✅ Original color
            ..default()
        });
        
        let impulse = Vec3::new(
            rng.gen_range(-5.0..5.0),
            rng.gen_range(4.0..10.0),
            rng.gen_range(-5.0..5.0),
        );
        
        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(*position).with_rotation(*rotation),
            GlobalTransform::default(),
            Visibility::Visible,
            RigidBody::Dynamic,
            Collider::cuboid(size.x * 0.5, size.y * 0.5, size.z * 0.5),
            Velocity::default(),
            ExternalImpulse {
                impulse,
                torque_impulse: Vec3::ZERO,
            },
            GravityScale(1.0),
            Damping {
                linear_damping: 1.0,
                angular_damping: 1.0,
            },
            CorpseSegment {
                fade_timer: Timer::from_seconds(3.0, TimerMode::Once),
            },
            Name::new("CorpseSegment"),
        ));
    }
    
    if !segment_data.is_empty() {
        death_position /= segment_data.len() as f32;
        spawn_blood_pool(commands, death_position);
    }
    
    info!("💀 Spawned corpse with {} segments", segment_data.len());
}

/// Spawns blood pool
pub fn spawn_blood_pool(commands: &mut Commands, position: Vec3) {
    commands.spawn((
        Transform::from_translation(position + Vec3::new(0.0, 0.01, 0.0))  // ✅ Ближе к земле
            .with_rotation(Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2))
            .with_scale(Vec3::splat(0.1)),
        GlobalTransform::default(),
        Visibility::Visible,
        BloodPool {
            fade_timer: Timer::from_seconds(20.0, TimerMode::Once),
        },
        Name::new("BloodPool"),
    ));
    
    info!("🩸 Spawned blood pool at {:?}", position);  // ✅ Debug
}

pub fn spawn_blood_pool_visuals(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    pools: Query<Entity, Added<BloodPool>>,
) {
    for entity in &pools {
        let mesh = meshes.add(Circle::new(2.5));  // ✅ Больше радиус (было 2.0)
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(0.05, 0.3, 0.05, 0.9),  // ✅ Темнее и более непрозрачная
            emissive: LinearRgba::rgb(0.0, 0.3, 0.0),  // ✅ Больше свечения
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });
        
        commands.entity(entity)
            .insert(Mesh3d(mesh))
            .insert(MeshMaterial3d(material));
        
        info!("🩸 Added blood pool mesh");  // ✅ Debug
    }
}

/// Fades out corpse segments over time
pub fn fade_corpse_segments(
    mut commands: Commands,
    time: Res<Time>,
    mut segments: Query<(Entity, &mut CorpseSegment, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut corpse, material_handle) in &mut segments {
        corpse.fade_timer.tick(time.delta());
        
        // Start fading after 1 second (let them settle first)
        if corpse.fade_timer.elapsed_secs() > 10.0 {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                let fade_start = 1.0;
                let fade_duration = 2.0;  // Fade from 1s to 3s
                let elapsed = corpse.fade_timer.elapsed_secs() - fade_start;
                let alpha = 1.0 - (elapsed / fade_duration).clamp(0.0, 1.0);
                material.base_color.set_alpha(alpha);
            }
        }
        
        // Despawn when done
        if corpse.fade_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Animates blood pool: grows then fades
pub fn animate_blood_pool(
    mut commands: Commands,
    time: Res<Time>,
    mut pools: Query<(Entity, &mut Transform, &mut BloodPool, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut transform, mut pool, material_handle) in &mut pools {
        pool.fade_timer.tick(time.delta());
        
        let progress = pool.fade_timer.fraction();  // 0.0 -> 1.0
        
        // PHASE 1: Grow (0.0 -> 0.3): scale 0.1 -> 1.0
        if progress < 0.3 {
            let grow = progress / 0.3;  // Normalize to 0.0 -> 1.0
            transform.scale = Vec3::splat(0.1 + grow * 0.9);
        } else if progress < 0.6 {
            // PHASE 2: Full size (0.3 -> 0.6)
            transform.scale = Vec3::ONE;
        }
        
        // PHASE 3: Fade out (0.6 -> 1.0): alpha 0.9 -> 0.0
        if progress > 0.6 {
            if let Some(material) = materials.get_mut(&material_handle.0) {
                let fade_progress = (progress - 0.6) / 0.4;  // 0.0 -> 1.0
                let alpha = 0.9 * (1.0 - fade_progress);
                material.base_color.set_alpha(alpha);
                
                // Fade emissive too for smooth disappearance
                let glow = 0.3 * (1.0 - fade_progress);
                material.emissive = LinearRgba::rgb(0.0, glow, 0.0);
            }
        }
        
        // Despawn when done
        if pool.fade_timer.finished() {
            commands.entity(entity).despawn();
        }
    }
}
