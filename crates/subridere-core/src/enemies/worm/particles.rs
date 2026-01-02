use super::components::WormState;
use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

/// Marker for dust particles
#[derive(Component)]
pub struct DustParticle {
    pub lifetime: Timer,
}

/// Spawns dust particles when worm lands
pub fn spawn_dust_on_landing(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    worms: Query<(&Transform, &WormState, &Velocity), Changed<WormState>>,
) {
    for (transform, state, velocity) in &worms {
        // Detect transition TO Recovering (just landed)
        if matches!(state, WormState::Recovering { .. }) {
            // Only spawn if falling fast (actual landing, not just state change)
            if velocity.linvel.y < -2.0 {
                spawn_dust_cloud(
                    &mut commands,
                    &mut meshes,
                    &mut materials,
                    transform.translation,
                );
            }
        }
    }
}

fn spawn_dust_cloud(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Spawn 8 dust particles
    for _ in 0..8 {
        let size = rng.gen_range(0.1..0.3);
        let mesh = meshes.add(Sphere::new(size));

        // Dust color (brown/gray)
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(0.6, 0.5, 0.4, 0.8),
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        // Random velocity (up and outwards)
        let horizontal = Vec3::new(
            rng.gen_range(-2.0..2.0),
            rng.gen_range(1.0..3.0), // upward
            rng.gen_range(-2.0..2.0),
        );

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position + Vec3::new(0.0, 0.2, 0.0)),
            GlobalTransform::default(),
            Visibility::Visible,
            // Physics
            RigidBody::Dynamic,
            Collider::ball(size),
            Velocity {
                linvel: horizontal,
                angvel: Vec3::ZERO,
            },
            GravityScale(0.5), // Lighter than normal
            Damping {
                linear_damping: 1.0,
                angular_damping: 0.0,
            },
            // Particle marker
            DustParticle {
                lifetime: Timer::from_seconds(1.5, TimerMode::Once),
            },
            Name::new("DustParticle"),
        ));
    }
}

/// Fades out and despawns dust particles
pub fn update_dust_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(Entity, &mut DustParticle, &MeshMaterial3d<StandardMaterial>)>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut particle, material_handle) in &mut particles {
        particle.lifetime.tick(time.delta());

        // Fade out alpha
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let progress = particle.lifetime.fraction();
            material.base_color.set_alpha(0.8 * (1.0 - progress));
        }

        // Despawn when done
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}

/// Marker for blood particles
#[derive(Component)]
pub struct BloodParticle {
    pub lifetime: Timer,
}

/// Spawns blood splatter when worm takes damage
pub fn spawn_blood_splatter(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
    hit_direction: Vec3, // Направление от которого прилетел снаряд
) {
    use rand::Rng;
    let mut rng = rand::thread_rng();

    // Spawn 12-15 blood particles
    let particle_count = rng.gen_range(12..16);

    for _ in 0..particle_count {
        let size = rng.gen_range(0.08..0.15);
        let mesh = meshes.add(Sphere::new(size));

        // Green blood color (darker green, slightly transparent)
        let material = materials.add(StandardMaterial {
            base_color: Color::srgba(0.1, 0.5, 0.1, 0.9),
            emissive: LinearRgba::rgb(0.0, 1.0, 0.0), // Slight green glow
            unlit: true,
            alpha_mode: AlphaMode::Blend,
            ..default()
        });

        // Spray pattern: opposite to hit direction + random spread
        let spray_base = -hit_direction.normalize_or(Vec3::Y);
        let random_offset = Vec3::new(
            rng.gen_range(-1.0..1.0),
            rng.gen_range(0.0..1.5), // Mostly upward
            rng.gen_range(-1.0..1.0),
        );
        let velocity = (spray_base + random_offset).normalize() * rng.gen_range(3.0..6.0);

        commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(position),
            GlobalTransform::default(),
            Visibility::Visible,
            // Physics
            RigidBody::Dynamic,
            Collider::ball(size),
            Velocity {
                linvel: velocity,
                angvel: Vec3::ZERO,
            },
            GravityScale(1.5), // Falls faster than dust
            Damping {
                linear_damping: 2.0, // Slows down quickly
                angular_damping: 0.0,
            },
            // Particle marker
            BloodParticle {
                lifetime: Timer::from_seconds(1.5, TimerMode::Once),
            },
            Name::new("BloodParticle"),
        ));
    }

    info!("💚 Spawned blood splatter");
}

/// Fades out and despawns blood particles
pub fn update_blood_particles(
    mut commands: Commands,
    time: Res<Time>,
    mut particles: Query<(
        Entity,
        &mut BloodParticle,
        &MeshMaterial3d<StandardMaterial>,
    )>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, mut particle, material_handle) in &mut particles {
        particle.lifetime.tick(time.delta());

        // Fade out alpha
        if let Some(material) = materials.get_mut(&material_handle.0) {
            let progress = particle.lifetime.fraction();
            material.base_color.set_alpha(0.9 * (1.0 - progress));

            // Fade emissive too
            let glow = 1.0 * (1.0 - progress);
            material.emissive = LinearRgba::rgb(0.0, glow, 0.0);
        }

        // Despawn when done
        if particle.lifetime.finished() {
            commands.entity(entity).despawn();
        }
    }
}
