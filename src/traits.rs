use serde::{Deserialize, Serialize};
use toml;

use std::path::Path;

use crate::error::*;
use crate::fs;

/// Serializing/deserializing data to/from TOML-configs
pub trait Toml {
    fn write<P>(&self, path: P) -> Result<()>
    where
        Self: Serialize,
        P: AsRef<Path>,
    {
        let data = toml::to_string(&self).map_err(|err| {
            Error::new(format!("Failed to serialize TOML structure: {err}"))
                .error_kind(ErrorKind::MetadataWritingError)
        })?;
        fs::write(&path, data)
    }

    fn read<P>(path: P) -> Result<Self>
    where
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
        P: AsRef<Path>,
    {
        let contents = fs::read_to_string(&path)?;
        let data = toml::from_str::<Self>(&contents)
            .map_err(|err| Error::new(&err).error_kind(ErrorKind::MetadataParsingError))?;

        Ok(data)
    }

    fn from_str(s: &str) -> Result<Self>
    where
        Self: Sized,
        for<'de> Self: Deserialize<'de>,
    {
        let data = toml::from_str::<Self>(s)
            .map_err(|err| Error::new(&err).error_kind(ErrorKind::MetadataParsingError))?;

        Ok(data)
    }
}
