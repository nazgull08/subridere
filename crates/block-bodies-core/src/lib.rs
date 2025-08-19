pub mod body;
pub mod error;
pub mod joint;
pub mod part;
pub mod serialization;

pub use body::BlockBody;
pub use error::BlockBodyError;
pub use joint::Joint;
pub use part::BlockPart;

// Re-export types for convenience
pub use glam::{Affine3A, Mat4, Quat, Vec3};
pub use slotmap::DefaultKey as PartId;
