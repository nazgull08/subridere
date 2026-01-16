use bevy::prelude::*;

use crate::app::AppState;
use crate::inventory::component::Equipment;
use crate::items::{EquipmentSlot, ItemId};
use crate::player::component::Player;

/// Проверка условия победы: GoldRing экипирован на палец
pub fn check_victory_condition(
    player_query: Query<&Equipment, With<Player>>,
    mut next_state: ResMut<NextState<AppState>>,
) {
    let Ok(equipment) = player_query.single() else {
        return;
    };

    // Проверяем оба слота для колец
    let left_ring = equipment.get(EquipmentSlot::LeftRing);
    let right_ring = equipment.get(EquipmentSlot::RightRing);

    let has_victory_ring =
        left_ring == Some(ItemId::GoldRing) || right_ring == Some(ItemId::GoldRing);

    if has_victory_ring {
        info!("🏆 Victory! Player equipped the Ring of Power!");
        next_state.set(AppState::Victory);
    }
}
