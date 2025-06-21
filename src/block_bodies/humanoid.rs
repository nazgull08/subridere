use bevy::prelude::*;
use crate::block_bodies::body::{BlockBody, BlockPart, SocketType};

pub fn make_blocky_humanoid(
    red: Handle<StandardMaterial>,
    gray: Handle<StandardMaterial>,
) -> BlockBody {
    let parts = vec![
        // === Туловище и голова ===
        BlockPart::new("Torso", Vec3::new(0.0, 1.0, 0.0), Vec3::new(0.5, 0.6, 0.3), gray.clone()),             // 0.7 .. 1.3
        BlockPart::new("Neck",  Vec3::new(0.0, 1.4, 0.0), Vec3::new(0.2, 0.2, 0.2), gray.clone()),             // 1.3 .. 1.5
        BlockPart::new("Head",  Vec3::new(0.0, 1.8, 0.0), Vec3::new(0.4, 0.4, 0.4), red.clone())                // 1.5 .. 2.1
            .with_socket(SocketType::Helmet),

        // === Левая рука ===
        BlockPart::new("ShoulderL", Vec3::new(-0.45, 1.3, 0.0), Vec3::new(0.2, 0.2, 0.2), red.clone()),         // 1.2 .. 1.4
        BlockPart::new("ArmL",      Vec3::new(-0.45, 1.0, 0.0), Vec3::new(0.2, 0.4, 0.2), gray.clone()),        // 0.8 .. 1.2
        BlockPart::new("ForearmL",  Vec3::new(-0.45, 0.7, 0.0), Vec3::new(0.2, 0.4, 0.2), red.clone()),         // 0.5 .. 0.9
        BlockPart::new("HandL",     Vec3::new(-0.45, 0.5, 0.0), Vec3::new(0.2, 0.2, 0.2), gray.clone())         // 0.4 .. 0.6
            .with_socket(SocketType::Shield),

        // === Правая рука ===
        BlockPart::new("ShoulderR", Vec3::new(0.45, 1.3, 0.0), Vec3::new(0.2, 0.2, 0.2), red.clone()),
        BlockPart::new("ArmR",      Vec3::new(0.45, 1.0, 0.0), Vec3::new(0.2, 0.4, 0.2), gray.clone()),
        BlockPart::new("ForearmR",  Vec3::new(0.45, 0.7, 0.0), Vec3::new(0.2, 0.4, 0.2), red.clone()),
        BlockPart::new("HandR",     Vec3::new(0.45, 0.5, 0.0), Vec3::new(0.2, 0.2, 0.2), gray.clone())
            .with_socket(SocketType::WeaponMain),

        // === Левая нога ===
        BlockPart::new("ThighL", Vec3::new(-0.15, 0.6, 0.0), Vec3::new(0.2, 0.3, 0.2), red.clone()),            // 0.45 .. 0.75
        BlockPart::new("ShinL",  Vec3::new(-0.15, 0.3, 0.0), Vec3::new(0.2, 0.3, 0.2), gray.clone()),           // 0.15 .. 0.45
        BlockPart::new("FootL",  Vec3::new(-0.15, 0.1, 0.1), Vec3::new(0.25, 0.1, 0.3), red.clone()),           // 0.0 .. 0.2

        // === Правая нога ===
        BlockPart::new("ThighR", Vec3::new(0.15, 0.6, 0.0), Vec3::new(0.2, 0.3, 0.2), red.clone()),
        BlockPart::new("ShinR",  Vec3::new(0.15, 0.3, 0.0), Vec3::new(0.2, 0.3, 0.2), gray.clone()),
        BlockPart::new("FootR",  Vec3::new(0.15, 0.1, 0.1), Vec3::new(0.25, 0.1, 0.3), red.clone()),
    ];

    BlockBody::new(parts)
}
