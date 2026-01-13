use crate::core::ButtonStyle;
use crate::interactions::{
    drag_system, handle_clicks, handle_hover_actions, handle_hover_exit_actions,
    handle_press_actions, handle_right_clicks, has_draggables, DragGhostStyle, DragState,
};
use crate::widgets::{
    hide_tooltip, show_tooltip, update_interactive_visuals, update_tooltip_hover, TooltipState,
    TooltipStyle,
};
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
                    // Click actions
                    handle_clicks,
                    handle_right_clicks,
                    // Hover actions
                    handle_hover_actions,
                    handle_hover_exit_actions,
                    handle_press_actions,
                    // Drag — одна система-автомат
                    drag_system.run_if(has_draggables),
                    // Tooltip
                    update_tooltip_hover,
                    show_tooltip,
                    hide_tooltip,
                    // Visual feedback
                    update_interactive_visuals,
                ),
            );
    }
}
