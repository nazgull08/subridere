// items/visual/shape.rs — Primitive shapes for item visuals
use bevy::prelude::{Color, Vec3};
use serde::{Deserialize, Serialize};

/// A single visual part of an item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VisualPart {
    /// Shape type
    pub shape: VisualShape,

    /// Size (width, height, depth) or (radius, height, radius) depending on shape
    pub size: (f32, f32, f32),

    /// Offset from item origin
    #[serde(default)]
    pub offset: (f32, f32, f32),

    /// RGBA color
    pub color: (f32, f32, f32, f32),
}

/// Available primitive shapes
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub enum VisualShape {
    Cube,
    Sphere,
    Cylinder,
    Capsule,
}

impl VisualPart {
    pub fn cube(size: (f32, f32, f32), color: (f32, f32, f32, f32)) -> Self {
        Self {
            shape: VisualShape::Cube,
            size,
            offset: (0.0, 0.0, 0.0),
            color,
        }
    }

    pub fn sphere(radius: f32, color: (f32, f32, f32, f32)) -> Self {
        Self {
            shape: VisualShape::Sphere,
            size: (radius, radius, radius),
            offset: (0.0, 0.0, 0.0),
            color,
        }
    }

    pub fn cylinder(radius: f32, height: f32, color: (f32, f32, f32, f32)) -> Self {
        Self {
            shape: VisualShape::Cylinder,
            size: (radius, height, radius),
            offset: (0.0, 0.0, 0.0),
            color,
        }
    }

    pub fn with_offset(mut self, offset: (f32, f32, f32)) -> Self {
        self.offset = offset;
        self
    }

    /// Convert offset tuple to Vec3
    pub fn offset_vec3(&self) -> Vec3 {
        Vec3::new(self.offset.0, self.offset.1, self.offset.2)
    }

    /// Convert size tuple to Vec3
    pub fn size_vec3(&self) -> Vec3 {
        Vec3::new(self.size.0, self.size.1, self.size.2)
    }

    /// Convert color to Bevy Color
    pub fn bevy_color(&self) -> Color {
        Color::srgba(self.color.0, self.color.1, self.color.2, self.color.3)
    }
}
