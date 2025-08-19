use bevy::prelude::*;

use super::{
    impact::play_impact_sounds_system,
    player::{damage::play_player_damage_sfx, events::PlayerDamageEvent},
};

pub struct SubAudioPlugin;

impl Plugin for SubAudioPlugin {
    fn build(&self, app: &mut App) {
        //        app.add_systems(Startup, start_background_audio);
        app.add_event::<PlayerDamageEvent>()
            .add_systems(Update, play_impact_sounds_system)
            .add_systems(Update, play_player_damage_sfx);
    }
}
