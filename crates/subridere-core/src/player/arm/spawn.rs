// crates/subridere-core/src/player/arm/spawn.rs
//
// Спавн IK-управляемых рук игрока

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use super::components::*;

/// Спавнит руку игрока как child камеры
pub fn spawn_player_arm(
    commands: &mut Commands,
    camera_entity: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
    config: &ArmConfig,
    side: ArmSide,
) -> Entity {
    let shoulder_offset = match side {
        ArmSide::Right => config.shoulder_offset_right,
        ArmSide::Left => config.shoulder_offset_left,
    };

    let side_name = match side {
        ArmSide::Right => "R",
        ArmSide::Left => "L",
    };

    // Материал для руки (skin tone)
    let arm_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.76, 0.60, 0.42),
        metallic: 0.0,
        perceptual_roughness: 0.8,
        ..default()
    });

    // === Shoulder (точка крепления, невидимый) ===
    let shoulder_entity = commands
        .spawn((
            Shoulder { side },
            Transform::from_translation(shoulder_offset),
            GlobalTransform::default(),
            Visibility::Inherited,
            Name::new(format!("Shoulder_{}", side_name)),
        ))
        .id();

    commands.entity(camera_entity).add_child(shoulder_entity);

    // === Upper Arm ===
    let upper_arm_mesh = meshes.add(Cuboid::new(
        config.upper_arm_size.x,
        config.upper_arm_size.y,
        config.upper_arm_size.z,
    ));

    let upper_arm_entity = commands
        .spawn((
            UpperArm {
                side,
                length: config.upper_arm_length,
            },
            Mesh3d(upper_arm_mesh),
            MeshMaterial3d(arm_material.clone()),
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Inherited,
            Name::new(format!("UpperArm_{}", side_name)),
        ))
        .id();

    commands.entity(shoulder_entity).add_child(upper_arm_entity);

    // === Forearm ===
    let forearm_mesh = meshes.add(Cuboid::new(
        config.forearm_size.x,
        config.forearm_size.y,
        config.forearm_size.z,
    ));

    let forearm_entity = commands
        .spawn((
            Forearm {
                side,
                length: config.forearm_length,
            },
            Mesh3d(forearm_mesh),
            MeshMaterial3d(arm_material.clone()),
            Transform::from_translation(Vec3::new(0.0, 0.0, -config.upper_arm_length)),
            GlobalTransform::default(),
            Visibility::Inherited,
            Name::new(format!("Forearm_{}", side_name)),
        ))
        .id();

    commands.entity(upper_arm_entity).add_child(forearm_entity);

    // === Hand ===
    let hand_mesh = meshes.add(Cuboid::new(
        config.hand_size.x,
        config.hand_size.y,
        config.hand_size.z,
    ));

    let hand_entity = commands
        .spawn((
            Hand { side },
            WeaponSocket { side },
            Mesh3d(hand_mesh),
            MeshMaterial3d(arm_material),
            Transform::from_translation(Vec3::new(0.0, 0.0, -config.forearm_length)),
            GlobalTransform::default(),
            Visibility::Inherited,
            Name::new(format!("Hand_{}", side_name)),
            // Обе руки имеют хитбокс для melee
            MeleeHitbox { side },
            Collider::cuboid(
                config.hand_size.x * 1.5,
                config.hand_size.y * 1.5,
                config.hand_size.z * 1.5,
            ),
            Sensor,
            ActiveEvents::COLLISION_EVENTS,
        ))
        .id();

    commands.entity(forearm_entity).add_child(hand_entity);

    info!("  👊 {} hand has MeleeHitbox", side_name);

    // === IK Target ===
    let ik_target = match side {
        ArmSide::Right => IkTarget::right(),
        ArmSide::Left => IkTarget::left(),
    };

    let ik_target_entity = commands
        .spawn((
            ik_target,
            Transform::default(),
            GlobalTransform::default(),
            Visibility::Hidden,
            Name::new(format!("IkTarget_{}", side_name)),
        ))
        .id();

    commands.entity(camera_entity).add_child(ik_target_entity);

    info!("✅ Spawned {:?} arm with IK", side);

    shoulder_entity
}

/// Спавнит обе руки игрока (вызывается из game_init/player.rs)
pub fn spawn_player_arms(
    commands: &mut Commands,
    camera_entity: Entity,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<StandardMaterial>,
) {
    let config = ArmConfig::default();

    // Правая рука
    spawn_player_arm(
        commands,
        camera_entity,
        meshes,
        materials,
        &config,
        ArmSide::Right,
    );

    // Левая рука
    spawn_player_arm(
        commands,
        camera_entity,
        meshes,
        materials,
        &config,
        ArmSide::Left,
    );

    info!("✅ Both arms spawned");
}
