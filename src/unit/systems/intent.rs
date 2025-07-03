use bevy::prelude::*;
use bevy_rapier3d::prelude::*;

use crate::enemy::component::Enemy;
use crate::stats::mana::component::Mana;
use crate::unit::component::{AttackIntent, LookAtIntent, TurnIntent};
use crate::{
    fighting::projectile::spawn::spawn_projectile,
    unit::component::{DashIntent, Grounded, JumpIntent, MoveIntent, ShootIntent, Unit, Velocity},
};

// Movement tuning constants
const MOVE_ACCEL: f32 = 50.0;
const DASH_SPEED: f32 = 12.0;
const JUMP_SPEED: f32 = 20.0;
const GRAVITY: f32 = 20.0;
const DAMPING: f32 = 3.0;

const ENEMY_MOVE_ACCEL: f32 = 20.0;
const ENEMY_MAX_SPEED: f32 = 6.0;
const ENEMY_FRICTION: f32 = 12.0;

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

pub fn apply_turn_intents(
    mut commands: Commands,
    mut query: Query<(Entity, &TurnIntent, &mut Transform)>,
) {
    for (entity, intent, mut transform) in &mut query {
        info!("turning");
        transform.rotation = intent.0;
        commands.entity(entity).remove::<TurnIntent>();
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

pub fn apply_move_intents_for_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut query: Query<(Entity, &MoveIntent, &mut Velocity), (With<Enemy>, With<Unit>)>,
) {
    let dt = time.delta_secs();

    for (entity, intent, mut velocity) in &mut query {
        let dir = intent.0.normalize_or_zero();
        velocity.0 += dir * ENEMY_MOVE_ACCEL * dt;
        // clamp:
        velocity.0.x = velocity.0.x.clamp(-ENEMY_MAX_SPEED, ENEMY_MAX_SPEED);
        velocity.0.z = velocity.0.z.clamp(-ENEMY_MAX_SPEED, ENEMY_MAX_SPEED);
        commands.entity(entity).remove::<MoveIntent>();
    }
}

/// Applies look rotation based on LookAtIntent — smooth Y-axis facing.
pub fn apply_look_intents_for_enemies(
    mut commands: Commands,
    mut query: Query<(Entity, &LookAtIntent, &mut Transform), (With<Enemy>, With<Unit>)>,
) {
    for (entity, intent, mut transform) in &mut query {
        let direction = intent.0 - transform.translation;
        if direction.length_squared() > 0.001 {
            let yaw = direction.z.atan2(direction.x);
            let target_rotation = Quat::from_rotation_y(-yaw);
            transform.rotation = transform.rotation.slerp(target_rotation, 0.2);
        }

        commands.entity(entity).remove::<LookAtIntent>();
    }
}

/// Processes enemy attack intents. Later: apply hit, trigger animations, sounds, etc.
pub fn apply_attack_intents_for_enemies(
    mut commands: Commands,
    query: Query<(Entity, &AttackIntent), With<Enemy>>,
) {
    for (entity, intent) in &query {
        match intent {
            AttackIntent::Bite => {
                // TODO: play bite animation, apply hitbox, etc
            }
            AttackIntent::Slash => {
                // TODO: play slash animation, apply hitbox, etc
            }
        }

        commands.entity(entity).remove::<AttackIntent>();
    }
}

pub fn handle_shoot_intents(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut query: Query<(Entity, &GlobalTransform, &ShootIntent, &mut Mana), With<Unit>>,
) {
    let mana_cost = 10.0;

    for (entity, transform, intent, mut mana) in &mut query {
        if mana.current >= mana_cost {
            mana.current -= mana_cost;

            let origin = transform.translation() + Vec3::new(0.0, 2.0, 0.0);
            let direction = intent.0;

            spawn_projectile(
                &mut commands,
                &mut meshes,
                &mut materials,
                origin + direction * 1.0,
                direction,
            );
        } else {
            info!("Entity {:?} has not enough mana to shoot", entity);
        }

        commands.entity(entity).remove::<ShootIntent>();
    }
}
