use std::path::Path;

use crate::error::*;
use crate::fs::{fopen, read, write};

pub fn unxz<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) -> Result<()> {
    let r = read(&src)?;
    let decomp = lzma::decompress(&r).map_err(|err| {
        Error::new(format!(
            "Failed to unxz package «{}»: {err}",
            &src.as_ref().display()
        ))
        .error_kind(ErrorKind::ArchiveExtractError)
    })?;

    write(&dst, decomp)
}

pub fn untar<S: AsRef<Path>, D: AsRef<Path>>(src: S, dst: D) -> Result<()> {
    let f = fopen(&src)?;
    let mut a = tar::Archive::new(f);
    a.unpack(&dst).map_err(|err| {
        Error::new(format!(
            "Failed to untar tarball «{}»: {err}",
            &src.as_ref().display()
        ))
        .error_kind(ErrorKind::ArchiveExtractError)
    })?;

    Ok(())
}
