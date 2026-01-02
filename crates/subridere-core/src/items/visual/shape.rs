use bevy::prelude::*;
use serde::Deserialize;

/// Describes a single visual part of an item
///
/// Each part is a primitive shape with position, size, and color.
/// Position is relative to the parent entity.
#[derive(Debug, Clone, Deserialize)]
pub struct VisualPart {
    /// Type of primitive shape
    pub shape: PrimitiveShape,

    /// Size of the shape (interpretation depends on shape type)
    /// - Cube: (width, height, depth)
    /// - Cylinder: (radius, height, radius)
    /// - Sphere: (radius, radius, radius)
    pub size: Vec3,

    /// Position offset relative to parent entity
    pub offset: Vec3,

    /// Color as RGBA (red, green, blue, alpha) - each 0.0 to 1.0
    pub color: (f32, f32, f32, f32),
}

/// Primitive shape types for item visuals
#[derive(Clone, Debug, Deserialize)]
pub enum PrimitiveShape {
    /// Box/Cube shape
    Cube,

    /// Cylinder (useful for staffs, handles, etc)
    Cylinder,

    /// Smooth sphere
    Sphere,

    /// Low-poly sphere (icosphere)
    Icosphere,
}

impl VisualPart {
    /// Convert the color tuple to Bevy Color
    pub fn bevy_color(&self) -> Color {
        Color::srgba(self.color.0, self.color.1, self.color.2, self.color.3)
    }
}
