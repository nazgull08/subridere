use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    enemy::component::*,
    stats::health::component::Health,
    unit::component::{Grounded, Unit, Velocity},
};

/// Общий базовый спавн для врага
pub fn spawn_enemy_base(commands: &mut Commands, pos: Vec3, kind: EnemyKind) -> Entity {
    let mut entity = commands.spawn_empty();

    entity.insert((
        Enemy,
        kind,
        EnemyState::Idle,
        StateTimer(Timer::from_seconds(2.0, TimerMode::Once)),
        Unit,
        Grounded(true),
        Velocity::default(),
        Health::new(100.0, 0.0),
        MeleeAttack {
            bite_damage: 15.0,
            slash_damage: 20.0,
        },
    ));

    entity.insert((
        Transform::from_translation(pos),
        GlobalTransform::default(),
        Visibility::Visible,
        Name::new(format!("{kind:?}")),
    ));

    entity.insert((
        Collider::capsule_y(0.3, 0.3),
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.01),
            ..default()
        },
        KinematicCharacterControllerOutput::default(),
    ));
    entity.insert((
        EnemyMemory {
                target_position: None,
                last_position: pos,
                stuck_timer: Timer::from_seconds(2.0, TimerMode::Once),
            },
        SightRange(10.0)
    ));

    entity.id()
}
