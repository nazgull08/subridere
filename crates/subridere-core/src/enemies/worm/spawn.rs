use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use block_bodies_core::serialization::BlockBodyFile;

use crate::{
    enemies::{
        components::Enemy,
        worm::components::{Worm, WormAI, WormHead, WormSegment, WormState},
    },
    stats::health::component::Health,
};

/// Physics - HEAD (active, controlled)
const HEAD_MASS: f32 = 4.0;
const HEAD_LINEAR_DAMPING: f32 = 4.0;
const HEAD_ANGULAR_DAMPING: f32 = 1.5; // ✅ СНИЖЕН (было 3.0) - легче повернуть

/// Physics - BODY (passive, follows)
const BODY_MASS: f32 = 0.5;
const BODY_LINEAR_DAMPING: f32 = 1.5;
const BODY_ANGULAR_DAMPING: f32 = 1.0;

pub fn spawn_worm(
    commands: &mut Commands,
    meshes: &mut ResMut<Assets<Mesh>>,
    materials: &mut ResMut<Assets<StandardMaterial>>,
    position: Vec3,
) -> Entity {
    let model_path = "./assets/models/worm.ron";
    info!("🐛 Spawning worm with forward=+X orientation");

    let body_file = BlockBodyFile::load_from_file(model_path).expect("Failed to load worm.ron");
    let body = body_file.to_body().expect("Failed to parse worm body");

    // Materials
    let red_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.8, 0.2, 0.2),
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });

    let green_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.2, 0.7, 0.3),
        metallic: 0.1,
        perceptual_roughness: 0.8,
        ..default()
    });

    let worm_id = commands
        .spawn((
            Enemy,
            Worm::default(),
            Health::new(50.0, 0.0),
            Transform::from_translation(position),
            GlobalTransform::default(),
            Visibility::Visible,
            Name::new("WormRoot"),
        ))
        .id();

    let mut segment_entities = Vec::new();

    for (_part_id, part) in body.parts.iter() {
        let material = if part.name == "Head" || part.name == "Tail" {
            red_material.clone()
        } else {
            green_material.clone()
        };

        let segment_index = match part.name.as_str() {
            "Head" => 0,
            "Seg1" => 1,
            "Seg2" => 2,
            "Seg3" => 3,
            "Seg4" => 4,
            "Tail" => 5,
            _ => 0,
        };

        let spacing = 0.8;
        let world_pos = position + Vec3::new(segment_index as f32 * spacing, 0.0, 0.0);

        let mesh = meshes.add(Cuboid::new(part.size.x, part.size.y, part.size.z));

        let is_head = part.name == "Head";
        let (mass, lin_damp, ang_damp) = if is_head {
            (HEAD_MASS, HEAD_LINEAR_DAMPING, HEAD_ANGULAR_DAMPING)
        } else {
            (BODY_MASS, BODY_LINEAR_DAMPING, BODY_ANGULAR_DAMPING)
        };

        // ✅ CRITICAL: Rotate cuboid so that +X is "forward" direction
        // By default Bevy cuboid's longest axis might not align with movement
        // We spawn along +X axis, so we want +X to be forward
        let rotation = Quat::from_rotation_y(0.0); // Identity - +X is already forward

        let mut segment_cmd = commands.spawn((
            Mesh3d(mesh),
            MeshMaterial3d(material),
            Transform::from_translation(world_pos).with_rotation(rotation),
            GlobalTransform::default(),
            Visibility::Visible,
            RigidBody::Dynamic,
            Collider::cuboid(part.size.x * 0.5, part.size.y * 0.5, part.size.z * 0.5),
            ColliderMassProperties::Density(mass),
            Damping {
                linear_damping: lin_damp,
                angular_damping: ang_damp,
            },
            Velocity::default(),
            Ccd::enabled(),
            Name::new(format!("Worm_{}", part.name)),
        ));

        if is_head {
            segment_cmd.insert((
                WormHead { worm_root: worm_id },
                WormAI::default(),
                WormState::default(),
                ExternalForce::default(),
                ExternalImpulse::default(),
                ActiveEvents::COLLISION_EVENTS,
            ));

            info!(
                "  ✓ HEAD at {:?} | ang_damp={} (easy to turn)",
                world_pos, ang_damp
            );
        } else {
            segment_cmd.insert(WormSegment {
                worm_root: worm_id,
                index: segment_index,
            });
        }

        segment_entities.push(segment_cmd.id());
    }

    // Create flexible joints
    for i in 0..segment_entities.len() - 1 {
        let parent = segment_entities[i];
        let child = segment_entities[i + 1];

        let spacing = 0.8;
        let anchor_offset = spacing * 0.5;
        let anchor_on_parent = Vec3::new(anchor_offset, 0.0, 0.0);
        let anchor_on_child = Vec3::new(-anchor_offset, 0.0, 0.0);

        let joint = SphericalJointBuilder::new()
            .local_anchor1(anchor_on_parent)
            .local_anchor2(anchor_on_child)
            .limits(JointAxis::AngX, [-1.0, 1.0])
            .limits(JointAxis::AngY, [-1.2, 1.2]) // Wide for side-to-side flex
            .limits(JointAxis::AngZ, [-1.0, 1.0])
            .build();

        commands
            .entity(child)
            .insert(ImpulseJoint::new(parent, joint));
    }

    info!(
        "✅ Spawned worm with 1 head + {} body segments",
        segment_entities.len() - 1
    );
    worm_id
}
