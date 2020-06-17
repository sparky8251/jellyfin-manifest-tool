use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;
use uuid::Uuid;

pub fn load_manifest(file: &PathBuf) -> Result<Vec<Plugin>> {
    let mut file = File::open(&file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let manifest: Vec<Plugin> = serde_json::from_str(&contents)?;
    Ok(manifest)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Plugin {
    category: String,
    guid: Uuid,
    name: String,
    overview: String,
    owner: String,
    description: String,
    versions: Vec<PluginVersion>,
}

#[derive(Serialize, Deserialize, Debug)]
struct PluginVersion {
    checksum: String, // TODO: See if I can limit this to valid md5 hashes
    changelog: String,
    name: String,
    #[serde(rename = "targetAbi")]
    target_abi: String, // TODO: see if I can limit this to semver representation
    #[serde(rename = "sourceUrl")]
    source_url: String, // TODO: Limit this to valid URLs only.
    filename: String,  // TODO: See if I can limit this to a "valid" filename
    timestamp: String, // TODO: See if I can limit this to valid timestamps only
    version: String, // TODO: See if I can limit this to 4 part SemVer with only the first part allowed to be non-zero
}
