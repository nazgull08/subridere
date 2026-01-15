use bevy::prelude::*;

use crate::stats::Attributes;

use super::component::{Experience, Level};
use super::event::{ExperienceGainEvent, LevelUpEvent};

const ATTRIBUTE_POINTS_PER_LEVEL: u8 = 3;

pub fn process_experience_gain(
    mut events: EventReader<ExperienceGainEvent>,
    mut query: Query<(&mut Experience, &mut Level, &mut Attributes)>,
    mut level_up_events: EventWriter<LevelUpEvent>,
) {
    for event in events.read() {
        let Ok((mut exp, mut level, mut attributes)) = query.get_mut(event.entity) else {
            continue;
        };

        exp.add(event.amount);
        info!(
            "🌟 Gained {} XP ({}/{})",
            event.amount, exp.current, exp.to_next_level
        );

        while exp.can_level_up() {
            exp.level_up(level.current);
            level.current += 1;

            // Добавляем очки напрямую в Attributes
            attributes.add_points(ATTRIBUTE_POINTS_PER_LEVEL);

            info!(
                "⬆️ LEVEL UP! Now level {} (+{} attribute points, total unspent: {})",
                level.current, ATTRIBUTE_POINTS_PER_LEVEL, attributes.unspent_points
            );

            level_up_events.send(LevelUpEvent {
                entity: event.entity,
                new_level: level.current,
            });
        }
    }
}
