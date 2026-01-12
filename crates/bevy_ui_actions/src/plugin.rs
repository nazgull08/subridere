use crate::observer::{
    handle_hover_actions, handle_press_actions, on_action_button_click, update_button_visuals,
};
use crate::style::ButtonStyle;
use bevy::prelude::*;

pub struct UiActionsPlugin;

impl Plugin for UiActionsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ButtonStyle>()
            .add_observer(on_action_button_click)
            .add_systems(
                Update,
                (
                    handle_hover_actions,
                    handle_press_actions,
                    update_button_visuals,
                ),
            );
    }
}
