use thiserror::Error;

#[derive(Error, Debug)]
pub enum BlockBodyError {
    #[error("Part '{0}' not found")]
    PartNotFound(String),

    #[error("Part '{0}' already exists")]
    PartAlreadyExists(String),

    #[error("Parent part '{parent}' not found for part '{child}'")]
    ParentNotFound { parent: String, child: String },

    #[error("Circular dependency detected: part '{0}' cannot be its own ancestor")]
    CircularDependency(String),

    #[error("Invalid part name '{0}': names cannot be empty")]
    InvalidPartName(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] ron::Error),

    #[error("Serialization error: {0}")]
    SpannedError(#[from] ron::error::SpannedError),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),
}
