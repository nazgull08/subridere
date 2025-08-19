use glam::{Affine3A, Quat, Vec3};
use serde::{Deserialize, Serialize};

/// A single block part representing a piece of a body
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct BlockPart {
    /// Human-readable name for this part (used in serialization)
    pub name: String,

    /// Local position relative to parent (or world if root)
    pub position: Vec3,

    /// Local rotation relative to parent (or world if root)  
    pub rotation: Quat,

    /// Size of this block part
    pub size: Vec3,

    /// Optional parent part name (None for root parts)
    pub parent: Option<String>,

    /// World-space transform cache (updated by forward kinematics)
    /// Note: This field is not serialized as it's computed at runtime
    #[serde(skip)]
    pub world_transform: Affine3A,
}

impl BlockPart {
    /// Create a new block part
    pub fn new(name: impl Into<String>, position: Vec3, size: Vec3) -> Self {
        Self {
            name: name.into(),
            position,
            rotation: Quat::IDENTITY,
            size,
            parent: None,
            world_transform: Affine3A::IDENTITY,
        }
    }

    /// Create a new block part with a parent
    pub fn with_parent(
        name: impl Into<String>,
        position: Vec3,
        size: Vec3,
        parent: impl Into<String>,
    ) -> Self {
        Self {
            name: name.into(),
            position,
            rotation: Quat::IDENTITY,
            size,
            parent: Some(parent.into()),
            world_transform: Affine3A::IDENTITY,
        }
    }

    /// Get local transform matrix
    pub fn local_transform(&self) -> Affine3A {
        Affine3A::from_rotation_translation(self.rotation, self.position)
    }

    /// Check if this is a root part (no parent)
    pub fn is_root(&self) -> bool {
        self.parent.is_none()
    }
}
