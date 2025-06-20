use bevy::prelude::*;
use super::component::Mana;

pub fn regenerate_mana(mut query: Query<&mut Mana>, time: Res<Time>) {
    for mut mana in &mut query {
        if mana.regen > 0.0 && mana.current < mana.max {
            mana.current = (mana.current + mana.regen * time.delta_secs()).min(mana.max);
        }
    }
}
