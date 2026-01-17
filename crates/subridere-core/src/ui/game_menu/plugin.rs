use bevy::prelude::*;
use bevy_ui_actions::{TooltipSet, TooltipStyle, UiActionsPlugin};

use crate::app::AppState;
use crate::ui::game_menu::tabs::inventory::SelectedSlot;

use super::spawn::{despawn_game_menu, save_active_tab, spawn_game_menu};
use super::state::{GameMenuActiveTab, GameMenuState, game_menu_open};
use super::tabs::character::sync::{
    sync_attributes_display, sync_level_display, sync_stats_display,
};
use super::tabs::inventory::sync::{sync_drag_visual, sync_equipment_slots, sync_inventory_slots};
use super::tabs::inventory::tooltip::{clear_tooltip_on_unhover, update_hovered_tooltip};

pub struct GameMenuPlugin;

impl Plugin for GameMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(UiActionsPlugin)
            .init_state::<GameMenuState>()
            .init_resource::<GameMenuActiveTab>()
            .init_resource::<SelectedSlot>()
            // Input handling
            .add_systems(
                Update,
                toggle_game_menu_input.run_if(in_state(AppState::InGame)),
            )
            // Initialize tooltip style with font on startup
            .add_systems(Startup, init_tooltip_style)
            // Menu lifecycle
            .add_systems(OnEnter(GameMenuState::Open), spawn_game_menu)
            .add_systems(
                OnExit(GameMenuState::Open),
                (save_active_tab, despawn_game_menu).chain(),
            )
            // Menu systems (only when open)
            .add_systems(
                Update,
                (
                    // Inventory slot visuals
                    sync_inventory_slots,
                    sync_equipment_slots,
                    sync_drag_visual,
                    // Character tab
                    sync_level_display,
                    sync_attributes_display,
                    sync_stats_display,
                )
                    .run_if(game_menu_open),
            )
            // Tooltip generation — must run in TooltipSet::GenerateContent
            // to be after DetectHover but before Display
            .add_systems(
                Update,
                (update_hovered_tooltip, clear_tooltip_on_unhover)
                    .in_set(TooltipSet::GenerateContent)
                    .run_if(game_menu_open),
            );

        info!("✅ Game Menu plugin initialized");
    }
}

/// Initialize tooltip style with the game's font
fn init_tooltip_style(mut style: ResMut<TooltipStyle>, asset_server: Res<AssetServer>) {
    let font = asset_server.load("fonts/dogica.ttf");
    style.font = Some(font);
    info!("✅ Tooltip style initialized with font");
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
