use std::path::PathBuf;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct FlakeInfo {
    pub description: Option<String>,
    pub path: PathBuf,
}

pub enum FlakeSource {}

pub enum FlakeEntry {
    Package(Package),
    Option(NixOption),
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Package {
    attribute_name: String,
    name: String,
    version: String,
    platforms: Vec<String>,
    description: Option<String>,
    license: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct NixOption {
    declarations: Vec<String>,
    descritpion: String,
    example: String,
    option_type: Option<String>,
}
