use bevy::prelude::*;

/// Marks entities that are affected by movement, gravity, and general unit behavior.
#[derive(Component)]
pub struct Unit;

/// Indicates whether the entity is currently grounded (on the floor).
#[derive(Component, Default)]
pub struct Grounded(pub bool);

#[derive(Component, Default)]
pub struct Velocity(pub Vec3);

/// Signals the entity wants to jump. Added by input, consumed by logic.
#[derive(Component)]
pub struct JumpIntent;

/// Signals the entity wants to dash. Direction can be zero (e.g., dash forward).
#[derive(Component)]
pub struct DashIntent(pub Vec3);

/// Signals the entity wants to move. Usually set every frame based on input.
#[derive(Component)]
pub struct MoveIntent(pub Vec3);

/// Signals the entity wants to turn (face a direction).
#[derive(Component)]
pub struct TurnIntent(pub Quat);

/// Signals the entity wants to shoot.
#[derive(Component)]
pub struct ShootIntent(pub Vec3);

#[derive(Component)]
pub struct LookAtIntent(pub Vec3);

/// Intent to pick up targeted item
#[derive(Component)]
pub struct PickupIntent;
