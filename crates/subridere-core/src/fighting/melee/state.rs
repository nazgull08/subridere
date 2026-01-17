use bevy::prelude::*;

use crate::fighting::components::{CombatState, PlayerCombatState};
use crate::fighting::melee::MeleeAttackIntent;
use crate::player::component::Player;

/// Длительность атаки
const ATTACK_DURATION: f32 = 0.4;
/// Длительность cooldown
const COOLDOWN_DURATION: f32 = 0.3;

/// Система обработки боевых состояний
pub fn process_combat_state(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &mut PlayerCombatState, Option<&MeleeAttackIntent>), With<Player>>,
) {
    let dt = time.delta_secs();

    for (entity, mut combat, maybe_intent) in &mut query {
        // Убираем intent после обработки
        let has_intent = maybe_intent.is_some();
        if has_intent {
            commands.entity(entity).remove::<MeleeAttackIntent>();
        }

        match &mut combat.state {
            CombatState::Ready => {
                if has_intent {
                    info!("⚔️ Ready → Attacking");
                    combat.state = CombatState::Attacking {
                        timer: 0.0,
                        duration: ATTACK_DURATION,
                        damage_dealt: false,
                    };
                }
            }

            CombatState::Attacking {
                timer, duration, ..
            } => {
                *timer += dt;

                if *timer >= *duration {
                    info!("⚔️ Attacking → Cooldown");
                    combat.state = CombatState::Cooldown {
                        remaining: COOLDOWN_DURATION,
                    };
                }
            }

            CombatState::Cooldown { remaining } => {
                *remaining -= dt;

                if *remaining <= 0.0 {
                    info!("⚔️ Cooldown → Ready");
                    combat.state = CombatState::Ready;
                }
            }
        }
    }
}
