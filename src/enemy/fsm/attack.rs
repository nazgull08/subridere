use crate::{
    enemy::component::*,
    player::component::Player,
    stats::damage::component::{Damage, DamageType, HasDealtDamage},
};
use bevy::prelude::*;

pub fn enemy_melee_attack_system(
    mut commands: Commands,
    mut enemies: Query<
        (Entity, &Transform, &EnemyState, &StateTimer),
        (With<Enemy>, Without<HasDealtDamage>),
    >,
    players: Query<(Entity, &Transform), With<Player>>,
) {
    for (enemy_ent, enemy_tf, state, timer) in &mut enemies {
        if let EnemyState::Attack(EnemyAttackState::Bite) = state {
            let t = timer.0.elapsed_secs();
            if !(0.08..0.12).contains(&t) {
                continue; // вне окна удара
            }

            for (player_ent, player_tf) in &players {
                if enemy_tf.translation.distance(player_tf.translation) < 2.5 {
                    // 1. Наносим урон
                    commands.entity(player_ent).insert(Damage {
                        amount: 15.0,
                        damage_type: DamageType::Physical,
                    });

                    // 2. Ставим флаг, чтобы не повторять урон
                    commands.entity(enemy_ent).insert(HasDealtDamage);

                    tracing::info!(?enemy_ent, "DEALT damage at {t:.2}s");
                    break;
                }
            }
        }
    }
}
