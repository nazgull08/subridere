// fighting/melee/state.rs

use bevy::prelude::*;

use crate::fighting::components::{
    AttackPhase, AttackTimings, CombatState, CurrentAttackTimings, PlayerCombatState,
};
use crate::fighting::melee::MeleeAttackIntent;
use crate::player::component::Player;

/// Hitstop duration (секунды)
const HITSTOP_DURATION: f32 = 0.07;

/// Система обработки боевых состояний (Souls-like phases)
pub fn process_combat_state(
    mut commands: Commands,
    time: Res<Time>,
    timings: Res<CurrentAttackTimings>,
    mut query: Query<(Entity, &mut PlayerCombatState, Option<&MeleeAttackIntent>), With<Player>>,
) {
    let dt = time.delta_secs();
    let timings = &timings.0;

    for (entity, mut combat, maybe_intent) in &mut query {
        // Убираем intent после обработки
        let has_intent = maybe_intent.is_some();
        if has_intent {
            commands.entity(entity).remove::<MeleeAttackIntent>();
        }

        match &mut combat.state {
            CombatState::Ready => {
                if has_intent {
                    info!("⚔️ ATTACK START → Windup");
                    combat.state = CombatState::Attacking {
                        phase: AttackPhase::Windup,
                        phase_timer: 0.0,
                        damage_dealt: false,
                    };
                }
            }

            CombatState::Attacking {
                phase,
                phase_timer,
                damage_dealt,
            } => {
                *phase_timer += dt;

                match phase {
                    AttackPhase::Windup => {
                        if *phase_timer >= timings.windup {
                            info!("⚔️ Windup → Active (hitbox ON)");
                            *phase = AttackPhase::Active;
                            *phase_timer = 0.0;
                        }
                    }

                    AttackPhase::Active => {
                        if *phase_timer >= timings.active {
                            info!("⚔️ Active → Recovery (hitbox OFF)");
                            *phase = AttackPhase::Recovery;
                            *phase_timer = 0.0;
                        }
                    }

                    AttackPhase::Recovery => {
                        if *phase_timer >= timings.recovery {
                            info!("⚔️ Recovery → Ready");
                            combat.state = CombatState::Ready;
                        }
                    }
                }
            }

            CombatState::Hitstop {
                remaining,
                return_phase,
                return_timer,
            } => {
                *remaining -= dt;

                if *remaining <= 0.0 {
                    info!("⚔️ Hitstop END → {:?}", return_phase);
                    combat.state = CombatState::Attacking {
                        phase: *return_phase,
                        phase_timer: *return_timer,
                        damage_dealt: true, // Уже нанесли урон
                    };
                }
            }
        }
    }
}

/// Запустить hitstop (вызывается из damage.rs при попадании)
pub fn trigger_hitstop(combat: &mut PlayerCombatState) {
    if let CombatState::Attacking {
        phase, phase_timer, ..
    } = &combat.state
    {
        info!("💥 HITSTOP triggered!");
        combat.state = CombatState::Hitstop {
            remaining: HITSTOP_DURATION,
            return_phase: *phase,
            return_timer: *phase_timer,
        };
    }
}

/// Проверка: находимся ли в активной фазе (можно наносить урон)
pub fn is_in_active_phase(combat: &PlayerCombatState) -> bool {
    matches!(
        combat.state,
        CombatState::Attacking {
            phase: AttackPhase::Active,
            ..
        }
    )
}
