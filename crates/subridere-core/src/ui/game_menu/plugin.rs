use bevy::prelude::*;
use bevy::window::CursorGrabMode;
use bevy_ui_actions::UiActionsPlugin;

use super::spawn::{despawn_game_menu, save_active_tab, spawn_game_menu};
use super::state::{GameMenuActiveTab, GameMenuState, game_menu_open};
use super::tabs::character::sync::{
    sync_attributes_display, sync_level_display, sync_stats_display,
};
use super::tabs::inventory::sync::{sync_drag_visual, sync_equipment_slots, sync_inventory_slots};

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiActionsPlugin)
            .init_state::<GameMenuState>()
            .init_resource::<GameMenuActiveTab>() // NEW
            .add_systems(Update, toggle_game_menu_input)
            .add_systems(OnEnter(GameMenuState::Open), (spawn_game_menu, show_cursor))
            .add_systems(
                OnExit(GameMenuState::Open),
                (save_active_tab, despawn_game_menu, hide_cursor).chain(), // save_active_tab first
            )
            .add_systems(
                Update,
                (
                    // Inventory sync
                    sync_inventory_slots,
                    sync_equipment_slots,
                    sync_drag_visual,
                    // Character sync
                    sync_level_display,
                    sync_attributes_display,
                    sync_stats_display,
                )
                    .run_if(game_menu_open),
            );

        info!("✅ Game Menu plugin initialized");
    }
}

fn toggle_game_menu_input(
    keys: Res<ButtonInput<KeyCode>>,
    state: Res<State<GameMenuState>>,
    mut next_state: ResMut<NextState<GameMenuState>>,
) {
    if keys.just_pressed(KeyCode::Tab) {
        match state.get() {
            GameMenuState::Closed => {
                info!("🎮 Opening game menu");
                next_state.set(GameMenuState::Open);
            }
            GameMenuState::Open => {
                info!("🎮 Closing game menu");
                next_state.set(GameMenuState::Closed);
            }
        }
    }

    if keys.just_pressed(KeyCode::Escape) && *state.get() == GameMenuState::Open {
        info!("🎮 Closing game menu (ESC)");
        next_state.set(GameMenuState::Closed);
    }
}

fn show_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::None;
        window.cursor_options.visible = true;
    }
}

fn hide_cursor(mut windows: Query<&mut Window>) {
    if let Ok(mut window) = windows.single_mut() {
        window.cursor_options.grab_mode = CursorGrabMode::Confined;
        window.cursor_options.visible = false;
    }
}
