use crate::manifest::{load_manifest, Plugin};
use std::fmt;
use std::path::PathBuf;
use uuid::Uuid;
use url::Url;

#[derive(Debug, Default)]
struct ValidationReport {
    guid: Option<Vec<GuidError>>,
    url: Option<Vec<UrlError>>,
}

#[derive(Debug)]
struct GuidError {
    plugin_name: String,
    error: uuid::Error,
}

#[derive(Debug)]
struct UrlError {
    plugin_name: String,
    plugin_version: String,
    error: url::ParseError,
}

impl ValidationReport {
    fn is_none(&self) -> bool {
        self.guid.is_none() && self.url.is_none()
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
            if let Some(v) = &self.url {
                for e in v {
                    results.push_str(&format!("Plugin {} version {} failed validation! URL {}.\n", e.plugin_name, e.plugin_version, e.error));
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
    report.url = validate_url(&manifest);
    println!("{}", report)
}

fn validate_guid(manifest: &[Plugin]) -> Option<Vec<GuidError>> {
    let mut results = Vec::new();

    for p in manifest {
        if let Err(e) = Uuid::parse_str(&p.guid) {
            results.push(GuidError {
                plugin_name: p.name.clone(),
                error: e,
            });
        }
    }

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

fn validate_url(manifest: &[Plugin]) -> Option<Vec<UrlError>> {
    let mut results = Vec::new();

    for p in manifest {
        for v in &p.versions {
            if let Err(e) = Url::parse(&v.source_url) {
                results.push(UrlError {
                    plugin_name: p.name.clone(),
                    plugin_version: v.version.clone(),
                    error: e,
                });
            }
        }
    }

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}
