//! Functions for working fith files and directories

use std::io::Write;
use std::path::Path;
use std::{fs, io};

use crate::error::*;

/// Open file for read and/or write ([`fs::File::open`] wrapper)
pub fn fopen<P: AsRef<Path>>(path: P) -> Result<fs::File> {
    fs::File::open(&path).map_err(|err| {
        Error::new(&err).error_kind(match err.kind() {
            io::ErrorKind::NotFound => ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            _ => {
                if !path.as_ref().is_file() {
                    ErrorKind::NotAFile
                } else {
                    ErrorKind::Other
                }
            }
        })
    })
}

/// Read file contents to the `Vec<u8>` ([`fs::read`] wrapper)
pub fn read<P: AsRef<Path>>(path: P) -> Result<Vec<u8>> {
    fs::read(&path).map_err(|err| {
        Error::new(&err).error_kind(match err.kind() {
            io::ErrorKind::NotFound => ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            io::ErrorKind::InvalidData => ErrorKind::InvalidData,
            _ => {
                if !path.as_ref().is_file() {
                    ErrorKind::NotAFile
                } else {
                    ErrorKind::Other
                }
            }
        })
    })
}

/// Read file contents to the `String` ([`fs::read_to_string`] wrapper)
pub fn read_to_string<P: AsRef<Path>>(path: P) -> Result<String> {
    fs::read_to_string(&path).map_err(|err| {
        Error::new(&err).error_kind(match err.kind() {
            io::ErrorKind::NotFound => ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            io::ErrorKind::InvalidData => ErrorKind::InvalidData,
            _ => {
                if !path.as_ref().is_file() {
                    ErrorKind::NotAFile
                } else {
                    ErrorKind::Other
                }
            }
        })
    })
}

pub fn write<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, cont: C) -> Result<()> {
    fs::write(&path, &cont).map_err(|err| {
        Error::new(&err).error_kind(match err.kind() {
            io::ErrorKind::NotFound => ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            io::ErrorKind::InvalidData => ErrorKind::InvalidData,
            _ => {
                if !path.as_ref().is_file() {
                    ErrorKind::NotAFile
                } else {
                    ErrorKind::Other
                }
            }
        })
    })
}

pub fn write_append<P: AsRef<Path>, C: AsRef<[u8]>>(path: P, cont: C) -> Result<()> {
    let mut f = fs::File::options()
        .append(true)
        .open(&path)
        .map_err(|err| {
            Error::new(&err).error_kind(match err.kind() {
                io::ErrorKind::NotFound => ErrorKind::NotFound,
                io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
                _ => {
                    if !path.as_ref().is_file() {
                        ErrorKind::NotAFile
                    } else {
                        ErrorKind::Other
                    }
                }
            })
        })?;

    f.write_all(&cont.as_ref()).map_err(|err| {
        Error::new(&err).error_kind(match err.kind() {
            io::ErrorKind::NotFound => ErrorKind::NotFound,
            io::ErrorKind::PermissionDenied => ErrorKind::PermissionDenied,
            _ => {
                if !path.as_ref().is_file() {
                    ErrorKind::NotAFile
                } else {
                    ErrorKind::Other
                }
            }
        })
    })?;

    Ok(())
}
