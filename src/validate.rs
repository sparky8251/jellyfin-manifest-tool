use crate::manifest::{load_manifest, FourPartSemver, ThreePartSemver, ValidatePlugin};
use std::convert::TryFrom;
use std::fmt;
use std::path::PathBuf;
use url::Url;
use uuid::Uuid;

#[derive(Debug, Default)]
struct ValidationReport {
    guid: Option<Vec<GuidError>>,
    url: Option<Vec<UrlError>>,
    semver: Option<Vec<SemverError>>,
    checksum: Option<Vec<ChecksumError>>,
    timestamp: Option<Vec<TimestampError>>,
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

#[derive(Debug)]
struct SemverError {
    plugin_name: String,
    plugin_version: String,
    abi_error: Option<crate::manifest::SemverError>,
    version_error: Option<crate::manifest::SemverError>,
}

#[derive(Debug)]
struct ChecksumError {
    plugin_name: String,
    plugin_version: String,
    error: crate::manifest::ChecksumError,
}

#[derive(Debug)]
struct TimestampError {
    plugin_name: String,
    plugin_version: String,
    error: humantime::TimestampError,
}

impl ValidationReport {
    fn is_none(&self) -> bool {
        self.guid.is_none()
            && self.url.is_none()
            && self.semver.is_none()
            && self.checksum.is_none()
            && self.timestamp.is_none()
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
                    results.push_str(&format!(
                        "Plugin {} version {} failed validation! URL {}.\n",
                        e.plugin_name, e.plugin_version, e.error
                    ));
                }
            }
            if let Some(v) = &self.semver {
                for e in v {
                    let mut result = String::new();
                    result.push_str(&format!(
                        "Plugin {} version {} failed validation! ",
                        e.plugin_name, e.plugin_version
                    ));
                    if e.abi_error.is_some() {
                        result.push_str(&format!("Target ABI {} ", e.abi_error.as_ref().unwrap()));
                    }
                    if e.version_error.is_some() {
                        result.push_str(&format!("Version {} ", e.version_error.as_ref().unwrap()));
                    }
                    result.push('\n');
                    results.push_str(&result);
                }
            }
            if let Some(v) = &self.checksum {
                for e in v {
                    let mut result = String::new();
                    result.push_str(&format!(
                        "Plugin {} version {} failed validation! {}",
                        e.plugin_name, e.plugin_version, e.error
                    ));
                    result.push('\n');
                    results.push_str(&result);
                }
            }
            if let Some(v) = &self.timestamp {
                for e in v {
                    let mut result = String::new();
                    result.push_str(&format!(
                        "Plugin {} version {} failed validation! {}",
                        e.plugin_name, e.plugin_version, e.error
                    ));
                    result.push('\n');
                    results.push_str(&result);
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
    report.semver = validate_semver(&manifest);
    report.checksum = validate_checksum(&manifest);
    report.timestamp = validate_timestamp(&manifest);
    println!("{}", report)
}

fn validate_guid(manifest: &[ValidatePlugin]) -> Option<Vec<GuidError>> {
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

fn validate_url(manifest: &[ValidatePlugin]) -> Option<Vec<UrlError>> {
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

fn validate_semver(manifest: &[ValidatePlugin]) -> Option<Vec<SemverError>> {
    let mut results = Vec::new();

    for p in manifest {
        for v in &p.versions {
            let abi_error = if let Err(abi) = ThreePartSemver::try_from(v.target_abi.clone()) {
                Some(abi)
            } else {
                None
            };
            let version_error = if let Err(version) = FourPartSemver::try_from(v.version.clone()) {
                Some(version)
            } else {
                None
            };

            if abi_error.is_some() || version_error.is_some() {
                results.push(SemverError {
                    plugin_name: p.name.clone(),
                    plugin_version: v.version.clone(),
                    abi_error,
                    version_error,
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

fn validate_checksum(manifest: &[ValidatePlugin]) -> Option<Vec<ChecksumError>> {
    let mut results = Vec::new();

    for p in manifest {
        for v in &p.versions {
            if v.checksum.chars().count() != 32 {
                results.push(ChecksumError {
                    plugin_name: p.name.clone(),
                    plugin_version: v.version.clone(),
                    error: crate::manifest::ChecksumError::InvalidLength(
                        v.checksum.chars().count(),
                    ),
                });
            }
            if v.checksum.chars().all(|c| !c.is_ascii_hexdigit()) {
                results.push(ChecksumError {
                    plugin_name: p.name.clone(),
                    plugin_version: v.version.clone(),
                    error: crate::manifest::ChecksumError::InvalidCharacters,
                });
            };
        }
    }

    if results.is_empty() {
        None
    } else {
        Some(results)
    }
}

fn validate_timestamp(manifest: &[ValidatePlugin]) -> Option<Vec<TimestampError>> {
    let mut results = Vec::new();

    for p in manifest {
        for v in &p.versions {
            if let Err(e) = humantime::parse_rfc3339_weak(&v.timestamp) {
                results.push(TimestampError {
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
