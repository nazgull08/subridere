pub mod input;
pub mod interaction;
pub mod sync;
pub mod ui;

// Re-exports
pub use input::{inventory_closed, manage_cursor_on_inventory, toggle_inventory_input};
pub use interaction::{
    SelectedSlot, handle_equip_slot_click, handle_equip_slot_hover, handle_slot_click,
    handle_slot_hover, update_selected_slot_visual,
};
pub use sync::{sync_equipment_to_ui, sync_inventory_to_ui, sync_stats_to_ui};
pub use ui::{
    EquipmentSlotType, EquipmentSlotUI, InventorySlotUI, InventoryUI, SlotIcon, SlotQuantity,
    StatsHpText, StatsMpText, StatsSpText, despawn_inventory_ui, spawn_inventory_ui,
};
