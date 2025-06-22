use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{
    enemy::{component::*, kind::EnemyKind},
    stats::health::component::Health,
    unit::component::{Grounded, Unit, Velocity},
};

/// Общий базовый спавн для врага
pub fn spawn_enemy_base(commands: &mut Commands, pos: Vec3, kind: EnemyKind) -> Entity {
    commands
        .spawn((
            Enemy,
            kind,
            EnemyState::Idle,
            Unit,
            Grounded(true),
            Velocity::default(),
            Health::new(100.0, 0.0),
            MeleeAttack { damage: 15.0 },
            Transform::from_translation(pos),
            GlobalTransform::default(),
            Visibility::Visible,
            Name::new(format!("{kind:?}")),
            Collider::capsule_y(0.9, 0.3),
            KinematicCharacterController {
                offset: CharacterLength::Absolute(0.01),
                ..default()
            },
            KinematicCharacterControllerOutput::default(),
        ))
        .id()
}
