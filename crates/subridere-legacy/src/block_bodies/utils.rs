use std::collections::HashMap;
use std::fs;
use std::path::Path;

use bevy::prelude::*;
use serde::Deserialize;
use thiserror::Error;

use crate::block_bodies::model::{BlockModel, BlockModelPart, SocketType};

#[derive(Debug, Deserialize)]
struct BlockModelFile {
    parts: Vec<BlockModelFilePart>,
}

#[derive(Debug, Deserialize)]
struct BlockModelFilePart {
    name: String,
    parent: Option<String>,
    local_offset: Vec3,
    size: Vec3,
    material: String,
    socket: Option<SocketType>,
}

#[derive(Debug, Error)]
pub enum ModelLoadError {
    #[error("Failed to read model file: {0}")]
    Io(#[from] std::io::Error),
    #[error("Failed to parse model from RON: {0}")]
    Parse(#[from] ron::error::SpannedError),
    #[error("Material '{0}' not found in material map")]
    MaterialNotFound(String),
}

pub fn load_model_from_ron<P: AsRef<Path>>(
    path: P,
    material_map: &HashMap<String, Handle<StandardMaterial>>,
) -> Result<BlockModel, ModelLoadError> {
    let data = fs::read_to_string(path)?;
    let parsed: BlockModelFile = ron::from_str(&data)?;

    let parts = parsed
        .parts
        .into_iter()
        .map(|part| {
            let material = material_map
                .get(&part.material)
                .ok_or_else(|| ModelLoadError::MaterialNotFound(part.material.clone()))?
                .clone();

            Ok(BlockModelPart {
                name: part.name,
                parent: part.parent,
                local_offset: part.local_offset,
                size: part.size,
                material,
                socket: part.socket,
            })
        })
        .collect::<Result<Vec<_>, ModelLoadError>>()?;

    Ok(BlockModel::new(parts))
}
