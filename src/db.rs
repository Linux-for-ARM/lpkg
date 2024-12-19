//! Installed packages database

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::consts::{DB_PACKAGES_DIR, DB_PACKAGES_FILE};
use crate::meta;
use crate::traits::Toml;
use crate::{error::*, fs};

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
    #[serde(rename = "entry")]
    pub entries: HashMap<String, Entry>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Entry {
    pub install_prefix: String,
    pub package: meta::Package,
    pub deps: Option<meta::Deps>,
}

impl Toml for Db {}

impl Default for Db {
    fn default() -> Self {
        let mut entries = HashMap::new();
        entries.insert(
            "lpkg".to_string(),
            Entry {
                install_prefix: "/".to_string(),
                package: meta::Package {
                    name: "lpkg".to_string(),
                    version: "0.1".to_string(),
                    summary: "Simple package manager for LFA".to_string(),
                    maintainer: "Michail Krasnov <michail383krasnov@mail.ru>".to_string(),
                    arch: "x86_64".to_string(),
                    description: None,
                },
                deps: None,
            },
        );

        Self { entries }
    }
}

impl Db {
    pub fn open() -> Result<Self> {
        let pth = Path::new(DB_PACKAGES_FILE);
        if !pth.exists() {
            let db = Db::default();
            db.write(DB_PACKAGES_FILE)?;
        }
        Self::read(DB_PACKAGES_FILE)
    }

    pub fn add_pkg<P: AsRef<Path>, I: AsRef<Path>>(
        &mut self,
        pkg: meta::Package,
        deps: Option<meta::Deps>,
        extract_pth: P,
        install_prefix: I,
    ) -> Result<()> {
        let prefix = install_prefix.as_ref().display().to_string();
        self.entries.insert(
            pkg.name.clone(),
            Entry {
                install_prefix: prefix,
                package: pkg.clone(),
                deps: deps.clone(),
            },
        );

        self.write(DB_PACKAGES_FILE)?;

        let flist = extract_pth.as_ref().join("files.txt");
        let flist_tgt = Path::new(DB_PACKAGES_DIR).join(format!("{}.txt", &pkg.name));
        fs::copy(flist, flist_tgt)?;

        Ok(())
    }

    pub fn check_pkg(&self, pkgname: &str) -> bool {
        self.entries.get(pkgname).is_some()
    }

    pub fn remove_pkg(&mut self, pkgname: &str) -> Result<()> {
        self.entries.remove(pkgname);
        std::fs::remove_file(Path::new(DB_PACKAGES_DIR).join("{pkgname}.txt"))
            .map_err(|err| Error::new(err))?;

        self.write(DB_PACKAGES_FILE)
    }

    pub fn get_flist(&self, pkgname: &str) -> Result<Vec<String>> {
        let pth = Path::new(DB_PACKAGES_DIR).join(format!("{pkgname}.txt"));

        let contents = fs::read_to_string(pth)?;
        let files = contents.split('\n').map(|s| s.to_string()).collect();

        Ok(files)
    }
}
