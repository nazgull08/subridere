use crate::observer::{
    handle_action_button_clicks, handle_hover_actions, handle_press_actions, update_button_visuals
};
use crate::style::ButtonStyle;
use bevy::prelude::*;

pub struct UiActionsPlugin;

impl Plugin for UiActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonStyle>()
            .add_systems(
                Update,
                (
                    handle_action_button_clicks,
                    handle_hover_actions,
                    handle_press_actions,
                    update_button_visuals,
                ),
            );
    }
}
