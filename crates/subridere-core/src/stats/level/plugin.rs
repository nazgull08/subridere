use bevy::prelude::*;

use super::event::{ExperienceGainEvent, LevelUpEvent};
use super::system::process_experience_gain;

pub struct LevelPlugin;

impl Plugin for LevelPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<ExperienceGainEvent>()
            .add_event::<LevelUpEvent>()
            .add_systems(Update, process_experience_gain);
    }
}
