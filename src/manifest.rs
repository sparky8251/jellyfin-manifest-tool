use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::convert::TryFrom;
use std::fmt;
use std::fs::File;
use std::io::Read;
use std::path::PathBuf;

pub fn load_manifest(file: &PathBuf) -> Result<Vec<ValidatePlugin>> {
    let mut file = File::open(&file)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    let manifest: Vec<ValidatePlugin> = serde_json::from_str(&contents)?;
    Ok(manifest)
}

#[derive(Deserialize, Debug)]
pub struct ValidatePlugin {
    category: String,
    pub guid: String,
    pub name: String,
    overview: String,
    owner: String,
    description: String,
    pub versions: Vec<ValidatePluginVersion>,
}

#[derive(Deserialize, Debug)]
pub struct ValidatePluginVersion {
    checksum: String,
    changelog: String,
    name: String,
    #[serde(rename = "targetAbi")]
    pub target_abi: String,
    #[serde(rename = "sourceUrl")]
    pub source_url: String,
    filename: String,
    timestamp: String,
    pub version: String,
}

#[derive(Serialize, Debug)]
pub struct Plugin {
    category: String,
    guid: String, // TODO: Limit to GUIDs only
    name: String,
    overview: String,
    owner: String,
    description: String,
    pub versions: Vec<PluginVersion>,
}

#[derive(Serialize, Debug)]
pub struct PluginVersion {
    checksum: String, // TODO: See if I can limit this to valid md5 hashes
    changelog: String,
    name: String,
    #[serde(rename = "targetAbi")]
    target_abi: String, // TODO: see if I can limit this to semver representation
    #[serde(rename = "sourceUrl")]
    source_url: String, //TODO: Limit to URL
    filename: String,  // TODO: See if I can limit this to a "valid" filename
    timestamp: String, // TODO: See if I can limit this to valid timestamps only
    version: String, // TODO: See if I can limit this to 4 part SemVer with only the first part allowed to be non-zero
}

#[derive(Debug)]
pub struct ThreePartSemver {
    major: usize,
    minor: usize,
    patch: usize,
}

#[derive(Debug)]
pub struct FourPartSemver {
    major: usize,
    minor: usize,
    patch: usize,
    wtf: usize,
}

#[derive(Debug)]
pub enum SemverError {
    InvalidLength(usize, usize),
    NotNumeric,
}

impl TryFrom<String> for ThreePartSemver {
    type Error = SemverError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('.').collect();
        let length = 3;
        if parts.len() != length {
            return Err(Self::Error::InvalidLength(parts.len(), length));
        }
        for s in &parts {
            if s.chars().all(|c| !c.is_digit(10)) {
                return Err(Self::Error::NotNumeric);
            };
        }
        let major = match parts[0].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("Good job! You somehow managed to fool me into accepting a digit that isnt a valid usize!"),
        };
        let minor = match parts[1].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("Good job! You somehow managed to fool me into accepting a digit that isnt a valid usize!"),
        };
        let patch = match parts[2].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("Good job! You somehow managed to fool me into accepting a digit that isnt a valid usize!"),
        };
        Ok(ThreePartSemver {
            major,
            minor,
            patch,
        })
    }
}

impl TryFrom<String> for FourPartSemver {
    type Error = SemverError;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split('.').collect();
        let length = 4;
        if parts.len() != length {
            return Err(Self::Error::InvalidLength(parts.len(), length));
        }
        for s in &parts {
            if s.chars().all(|c| !c.is_digit(10)) {
                return Err(Self::Error::NotNumeric);
            };
        }
        let major = match parts[0].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("Good job! You somehow managed to fool me into accepting a digit that isnt a valid usize!"),
        };
        let minor = match parts[1].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("Good job! You somehow managed to fool me into accepting a digit that isnt a valid usize!"),
        };
        let patch = match parts[2].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("Good job! You somehow managed to fool me into accepting a digit that isnt a valid usize!"),
        };
        let wtf = match parts[3].parse::<usize>() {
            Ok(v) => v,
            Err(_) => panic!("Good job! You somehow managed to fool me into accepting a digit that isnt a valid usize!"),
        };
        Ok(FourPartSemver {
            major,
            minor,
            patch,
            wtf,
        })
    }
}

impl fmt::Display for SemverError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::InvalidLength(got, expected) => {
                write!(f, "Expected {} parts but got {}", expected, got)
            }
            Self::NotNumeric => write!(f, "Got a version part that was not numeric"),
        }
    }
}
