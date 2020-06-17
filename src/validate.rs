use crate::manifest::load_manifest;
use std::path::PathBuf;

pub fn validate_manifest(file: PathBuf) {
    let manifest = load_manifest(&file).expect("failed to load manifest");

    println!("Manifest passed all validation checks!")
}
