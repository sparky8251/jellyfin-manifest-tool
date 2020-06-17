use crate::manifest::{load_manifest, Plugin};
use std::fmt;
use std::path::PathBuf;
use uuid::Uuid;

#[derive(Debug, Default)]
struct ValidationReport {
    guid: Option<Vec<GuidError>>,
}

#[derive(Debug)]
struct GuidError {
    plugin_name: String,
    error: uuid::Error,
}

impl ValidationReport {
    fn is_none(&self) -> bool {
        self.guid.is_none()
    }
}

impl fmt::Display for ValidationReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self.is_none() {
            write!(f, "Manifest has passed all validation checks!")
        } else {
            let mut results = String::new();
            if let Some(v) = &self.guid {
                for e in v {
                    results.push_str(&format!(
                        "Plugin {} failed validation! GUID {}.\n",
                        e.plugin_name, e.error
                    ));
                }
            }
            write!(f, "{}", results.trim())
        }
    }
}

pub fn validate_manifest(file: PathBuf) {
    let manifest = load_manifest(&file).expect("failed to load manifest");
    let mut report = ValidationReport::default();
    report.guid = validate_guid(&manifest);
    println!("{}", report)
}

fn validate_guid(manifest: &[Plugin]) -> Option<Vec<GuidError>> {
    let mut results = Vec::new();

    for p in manifest {
        match Uuid::parse_str(&p.guid) {
            Ok(_) => (),
            Err(e) => results.push(GuidError {
                plugin_name: p.name.clone(),
                error: e,
            }),
        }
    }

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}
