use serde::{Deserialize, Serialize};

use crate::{err, ValidationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Runtime {
    Kotlin,
    Ts,
    Web,
    Native,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct EntryPoint {
    pub runtime: Runtime,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub main: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Permission {
    TileUpdate,
    DeepLinkHandle,
    Notifications,
    Search,
    Location,
    Camera,
    Microphone,
    Storage,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct AppManifest {
    #[serde(rename = "appId")]
    pub app_id: String,
    pub version: String,
    #[serde(rename = "entryPoint")]
    pub entry_point: EntryPoint,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub permissions: Vec<Permission>,
    #[serde(rename = "deepLinks", default, skip_serializing_if = "Vec::is_empty")]
    pub deep_links: Vec<String>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tiles: Vec<String>,
}

impl AppManifest {
    /// Regras do `app-manifest.schema.json`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        let parts: Vec<&str> = self.app_id.split('.').collect();
        let segment_ok = |s: &str| {
            let mut cs = s.chars();
            matches!(cs.next(), Some(c) if c.is_ascii_lowercase())
                && cs.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
        };
        if parts.len() < 2 || !parts.iter().all(|p| segment_ok(p)) {
            return Err(err("appId", format!("'{}' não é domínio reverso válido", self.app_id)));
        }
        if !is_semver(&self.version) {
            return Err(err("version", format!("'{}' não é SemVer", self.version)));
        }
        for (i, p) in self.permissions.iter().enumerate() {
            if self.permissions[..i].contains(p) {
                return Err(err("permissions", "permissão repetida"));
            }
        }
        if let Some(bad) = self.tiles.iter().find(|t| !crate::tile::is_valid_tile_id(t)) {
            return Err(err("tiles", format!("tileId '{bad}' inválido")));
        }
        if let Some(bad) = self.deep_links.iter().find(|d| !d.starts_with("conexus://")) {
            return Err(err("deepLinks", format!("'{bad}' não usa o esquema conexus://")));
        }
        Ok(())
    }
}

/// SemVer 2.0 simplificado, igual ao padrão do schema.
fn is_semver(v: &str) -> bool {
    let (core, build) = v.split_once('+').map_or((v, None), |(a, b)| (a, Some(b)));
    let build_ok = build.map_or(true, |b| {
        !b.is_empty() && b.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    });
    let (core, pre) = core.split_once('-').map_or((core, None), |(a, b)| (a, Some(b)));
    let nums: Vec<&str> = core.split('.').collect();
    let num_ok = nums.len() == 3
        && nums.iter().all(|n| {
            !n.is_empty() && n.chars().all(|c| c.is_ascii_digit()) && (*n == "0" || !n.starts_with('0'))
        });
    let pre_ok = pre.map_or(true, |p| {
        !p.is_empty() && p.chars().all(|c| c.is_ascii_alphanumeric() || c == '.' || c == '-')
    });
    num_ok && pre_ok && build_ok
}
