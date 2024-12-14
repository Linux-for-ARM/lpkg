//! Package installer

use crate::consts::ARCHIVES_CACHE_DIR;
use crate::error::*;
// use crate::meta::Meta;
use crate::package::{untar, unxz};
// use crate::{err_msg, msg, ok_msg};

use std::path::{Path, PathBuf};

/// Функци передаётся путь до пакета с расширением `*.txz`. Функция возвращает
/// путь до файла, в котором будет содержаться распакованный пакет с
/// расширением `*.tar`
fn get_unxz_dst<P: AsRef<Path>>(src: P) -> Option<PathBuf> {
    let pth = src.as_ref().file_name();

    if let None = pth {
        return None;
    }

    // Мы можем сделать .unwrap(), потому что выше сделали проверку на None
    let file_name = pth.unwrap().to_string_lossy();

    /* Формируем путь до распакованного из XZ-архива тарболла с пакетом.
     * Тарболл содержит в себе файлы package.toml, files.txt и pkgfiles.tar,
     * располагается в системном кеше lpkg и вместо расширения *.txz
     * имеет расширение *.tar (меняем txz на tar).
     *
     * Подразумеваем, что паттерн `.txz` только один, который содержится В
     * КОНЦЕ имени файла
     */
    let decompressed_fname = file_name.replace(".txz", ".tar");
    let unxz_dst = Path::new(ARCHIVES_CACHE_DIR).join(decompressed_fname);

    Some(unxz_dst)
}

/// Функции передаётся путь до тарболла с расширением `*.tar`. Функция возвращает
/// путь до директории, в которую будет распакован этот тарболл (имя директории =
/// имя тарболла без расширения `.tar`)
fn get_untar_dst<P: AsRef<Path>>(src: P) -> Option<PathBuf> {
    let pth = src.as_ref().file_stem();
    dbg!(&pth);

    if let None = pth {
        return None;
    }

    let file_name = pth.unwrap();
    dbg!(&file_name);

    let extracted_fname = Path::new(ARCHIVES_CACHE_DIR).join(file_name);
    Some(extracted_fname)
}

pub fn extract_pkg<P: AsRef<Path>>(package_pth: P) -> Result<()> {
    let unxz_pth = get_unxz_dst(&package_pth);

    // Если не можем получить путь до файла, в который будем распаковывать, то
    // нет смысла продолжать работать дальше
    if let None = unxz_pth {
        return Err(Error::new(format!(
            "[extract_pkg] Failed to get path to decompressed file!"
        ))
        .error_kind(ErrorKind::ArchiveExtractError));
    }

    // Мы можем сделать .unwrap(), потому что выше сделали проверку на None
    let unxz_pth = unxz_pth.unwrap();

    // Выполняем декомпрессию *.txz-архива
    unxz(&package_pth, &unxz_pth)?;

    // Выполняем распаковку тарболла
    let untar_pth = get_untar_dst(&unxz_pth);

    if let None = untar_pth {
        return Err(Error::new(format!(
            "[extract_pkg] Failed to get path to extracted package!"
        ))
        .error_kind(ErrorKind::ArchiveExtractError));
    }

    let untar_pth = untar_pth.unwrap();

    untar(&unxz_pth, untar_pth)?;

    Ok(())
}

pub fn install_files<P: AsRef<Path>>(extracted_pth: P, prefix: P) -> Result<()> {
    let pkgfiles = extracted_pth.as_ref().join("pkgfiles.tar");
    untar(pkgfiles, &prefix)
}
