use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;
use thiserror::Error;


#[derive(Component)]
pub struct PoseToApply(pub BlockPose);

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockPosePart {
    pub name: String,
    pub offset: Vec3,
    pub rotation: Quat,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockPose {
    pub parts: Vec<BlockPosePart>,
}

impl BlockPose {
    pub fn get_part(&self, name: &str) -> Option<&BlockPosePart> {
        self.parts.iter().find(|p| p.name == name)
    }

    pub fn part_map(&self) -> std::collections::HashMap<String, &BlockPosePart> {
        self.parts.iter().map(|p| (p.name.clone(), p)).collect()
    }

    pub fn from_ron_file<P: AsRef<Path>>(path: P) -> Result<Self, PoseLoadError> {
        let data = fs::read_to_string(path)?;
        let pose = ron::from_str::<Self>(&data)?;
        Ok(pose)
    }
}

#[derive(Debug, Error)]
pub enum PoseLoadError {
    #[error("Failed to read pose file: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to parse pose from RON: {0}")]
    Parse(#[from] ron::error::SpannedError),
}
