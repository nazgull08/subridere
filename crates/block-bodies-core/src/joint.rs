use serde::{Deserialize, Serialize};

/// Types of joints that can connect parts
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum JointType {
    /// Fixed connection - no movement
    Fixed,
    /// Rotation around single axis
    Revolute,
    /// Full 3D rotation
    Spherical,
}

/// A joint connecting two parts
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Joint {
    /// ID of the parent part
    pub parent_id: String,

    /// ID of the child part
    pub child_id: String,

    /// Type of joint
    pub joint_type: JointType,

    /// Joint limits (implementation depends on joint type)
    pub limits: JointLimits,
}

/// Constraints on joint movement
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JointLimits {
    /// Minimum rotation angles (radians) for each axis
    pub min_rotation: [f32; 3],

    /// Maximum rotation angles (radians) for each axis  
    pub max_rotation: [f32; 3],

    /// Whether each axis is constrained
    pub constrained: [bool; 3],
}

impl Joint {
    /// Create a new fixed joint
    pub fn fixed(parent_id: impl Into<String>, child_id: impl Into<String>) -> Self {
        Self {
            parent_id: parent_id.into(),
            child_id: child_id.into(),
            joint_type: JointType::Fixed,
            limits: JointLimits::none(),
        }
    }

    /// Create a new revolute joint (single axis rotation)
    pub fn revolute(
        parent_id: impl Into<String>,
        child_id: impl Into<String>,
        min_angle: f32,
        max_angle: f32,
        axis: usize, // 0=X, 1=Y, 2=Z
    ) -> Self {
        let mut limits = JointLimits::none();
        limits.min_rotation[axis] = min_angle;
        limits.max_rotation[axis] = max_angle;
        limits.constrained[axis] = true;

        Self {
            parent_id: parent_id.into(),
            child_id: child_id.into(),
            joint_type: JointType::Revolute,
            limits,
        }
    }

    /// Create a new spherical joint (3D rotation)
    pub fn spherical(parent_id: impl Into<String>, child_id: impl Into<String>) -> Self {
        Self {
            parent_id: parent_id.into(),
            child_id: child_id.into(),
            joint_type: JointType::Spherical,
            limits: JointLimits::none(),
        }
    }
}

impl JointLimits {
    /// No constraints on any axis
    pub fn none() -> Self {
        Self {
            min_rotation: [-std::f32::consts::PI; 3],
            max_rotation: [std::f32::consts::PI; 3],
            constrained: [false; 3],
        }
    }
}
