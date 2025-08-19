use crate::{BlockBody, BlockBodyError, BlockPart};
use ron::ser::{to_string_pretty, PrettyConfig};
use serde::{Deserialize, Serialize};
use std::path::Path;

/// Serializable representation of a BlockBody for RON files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockBodyFile {
    pub parts: Vec<BlockPart>,
}

impl BlockBodyFile {
    /// Convert from runtime BlockBody to serializable format
    pub fn from_body(body: &BlockBody) -> Self {
        let mut parts: Vec<BlockPart> = body.parts.values().cloned().collect();

        // Sort by name for consistent serialization
        parts.sort_by(|a, b| a.name.cmp(&b.name));

        Self { parts }
    }

    /// Convert to runtime BlockBody from serializable format
    pub fn to_body(self) -> Result<BlockBody, BlockBodyError> {
        let mut body = BlockBody::new();

        // Sort parts to ensure parents are added before children
        let mut remaining_parts = self.parts;
        let mut added_parts = std::collections::HashSet::new();

        while !remaining_parts.is_empty() {
            let initial_len = remaining_parts.len();

            // Find parts that can be added (either root parts or parts whose parents are already added)
            let (ready_parts, still_waiting): (Vec<_>, Vec<_>) =
                remaining_parts.into_iter().partition(|part| {
                    part.parent.is_none()
                        || part
                            .parent
                            .as_ref()
                            .is_some_and(|parent| added_parts.contains(parent))
                });

            // Add all ready parts
            for part in ready_parts {
                let part_name = part.name.clone();
                body.add_part(part)?;
                added_parts.insert(part_name);
            }

            remaining_parts = still_waiting;

            // If we didn't make progress, we have circular dependencies or missing parents
            if remaining_parts.len() == initial_len {
                if let Some(orphan) = remaining_parts.first() {
                    return Err(BlockBodyError::ParentNotFound {
                        parent: orphan.parent.clone().unwrap_or_default(),
                        child: orphan.name.clone(),
                    });
                }
            }
        }

        // Validate the structure
        body.validate()?;

        Ok(body)
    }

    /// Load from RON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, BlockBodyError> {
        let content = std::fs::read_to_string(path)?;
        let body_file: BlockBodyFile = ron::from_str(&content)?;
        Ok(body_file)
    }

    /// Save to RON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), BlockBodyError> {
        let content = to_string_pretty(self, PrettyConfig::default())?;
        std::fs::write(path, content)?;
        Ok(())
    }
}

impl BlockBody {
    /// Load BlockBody from RON file
    pub fn load_from_file<P: AsRef<Path>>(path: P) -> Result<Self, BlockBodyError> {
        let body_file = BlockBodyFile::load_from_file(path)?;
        body_file.to_body()
    }

    /// Save BlockBody to RON file
    pub fn save_to_file<P: AsRef<Path>>(&self, path: P) -> Result<(), BlockBodyError> {
        let body_file = BlockBodyFile::from_body(self);
        body_file.save_to_file(path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{BlockPart, Vec3};
    use tempfile::NamedTempFile;

    #[test]
    fn test_serialization_roundtrip() {
        // Create a test body
        let mut body = BlockBody::new();

        let torso = BlockPart::new("torso", Vec3::ZERO, Vec3::ONE);
        let head =
            BlockPart::with_parent("head", Vec3::new(0.0, 1.0, 0.0), Vec3::splat(0.5), "torso");

        body.add_part(torso).unwrap();
        body.add_part(head).unwrap();

        // Convert to serializable format and back
        let body_file = BlockBodyFile::from_body(&body);
        let restored_body = body_file.to_body().unwrap();

        // Check that we got the same data
        assert_eq!(restored_body.parts.len(), 2);
        assert!(restored_body.get_part_by_name("torso").is_some());
        assert!(restored_body.get_part_by_name("head").is_some());

        let head_part = restored_body.get_part_by_name("head").unwrap();
        assert_eq!(head_part.parent, Some("torso".to_string()));
    }

    #[test]
    fn test_file_io() -> Result<(), BlockBodyError> {
        // Create a test body
        let mut body = BlockBody::new();
        let torso = BlockPart::new("torso", Vec3::ZERO, Vec3::ONE);
        body.add_part(torso)?;

        // Save to temporary file
        let temp_file = NamedTempFile::new().unwrap();
        body.save_to_file(temp_file.path())?;

        // Load from file
        let loaded_body = BlockBody::load_from_file(temp_file.path())?;

        // Verify
        assert_eq!(loaded_body.parts.len(), 1);
        assert!(loaded_body.get_part_by_name("torso").is_some());

        Ok(())
    }
}
