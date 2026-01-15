use bevy::prelude::*;

use super::component::{Experience, Level};
use super::event::{ExperienceGainEvent, LevelUpEvent};

const ATTRIBUTE_POINTS_PER_LEVEL: u32 = 3;

pub fn process_experience_gain(
    mut events: EventReader<ExperienceGainEvent>,
    mut query: Query<(&mut Experience, &mut Level)>,
    mut level_up_events: EventWriter<LevelUpEvent>,
) {
    for event in events.read() {
        let Ok((mut exp, mut level)) = query.get_mut(event.entity) else {
            continue;
        };

        exp.add(event.amount);
        info!(
            "🌟 Gained {} XP ({}/{})",
            event.amount, exp.current, exp.to_next_level
        );

        // Проверяем level up (может быть несколько за раз)
        while exp.can_level_up() {
            exp.level_up(level.current);
            level.current += 1;
            level.attribute_points += ATTRIBUTE_POINTS_PER_LEVEL;

            info!(
                "⬆️ LEVEL UP! Now level {} (+{} attribute points)",
                level.current, ATTRIBUTE_POINTS_PER_LEVEL
            );

            level_up_events.send(LevelUpEvent {
                entity: event.entity,
                new_level: level.current,
            });
        }
    }
}
