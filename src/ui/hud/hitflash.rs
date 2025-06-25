use bevy::ecs::event::Event;
use bevy::prelude::*;


#[derive(Event)]
pub struct HitFlashEvent;


#[derive(Component)]
pub struct HitOverlay {
    pub timer: Timer,
}


pub fn spawn_hit_overlay(
    mut commands: Commands,
    mut evr: EventReader<HitFlashEvent>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    cameras: Query<&GlobalTransform, With<Camera3d>>,
) {
    let Ok(cam_tf) = cameras.single() else { return; };

    let mesh = meshes.add(Plane3d::default().mesh().size(2.0, 2.0));
    let material = materials.add(StandardMaterial {
        base_color: Color::srgba(1.0, 0.0, 0.0, 0.4),
        unlit: true,
        alpha_mode: AlphaMode::Blend,
        ..default()
    });

    let cam_pos = cam_tf.translation();
    let cam_forward = cam_tf.forward();

    for _ in evr.read() {
        commands.spawn((
            (   Mesh3d(mesh.clone()),
                MeshMaterial3d(material.clone()),
                Transform::from_translation(cam_pos + cam_forward * 0.5)
                    .looking_at(cam_pos, Vec3::Y)
                    .with_scale(Vec3::splat(0.5)), // тонкий экран
            ),
            HitOverlay {
                timer: Timer::from_seconds(0.4, TimerMode::Once),
            },
            Name::new("3DHitOverlay"),
        ));
    }
}

pub fn update_hit_overlay(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut HitOverlay)>,
) {
    for (e, mut overlay) in &mut q {
        overlay.timer.tick(time.delta());


        if overlay.timer.finished() {
            commands.entity(e).despawn();
        }
    }
}
