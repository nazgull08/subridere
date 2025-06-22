// src/block_bodies/pose.rs

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fs, path::Path};
use thiserror::Error;

/// Компонент для задания разовой позы (например, в момент спавна)
#[derive(Debug, Component)]
pub struct PoseToApply(pub BlockPose);

/// Поворот части тела в позе
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BlockPosePart {
    pub name: String,
    pub rotation: Quat,
}

/// Позы определяются только поворотами (rotation), а расположение берётся из модели
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct BlockPose {
    pub parts: Vec<BlockPosePart>,
}

impl BlockPose {
    /// Получить часть по имени
    pub fn get_part(&self, name: &str) -> Option<&BlockPosePart> {
        self.parts.iter().find(|p| p.name == name)
    }

    /// Быстрое отображение по имени
    pub fn part_map(&self) -> HashMap<String, &BlockPosePart> {
        self.parts.iter().map(|p| (p.name.clone(), p)).collect()
    }

    /// Загрузить позу из RON-файла
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
