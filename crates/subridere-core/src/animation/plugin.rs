use bevy::prelude::*;

use super::events::{AnimationFinished, AnimationRequest};

pub struct PlayerAnimationPlugin;

impl Plugin for PlayerAnimationPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<AnimationRequest>()
            .add_event::<AnimationFinished>();

        info!("✅ Animation plugin initialized");
    }
}
