// items/visual/shape.rs — Primitive shapes for item visuals

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
}
