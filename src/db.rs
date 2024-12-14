//! Installed packages database

use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

use crate::consts::{DB_PACKAGES_FILE, DB_PACKAGES_DIR};
use crate::{error::*, fs};
use crate::meta;
use crate::traits::Toml;

#[derive(Debug, Deserialize, Serialize)]
pub struct Db {
    #[serde(rename = "package")]
    pub packages: HashMap<String, meta::Package>,

    #[serde(rename = "deps")]
    pub dependencies: HashMap<String, Option<meta::Deps>>,
}

impl Toml for Db {}

impl Db {
    pub fn add_pkg(&mut self, pkg: meta::Package, deps: Option<meta::Deps>) -> Result<()> {
        self.packages.insert(pkg.name.clone(), pkg.clone());
        if let Some(deps) = deps {
            self.dependencies.insert(pkg.name.clone(), Some(deps));
        }

        self.write(DB_PACKAGES_FILE)
    }

    pub fn remove_pkg(&mut self, pkgname: &str) -> Result<()> {
        self.packages.remove(pkgname);
        self.dependencies.remove(pkgname);

        self.write(DB_PACKAGES_FILE)
    }

    pub fn get_flist(&self, pkgname: &str) -> Result<Vec<String>> {
        let pth = Path::new(DB_PACKAGES_DIR).join(format!("{pkgname}.txt"));

        let contents = fs::read_to_string(pth)?;
        let files = contents.split('\n').map(|s| s.to_string()).collect();

        Ok(files)
    }
}
