// fighting/melee/damage.rs

use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::fighting::components::{CombatState, PlayerCombatState};
use crate::items::WorldItem;
use crate::player::body::MeleeHitbox;
use crate::player::component::Player;

use super::debug::PhysicsDebugTracker;

const PUNCH_FORCE: f32 = 5.0;
const PUNCH_LIFT: f32 = 2.0;
const MIN_EFFECTIVE_MASS: f32 = 5.0;

pub fn process_melee_collisions(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    mut player_query: Query<&mut PlayerCombatState, With<Player>>,
    hitbox_query: Query<Entity, With<MeleeHitbox>>,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    world_items: Query<Entity, With<WorldItem>>,
    mass_query: Query<&ColliderMassProperties>,
    parent_query: Query<&ChildOf>,
    names: Query<&Name>,
    transforms: Query<&Transform>,
    time: Res<Time>,
) {
    let Ok(mut combat) = player_query.single_mut() else {
        collision_events.clear();
        return;
    };

    let CombatState::Attacking {
        ref mut damage_dealt,
        timer,
        duration,
    } = combat.state
    else {
        collision_events.clear();
        return;
    };

    if *damage_dealt {
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
        info!(
            "🔔 COLLISION FRAME | attack progress: {:.0}%",
            (timer / duration) * 100.0
        );
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
    info!("📦 APPLYING IMPULSE to {} targets:", targets.len());

    for root in targets.iter() {
        let name = names.get(*root).map(|n| n.as_str()).unwrap_or("?");

        let real_mass = mass_query
            .get(*root)
            .map(|m| match m {
                ColliderMassProperties::Mass(m) => *m,
                ColliderMassProperties::Density(d) => *d,
                ColliderMassProperties::MassProperties(props) => props.mass,
            })
            .unwrap_or(1.0);

        let effective_mass = real_mass.max(MIN_EFFECTIVE_MASS);

        let impulse = punch_direction * PUNCH_FORCE + Vec3::Y * PUNCH_LIFT;
        let resulting_velocity = impulse / effective_mass;

        info!(
            "   '{}': mass={:.1}kg (eff={:.1}kg) → impulse=[{:.1},{:.1},{:.1}] → {:.1} m/s",
            name,
            real_mass,
            effective_mass,
            impulse.x,
            impulse.y,
            impulse.z,
            resulting_velocity.length()
        );

        commands.entity(*root).insert(ExternalImpulse {
            impulse,
            torque_impulse: Vec3::ZERO,
        });

        // Добавляем трекер сразу здесь
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

    *damage_dealt = true;
}

fn find_root(entity: Entity, parent_query: &Query<&ChildOf>) -> Entity {
    let mut current = entity;
    while let Ok(child_of) = parent_query.get(current) {
        current = child_of.parent();
    }
    current
}
