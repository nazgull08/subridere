use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::enemies::components::Enemy;
use crate::fighting::components::{CombatState, PlayerCombatState};
use crate::fighting::events::MeleeHitEvent;
use crate::player::component::Player;
use crate::stats::computed::ComputedStats;
use crate::stats::damage::{Damage, DamageType};

/// Дистанция удара
const MELEE_RANGE: f32 = 3.0;
/// Окно нанесения урона (начало и конец в процентах от длительности)
const DAMAGE_WINDOW_START: f32 = 0.2;
const DAMAGE_WINDOW_END: f32 = 0.5;

/// Система: raycast в damage window, наносит урон врагам
pub fn apply_melee_damage(
    mut commands: Commands,
    camera_query: Query<&GlobalTransform, With<Camera>>,
    mut player_query: Query<(Entity, &mut PlayerCombatState, &ComputedStats), With<Player>>,
    rapier_context: ReadRapierContext,
    enemies: Query<Entity, With<Enemy>>,
    parent_query: Query<&ChildOf>,
    mut hit_events: EventWriter<MeleeHitEvent>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let Ok(rapier_context) = rapier_context.single() else {
        return;
    };

    for (player_entity, mut combat, stats) in &mut player_query {
        // Проверяем что мы в состоянии атаки
        let CombatState::Attacking {
            timer,
            duration,
            ref mut damage_dealt,
        } = combat.state
        else {
            continue;
        };

        // Уже нанесли урон в этой атаке
        if *damage_dealt {
            continue;
        }

        // Проверяем damage window
        let progress = timer / duration;
        if progress < DAMAGE_WINDOW_START || progress > DAMAGE_WINDOW_END {
            continue;
        }

        // Raycast вперёд от камеры
        let ray_dir = camera_transform.forward();
        let ray_origin = camera_transform.translation() + *ray_dir * 0.5;

        let Some((hit_entity, _distance)) = rapier_context.cast_ray(
            ray_origin,
            *ray_dir,
            MELEE_RANGE,
            true,
            QueryFilter::default().exclude_collider(player_entity),
        ) else {
            continue;
        };

        // Проверяем попали ли во врага (или его child)
        let enemy_entity = find_enemy_entity(hit_entity, &enemies, &parent_query);

        let Some(target) = enemy_entity else {
            continue;
        };

        // Наносим урон
        let damage_amount = stats.melee_damage;

        commands.entity(target).insert(Damage {
            amount: damage_amount,
            damage_type: DamageType::Physical,
        });

        // Помечаем что урон нанесён
        *damage_dealt = true;

        // Отправляем событие для audio/particles
        hit_events.write(MeleeHitEvent {
            target,
            damage: damage_amount,
        });

        info!("💥 Melee hit! {} damage to {:?}", damage_amount, target);
    }
}

/// Найти Enemy entity (проверяя parents)
fn find_enemy_entity(
    hit_entity: Entity,
    enemies: &Query<Entity, With<Enemy>>,
    parent_query: &Query<&ChildOf>,
) -> Option<Entity> {
    // Прямое попадание
    if enemies.contains(hit_entity) {
        return Some(hit_entity);
    }

    // Проверяем parent (для worm segments и т.д.)
    if let Ok(child_of) = parent_query.get(hit_entity) {
        let parent = child_of.parent();
        if enemies.contains(parent) {
            return Some(parent);
        }
    }

    None
}
