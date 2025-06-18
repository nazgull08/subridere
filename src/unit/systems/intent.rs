use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::{fighting::projectile::spawn::spawn_projectile, unit::component::{DashIntent, Grounded, JumpIntent, MoveIntent, ShootIntent, Unit, Velocity}};

// Movement tuning constants
const MOVE_ACCEL: f32 = 30.0;
const DASH_SPEED: f32 = 12.0;
const JUMP_SPEED: f32 = 8.0;
const GRAVITY: f32 = 20.0;
const DAMPING: f32 = 6.0;

/// Updates `Grounded` based on Rapier's KinematicCharacterController output.
pub fn update_grounded_system(
    mut query: Query<(&KinematicCharacterControllerOutput, &mut Grounded), With<Unit>>,
) {
    for (output, mut grounded) in &mut query {
        grounded.0 = output.grounded;
    }
}

/// Applies movement intents, converting local XZ input to world-space acceleration.
pub fn apply_move_intents(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &MoveIntent, &mut Velocity, &Transform), With<Unit>>,
) {
    let dt = time.delta_secs();
    for (entity, intent, mut velocity, transform) in &mut query {
        let local = intent.0;
        if local.length_squared() > 0.0 {
            // In Bevy: forward is -Z, right is +X
            let forward = -transform.forward();
            let right = transform.right();
            let dir = (right * local.x + forward * local.z).normalize_or_zero();
            velocity.0.x += dir.x * MOVE_ACCEL * dt;
            velocity.0.z += dir.z * MOVE_ACCEL * dt;
        }
        commands.entity(entity).remove::<MoveIntent>();
    }
}

/// Applies jump if grounded, resetting vertical velocity.
pub fn apply_jump_intents(
    mut commands: Commands,
    mut query: Query<(Entity, &Grounded, &mut Velocity), With<JumpIntent>>,
) {
    for (entity, grounded, mut velocity) in &mut query {
        if grounded.0 {
            velocity.0.y = JUMP_SPEED;
            // Optional: play jump SFX here
        }
        commands.entity(entity).remove::<JumpIntent>();
    }
}

/// Overrides horizontal velocity for dash intents.
pub fn apply_dash_intents(
    mut commands: Commands,
    mut query: Query<(Entity, &DashIntent, &mut Velocity), With<Unit>>,
) {
    for (entity, intent, mut velocity) in &mut query {
        let dash_vec = intent.0.normalize_or_zero() * DASH_SPEED;
        velocity.0.x = dash_vec.x;
        velocity.0.z = dash_vec.z;
        commands.entity(entity).remove::<DashIntent>();
    }
}

/// Applies gravity, damping, and moves the character via KinematicCharacterController.
pub fn apply_velocity(
    time: Res<Time>,
    mut query: Query<(&mut Velocity, &mut KinematicCharacterController, &Grounded), With<Unit>>,
) {
    let dt = time.delta_secs();

    for (mut velocity, mut controller, grounded) in &mut query {
        if velocity.0.length_squared() < 0.0001 && grounded.0 {
            controller.translation = Some(Vec3::ZERO);
            continue;
        }

        velocity.0.x = velocity.0.x.lerp(0.0, DAMPING * dt);
        velocity.0.z = velocity.0.z.lerp(0.0, DAMPING * dt);

        if !grounded.0 {
            velocity.0.y -= GRAVITY * dt;
        } else if velocity.0.y < 0.0 {
            velocity.0.y = 0.0;
        }

        controller.translation = Some(velocity.0 * dt);
    }
}

pub fn handle_shoot_intents(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    query: Query<(Entity, &GlobalTransform, &ShootIntent), With<Unit>>,
) {
    for (entity, transform, intent) in &query {
        let origin = transform.translation() + Vec3::new(0.0, 2.0, 0.0);
        let direction = intent.0;

        spawn_projectile(
            &mut commands,
            &mut meshes,
            &mut materials,
            origin + direction * 1.0, // сместим немного от центра тела
            direction,
        );

        commands.entity(entity).remove::<ShootIntent>();
    }
}
