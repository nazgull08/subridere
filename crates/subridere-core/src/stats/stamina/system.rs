use super::component::Stamina;
use bevy::prelude::*;

pub fn regenerate_stamina(mut query: Query<&mut Stamina>, time: Res<Time>) {
    for mut stamina in &mut query {
        if stamina.regen > 0.0 && stamina.current < stamina.max {
            stamina.current =
                (stamina.current + stamina.regen * time.delta_secs()).min(stamina.max);
        }
    }
}
