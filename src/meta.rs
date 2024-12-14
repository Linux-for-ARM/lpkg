//! Package metadata definitions

use serde::Deserialize;
use crate::traits::Toml;

#[derive(Debug, Deserialize)]
pub struct Meta {
    pub package: Package,
    pub deps: Option<Deps>,
}

impl Toml for Meta {}

#[derive(Debug, Deserialize)]
pub struct Package {
    pub name: String,
    pub version: String,
    pub summary: String,
    pub maintainer: String,
    pub arch: String,
    pub description: String,
}

impl Toml for Package {}

#[derive(Debug, Deserialize)]
pub struct Deps {
    pub required: Option<Vec<String>>,
    pub optional: Option<Vec<String>>,
}

impl Toml for Deps {}
