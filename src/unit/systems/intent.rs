use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::unit::component::{JumpIntent, DashIntent, MoveIntent, Velocity, Grounded, Unit};

/// Update the grounded flag based on the KinematicCharacterController output
pub fn update_grounded_system(
    mut query: Query<(&KinematicCharacterControllerOutput, &mut Grounded), With<Unit>>,
) {
    for (output, mut grounded) in &mut query {
        grounded.0 = output.grounded;
    }
}

pub fn apply_move_intents(
    mut commands: Commands,
    mut query: Query<(Entity, &MoveIntent, &mut Velocity, &Transform)>,
) {
    for (entity, intent, mut velocity, transform) in &mut query {
        // Local <x, z> to world space
        let forward = transform.forward(); // Vec3
        let right   = transform.right();

        let local = intent.0;              // already normalized
        let world_dir = (right * local.x + forward * local.z).normalize_or_zero();

        velocity.0.x -= world_dir.x * 0.5;
        velocity.0.z -= world_dir.z * 0.5;

        commands.entity(entity).remove::<MoveIntent>();
    }
}

/// Applies JumpIntent if grounded.
pub fn apply_jump_intents(
    mut commands: Commands,
    mut query: Query<(Entity, &Grounded, &mut Velocity), With<JumpIntent>>,
) {
    for (entity, grounded, mut velocity) in &mut query {
        if grounded.0 {
            velocity.0.y = 8.0; // stronger jump impulse
            println!("jumping !");
            commands.entity(entity).remove::<JumpIntent>();
        }
    }
}

/// Applies DashIntent by overriding horizontal velocity.
pub fn apply_dash_intents(
    mut commands: Commands,
    mut query: Query<(Entity, &DashIntent, &mut Velocity)>,
) {
    for (entity, intent, mut velocity) in &mut query {
        let dash_vec = intent.0.normalize_or_zero() * 12.0;
        velocity.0.x = dash_vec.x;
        velocity.0.z = dash_vec.z;
        commands.entity(entity).remove::<DashIntent>();
    }
}

/// Applies gravity and updates controller translation based on velocity.
pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut KinematicCharacterController, &Grounded), With<Unit>>,
) {
    for (mut velocity, mut controller, grounded) in &mut query {
        // Gravity
        if !grounded.0 {
            velocity.0.y -= 20.0 * time.delta_secs();
        } else if velocity.0.y < 0.0 {
            velocity.0.y = 0.0;
        }

        // Friction / damping for xz
        let damping = 6.0;
        velocity.0.x = velocity.0.x.lerp(0.0, damping * time.delta_secs());
        velocity.0.z = velocity.0.z.lerp(0.0, damping * time.delta_secs());

        controller.translation = Some(velocity.0 * time.delta_secs());
    }
}
