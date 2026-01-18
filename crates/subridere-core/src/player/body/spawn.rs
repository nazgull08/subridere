use std::f32::consts::FRAC_PI_2;

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

const ARM_COLOR: Color = Color::srgb(0.8, 0.6, 0.5);
const HAND_COLOR: Color = Color::srgb(0.75, 0.55, 0.45);
const HAND_SIZE: Vec3 = Vec3::new(0.35, 0.4, 0.2);

pub fn spawn_first_person_arms(
    commands: &mut Commands,
    camera_entity: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let arm_material = materials.add(StandardMaterial {
        base_color: ARM_COLOR,
        perceptual_roughness: 0.8,
        ..default()
    });

    let hand_material = materials.add(StandardMaterial {
        base_color: HAND_COLOR,
        perceptual_roughness: 0.8,
        ..default()
    });

    let arm_mesh = meshes.add(Cuboid::new(0.3, 0.9, 0.3));
    let hand_mesh = meshes.add(Cuboid::new(HAND_SIZE.x, HAND_SIZE.y, HAND_SIZE.z));

    commands.entity(camera_entity).with_children(|camera| {
        camera
            .spawn((
                FirstPersonArms,
                Transform::default(),
                Visibility::Visible,
                Name::new("FirstPersonArms"),
            ))
            .with_children(|arms_root| {
                spawn_arm(
                    arms_root,
                    ArmSide::Right,
                    Vec3::new(1.0, -0.8, -2.0),
                    &arm_mesh,
                    &hand_mesh,
                    &arm_material,
                    &hand_material,
                );

                spawn_arm(
                    arms_root,
                    ArmSide::Left,
                    Vec3::new(-1.0, -0.8, -2.0),
                    &arm_mesh,
                    &hand_mesh,
                    &arm_material,
                    &hand_material,
                );
            });
    });

    info!("✅ First-person arms spawned");
}

fn spawn_arm(
    parent: &mut ChildSpawnerCommands,
    side: ArmSide,
    base_position: Vec3,
    arm_mesh: &Handle<Mesh>,
    hand_mesh: &Handle<Mesh>,
    arm_material: &Handle<StandardMaterial>,
    hand_material: &Handle<StandardMaterial>,
) {
    let side_name = match side {
        ArmSide::Left => "Left",
        ArmSide::Right => "Right",
    };

    let arm_rotation = Quat::from_rotation_x(FRAC_PI_2);

    parent
        .spawn((
            ArmPart { side },
            Mesh3d(arm_mesh.clone()),
            MeshMaterial3d(arm_material.clone()),
            Transform::from_translation(base_position).with_rotation(arm_rotation),
            Visibility::Visible,
            Name::new(format!("Arm{}", side_name)),
        ))
        .with_children(|arm| {
            // Кисть — она же хитбокс для правой руки
            let mut hand_cmd = arm.spawn((
                HandPart { side },
                Mesh3d(hand_mesh.clone()),
                MeshMaterial3d(hand_material.clone()),
                Transform::from_translation(Vec3::new(0.0, -0.6, 0.0)),
                Visibility::Visible,
                Name::new(format!("Hand{}", side_name)),
            ));

            // Только правая рука — боевая
            if side == ArmSide::Right {
                hand_cmd.insert((
                    MeleeHitbox,
                    WeaponMount { side },
                    Collider::cuboid(HAND_SIZE.x, HAND_SIZE.y, HAND_SIZE.z),
                    Sensor,
                    ActiveEvents::COLLISION_EVENTS,
                ));
                info!("  👊 Right hand has hitbox");
            }
        });
}
