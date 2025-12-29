use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use crate::player::component::Player;
use crate::stats::damage::component::{Damage, DamageType};
use super::components::WormHead;

#[derive(Component)]
pub struct WormDamageCooldown {
    pub timer: Timer,
}

impl Default for WormDamageCooldown {
    fn default() -> Self {
        Self {
            timer: Timer::from_seconds(1.0, TimerMode::Once),
        }
    }
}

/// Detects collisions between worm head and player
pub fn worm_collision_damage_system(
    mut commands: Commands,
    mut collision_events: EventReader<CollisionEvent>,
    worm_heads: Query<(Entity, Option<&WormDamageCooldown>), With<WormHead>>,
    players: Query<Entity, With<Player>>,
) {
    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let (worm_entity, player_entity) = 
                if worm_heads.get(*e1).is_ok() && players.get(*e2).is_ok() {
                    (*e1, *e2)
                } else if worm_heads.get(*e2).is_ok() && players.get(*e1).is_ok() {
                    (*e2, *e1)
                } else {
                    continue;
                };

            if let Ok((worm_e, cooldown)) = worm_heads.get(worm_entity) {
                if cooldown.is_some() {
                    continue;
                }

                commands.entity(player_entity).insert(Damage {
                    amount: 15.0,
                    damage_type: DamageType::Physical,
                });

                commands.entity(worm_e).insert(WormDamageCooldown::default());

                info!("🦷 Worm bit player! 15 damage");
            }
        }
    }
}

pub fn worm_damage_cooldown_system(
    mut commands: Commands,
    time: Res<Time>,
    mut worms: Query<(Entity, &mut WormDamageCooldown), With<WormHead>>,
) {
    for (entity, mut cooldown) in &mut worms {
        cooldown.timer.tick(time.delta());
        
        if cooldown.timer.finished() {
            commands.entity(entity).remove::<WormDamageCooldown>();
        }
    }
}
