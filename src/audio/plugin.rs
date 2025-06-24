use bevy::prelude::*;

use super::impact::play_impact_sounds_system;

pub struct SubAudioPlugin;

impl Plugin for SubAudioPlugin {
    fn build(&self, app: &mut App) {
//        app.add_systems(Startup, start_background_audio);
        app.add_systems(Update, play_impact_sounds_system);
    }
}
