use bevy::prelude::*;

/// Event when magic bolt is fired
#[derive(Event)]
pub struct MagicBoltFireEvent;

/// Event when physical cube is launched
#[derive(Event)]
pub struct PhysicsCubeFireEvent;
