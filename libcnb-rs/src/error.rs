use thiserror::Error as ThisError;

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Problem Serializing TOML: {0}")]
    TomlSerialize(#[from] toml::ser::Error),
    #[error("Problem Deserializing TOML: {0}")]
    TomlDeserialize(#[from] toml::de::Error),
}
