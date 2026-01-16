use bevy::prelude::*;
use bevy_ui_actions::UiActionsPlugin;

use crate::app::AppState;

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
            .init_resource::<GameMenuActiveTab>()
            .add_systems(
                Update,
                toggle_game_menu_input.run_if(in_state(AppState::InGame)),
            )
            .add_systems(OnEnter(GameMenuState::Open), spawn_game_menu)
            .add_systems(
                OnExit(GameMenuState::Open),
                (save_active_tab, despawn_game_menu).chain(),
            )
            .add_systems(
                Update,
                (
                    sync_inventory_slots,
                    sync_equipment_slots,
                    sync_drag_visual,
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
