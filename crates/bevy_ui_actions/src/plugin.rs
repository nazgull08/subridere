use crate::drag::{DragGhostStyle, DragState};
use crate::tooltip::{TooltipState, TooltipStyle};
use crate::systems::{
    handle_clicks,
    handle_drag_abort,
    handle_drag_end,
    handle_drag_move,
    handle_drag_start,
    handle_hover_actions,
    handle_hover_exit_actions,
    handle_press_actions,
    handle_right_clicks,
    hide_tooltip,
    show_tooltip,
    update_ghost_position,
    update_interactive_visuals,
    update_tooltip_hover,
};
use crate::style::ButtonStyle;
use bevy::prelude::*;

pub struct UiActionsPlugin;

impl Plugin for UiActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonStyle>()
            .init_resource::<DragState>()
            .init_resource::<DragGhostStyle>()
            .init_resource::<TooltipState>()
            .init_resource::<TooltipStyle>()
            .add_systems(
                Update,
                (
                    handle_clicks,
                    handle_right_clicks,
                    handle_hover_actions,
                    handle_hover_exit_actions,
                    handle_press_actions,
                    handle_drag_start,
                    handle_drag_move,
                    update_ghost_position,
                    handle_drag_end,
                    handle_drag_abort,
                    update_tooltip_hover,
                    show_tooltip,
                    hide_tooltip,
                    update_interactive_visuals,
                ),
            );
    }
}
