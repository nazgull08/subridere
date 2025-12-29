use bevy::prelude::*;

use super::{
    impact::play_impact_sounds_system,
    player::{damage::play_player_damage_sfx, events::PlayerDamageEvent},
    worm::{
        events::{WormBiteEvent, WormHurtEvent},
        sounds::{play_worm_bite_sfx, play_worm_hurt_sfx},
    },
    weapons::{
        events::{MagicBoltFireEvent, PhysicsCubeFireEvent},
        sounds::{play_magic_bolt_fire_sfx, play_physical_cube_fire_sfx},
    },
};

pub struct SubAudioPlugin;

impl Plugin for SubAudioPlugin {
    fn build(&self, app: &mut App) {
        app
            // Player events
            .add_event::<PlayerDamageEvent>()
            
            // Worm events
            .add_event::<WormBiteEvent>()
            .add_event::<WormHurtEvent>()
            
            // Weapon events
            .add_event::<MagicBoltFireEvent>()
            .add_event::<PhysicsCubeFireEvent>()
            
            // Systems
            .add_systems(
                Update,
                (
                    play_impact_sounds_system,
                    play_player_damage_sfx,
                    play_worm_bite_sfx,
                    play_worm_hurt_sfx,
                    play_magic_bolt_fire_sfx,
                    play_physical_cube_fire_sfx,
                ),
            );
    }
}
