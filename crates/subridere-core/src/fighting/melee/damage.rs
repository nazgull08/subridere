// crates/subridere-core/src/fighting/melee/damage.rs

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::fighting::components::{ArmCombatState, AttackPhase, PlayerCombatState};
use crate::items::WorldItem;
use crate::player::arm::{ArmSide, MeleeHitbox};
use crate::player::component::Player;

/// Базовая скорость для "среднего" предмета (5kg)
const BASE_VELOCITY: f32 = 5.0;
/// Вертикальная составляющая скорости
const LIFT_VELOCITY: f32 = 2.5;
/// Эталонная масса
const REFERENCE_MASS: f32 = 5.0;
/// Максимальный множитель скорости для лёгких предметов
const MAX_LIGHT_BONUS: f32 = 2.0;
/// Минимальный множитель для тяжёлых предметов
const MIN_HEAVY_FACTOR: f32 = 0.5;

pub fn process_melee_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<&mut PlayerCombatState, With<Player>>,
    hitbox_query: Query<(Entity, &MeleeHitbox)>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    world_items: Query<Entity, With<WorldItem>>,
    mass_query: Query<&AdditionalMassProperties>,
    parent_query: Query<&ChildOf>,
    names: Query<&Name>,
    transforms: Query<&Transform>,
    time: Res<Time>,
) {
    let Ok(mut combat) = player_query.single_mut() else {
        collision_events.clear();
        return;
    };

    // Проверяем есть ли хотя бы одна рука в Active фазе без damage_dealt
    let right_can_hit = can_arm_hit(&combat.right);
    let left_can_hit = can_arm_hit(&combat.left);

    if !right_can_hit && !left_can_hit {
        collision_events.clear();
        return;
    }

    let punch_direction = camera_query
        .single()
        .map(|t| *t.forward())
        .unwrap_or(Vec3::NEG_Z);

    let events: Vec<_> = collision_events.read().collect();

    if events.is_empty() {
        return;
    }

    info!("════════════════════════════════════════════════════");
    info!("🔔 COLLISION in ACTIVE phase");
    info!("   events count: {}", events.len());

    let mut targets: Vec<(Entity, ArmSide)> = Vec::new();

    for (i, event) in events.iter().enumerate() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };

        // Найти какой хитбокс участвовал в коллизии
        let (hitbox_entity, hitbox_side) = {
            let mut found = None;
            for (entity, hitbox) in &hitbox_query {
                if *e1 == entity || *e2 == entity {
                    found = Some((entity, hitbox.side));
                    break;
                }
            }
            match found {
                Some(f) => f,
                None => continue,
            }
        };

        // Проверяем что эта рука может нанести урон
        let arm_can_hit = match hitbox_side {
            ArmSide::Right => right_can_hit,
            ArmSide::Left => left_can_hit,
        };
        if !arm_can_hit {
            continue;
        }

        let target_entity = if *e1 == hitbox_entity { *e2 } else { *e1 };

        let target_name = names
            .get(target_entity)
            .map(|n| n.to_string())
            .unwrap_or_else(|_| format!("{:?}", target_entity));

        let root = find_root(target_entity, &parent_query);
        let root_name = names
            .get(root)
            .map(|n| n.to_string())
            .unwrap_or_else(|_| format!("{:?}", root));

        let side_name = match hitbox_side {
            ArmSide::Right => "RIGHT",
            ArmSide::Left => "LEFT",
        };

        info!(
            "   [{}] {} hand hit: '{}' → root: '{}'",
            i, side_name, target_name, root_name
        );

        if world_items.get(root).is_ok() {
            if !targets.iter().any(|(e, _)| *e == root) {
                targets.push((root, hitbox_side));
                info!("       ✓ added to targets");
            } else {
                info!("       ⏭️ already in targets");
            }
        } else {
            info!("       ❌ not a WorldItem");
        }
    }

    if targets.is_empty() {
        return;
    }

    info!("────────────────────────────────────────────────────");
    info!("💥 HIT! Applying impulse to {} targets", targets.len());

    // Собираем какие руки попали
    let mut right_hit = false;
    let mut left_hit = false;

    for (root, side) in targets.iter() {
        let name = names.get(*root).map(|n| n.as_str()).unwrap_or("?");

        let real_mass = mass_query
            .get(*root)
            .map(|m| match m {
                AdditionalMassProperties::Mass(mass) => *mass,
                AdditionalMassProperties::MassProperties(props) => props.mass,
            })
            .unwrap_or(1.0);

        let velocity_factor = (REFERENCE_MASS / real_mass)
            .sqrt()
            .clamp(MIN_HEAVY_FACTOR, MAX_LIGHT_BONUS);

        let target_velocity = BASE_VELOCITY * velocity_factor;
        let target_lift = LIFT_VELOCITY * velocity_factor;

        let impulse =
            punch_direction * target_velocity * real_mass + Vec3::Y * target_lift * real_mass;

        info!(
            "   '{}': mass={:.1}kg → vel={:.1} m/s",
            name, real_mass, target_velocity,
        );

        commands.entity(*root).insert(ExternalImpulse {
            impulse,
            torque_impulse: Vec3::ZERO,
        });

        match side {
            ArmSide::Right => right_hit = true,
            ArmSide::Left => left_hit = true,
        }
    }

    info!("════════════════════════════════════════════════════");

    // Помечаем damage_dealt на руках которые попали
    if right_hit {
        mark_damage_dealt(&mut combat.right);
    }
    if left_hit {
        mark_damage_dealt(&mut combat.left);
    }
}

/// Проверяет может ли рука нанести урон (Active фаза, урон ещё не нанесён)
fn can_arm_hit(arm: &ArmCombatState) -> bool {
    matches!(
        arm,
        ArmCombatState::Attacking {
            phase: AttackPhase::Active,
            damage_dealt: false,
            ..
        }
    )
}

/// Помечает что рука нанесла урон
fn mark_damage_dealt(arm: &mut ArmCombatState) {
    if let ArmCombatState::Attacking { damage_dealt, .. } = arm {
        *damage_dealt = true;
    }
}

fn find_root(entity: Entity, parent_query: &Query<&ChildOf>) -> Entity {
    let mut current = entity;
    while let Ok(child_of) = parent_query.get(current) {
        current = child_of.parent();
    }
    current
}
