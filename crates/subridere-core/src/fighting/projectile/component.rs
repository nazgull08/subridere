use bevy::prelude::*;

#[derive(Component, Debug)]
pub struct Projectile {
    pub size: f32,
    pub lifetime: f32,
    pub velocity: Vec3,
}

impl Projectile {
    pub fn new(size: f32, lifetime: f32, velocity: Vec3) -> Self {
        Self {
            size,
            lifetime,
            velocity,
        }
    }
}
