// crates/block-bodies-core/src/lib.rs

pub mod body;
pub mod error;
pub mod ik;
pub mod joint;
pub mod part;
pub mod serialization;

pub use body::BlockBody;
pub use error::BlockBodyError;
pub use ik::{solve_arm_ik, solve_leg_ik, solve_two_bone_ik, TwoBoneIkResult};
pub use joint::Joint;
pub use part::BlockPart;

// Re-export types for convenience
pub use glam::{Affine3A, Mat4, Quat, Vec3};
pub use slotmap::DefaultKey as PartId;
