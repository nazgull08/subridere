use bevy::prelude::*;

use crate::app::AppState;

use super::arms::animate_arm_swing;
use super::events::{AnimationFinished, AnimationRequest};

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationRequest>()
            .add_event::<AnimationFinished>()
            .add_systems(Update, animate_arm_swing.run_if(in_state(AppState::InGame)));

        info!("✅ Player animation plugin initialized");
    }
}
