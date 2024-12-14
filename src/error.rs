//! Обработка ошибок

use std::error;
use std::fmt;
use std::result;

use colored::Colorize;

pub type Result<T> = result::Result<T, Error>;

#[derive(Debug)]
pub struct Error {
    pub description: String,
    pub kind: ErrorKind,
}

#[derive(Debug, Clone, Copy, Default)]
pub enum ErrorKind {
    NotFound,
    PermissionDenied,
    IsAFile,
    IsADir,
    NotAFile,
    NotADir,
    ReadError,
    WriteError,
    IOError,
    InvalidData,
    ArchiveExtractError,
    ArchiveCreateError,
    MetadataParsingError,
    MetadataWritingError,
    CalculatePackageSizeError,

    #[default]
    Other,
}

impl Error {
    pub fn new(descr: impl ToString) -> Self {
        Self {
            description: descr.to_string(),
            kind: ErrorKind::default(),
        }
    }

    pub fn error_kind(mut self, kind: ErrorKind) -> Self {
        self.kind = kind;
        self
    }
}

impl fmt::Display for ErrorKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::NotFound => "not found",
                Self::PermissionDenied => "permission denied",
                Self::IsAFile => "is a file",
                Self::IsADir => "is a directory",
                Self::NotAFile => "not a file",
                Self::NotADir => "not a directory",
                Self::ReadError => "read error",
                Self::WriteError => "write error",
                Self::IOError => "input/output error",
                Self::InvalidData => "invalid data",
                Self::ArchiveExtractError => "[tar] archive extract error",
                Self::ArchiveCreateError => "[tar] archive create error",
                Self::MetadataParsingError => "package metadata («package.toml») parsing error",
                Self::MetadataWritingError => "package metadata («package.toml») writing error",
                Self::CalculatePackageSizeError => "calculate package size error",
                Self::Other => "other/unknown",
            }
        )
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}{}{} {} {}",
            "[".bold(),
            "E".bold().red(),
            "]".bold(),
            &self.description,
            self.kind.to_string().dimmed(),
        )
    }
}

impl error::Error for Error {
    fn description(&self) -> &str {
        &self.description
    }
}
