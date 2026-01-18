// fighting/melee/damage.rs

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::fighting::components::{AttackPhase, CombatState, PlayerCombatState};
use crate::items::WorldItem;
use crate::player::body::MeleeHitbox;
use crate::player::component::Player;

use super::debug::PhysicsDebugTracker;

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
    hitbox_query: Query<Entity, With<MeleeHitbox>>,
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

    // Проверяем Active фазу и damage_dealt
    let damage_dealt = match &combat.state {
        CombatState::Attacking {
            phase: AttackPhase::Active,
            damage_dealt,
            ..
        } => *damage_dealt,
        _ => {
            collision_events.clear();
            return;
        }
    };

    if damage_dealt {
        collision_events.clear();
        return;
    }

    let Ok(hitbox_entity) = hitbox_query.single() else {
        collision_events.clear();
        return;
    };

    let punch_direction = camera_query
        .single()
        .map(|t| *t.forward())
        .unwrap_or(Vec3::NEG_Z);

    let events: Vec<_> = collision_events.read().collect();

    if !events.is_empty() {
        info!("════════════════════════════════════════════════════");
        info!("🔔 COLLISION in ACTIVE phase");
        info!("   events count: {}", events.len());
    }

    let mut targets: Vec<Entity> = Vec::new();

    for (i, event) in events.iter().enumerate() {
        let CollisionEvent::Started(e1, e2, _) = event else {
            continue;
        };

        let target_entity = if *e1 == hitbox_entity {
            *e2
        } else if *e2 == hitbox_entity {
            *e1
        } else {
            continue;
        };

        let target_name = names
            .get(target_entity)
            .map(|n| n.to_string())
            .unwrap_or_else(|_| format!("{:?}", target_entity));

        let root = find_root(target_entity, &parent_query);
        let root_name = names
            .get(root)
            .map(|n| n.to_string())
            .unwrap_or_else(|_| format!("{:?}", root));

        info!("   [{}] hit: '{}' → root: '{}'", i, target_name, root_name);

        if world_items.get(root).is_ok() {
            if !targets.contains(&root) {
                targets.push(root);
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

    for root in targets.iter() {
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

        let start_pos = transforms
            .get(*root)
            .map(|t| t.translation)
            .unwrap_or(Vec3::ZERO);
        commands.entity(*root).insert(PhysicsDebugTracker {
            start_time: time.elapsed_secs(),
            start_pos,
            max_speed: 0.0,
            item_name: name.to_string(),
        });
    }

    info!("════════════════════════════════════════════════════");

    // Помечаем что урон нанесён
    if let CombatState::Attacking { damage_dealt, .. } = &mut combat.state {
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
