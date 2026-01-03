pub mod actions;
pub mod context_menu;
pub mod input;
pub mod interaction;
pub mod sync;
pub mod ui;

// Re-exports
pub use input::{inventory_closed, manage_cursor_on_inventory, toggle_inventory_input};
pub use interaction::{
    SelectedSlot, 
    handle_equip_slot_click, 
    handle_equip_slot_hover, 
    handle_slot_click,
    handle_slot_hover, 
    update_selected_slot_visual,
    detect_inventory_right_click,
    detect_equipment_right_click,
    close_menu_on_outside_click,
    force_close_menu_on_inventory_exit
};
pub use sync::{sync_equipment_to_ui, sync_inventory_to_ui, sync_stats_to_ui};
pub use ui::{
    EquipmentSlotType, EquipmentSlotUI, InventorySlotUI, InventoryUI, SlotIcon, SlotQuantity,
    StatsHpText, StatsMpText, StatsSpText, despawn_inventory_ui, spawn_inventory_ui,
};
pub use actions::process_item_actions;
pub use context_menu::{
    ContextMenuState,
    spawn_context_menu,
    despawn_context_menu,
    handle_menu_button_hover,
    handle_menu_button_clicks,
};
