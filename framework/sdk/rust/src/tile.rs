use serde::{Deserialize, Serialize};

use crate::{err, ValidationError};

/// Tamanhos de tile do SDD (S/M/W/L).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileSize {
    S,
    M,
    W,
    L,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TileTemplate {
    Icon,
    Text,
    Image,
    Media,
    Flip,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum UpdateMode {
    Push,
    Periodic,
    Manual,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TileContent {
    pub template: TileTemplate,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub data: Option<serde_json::Value>,
}

/// Limite do `content.data` serializado: um app não infla a memória do tile engine.
pub const MAX_TILE_DATA_BYTES: usize = 16 * 1024;

impl TileContent {
    /// `data`, quando presente, deve ser objeto e caber em `MAX_TILE_DATA_BYTES`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(data) = &self.data {
            if !data.is_object() {
                return Err(err("content.data", "deve ser um objeto"));
            }
            let size = serde_json::to_vec(data).map(|v| v.len()).unwrap_or(usize::MAX);
            if size > MAX_TILE_DATA_BYTES {
                return Err(err("content.data", format!("{size} bytes (máximo {MAX_TILE_DATA_BYTES})")));
            }
        }
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TileActivation {
    #[serde(rename = "deepLink", default, skip_serializing_if = "Option::is_none")]
    pub deep_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TileUpdate {
    pub mode: UpdateMode,
    #[serde(rename = "periodMinutes", default, skip_serializing_if = "Option::is_none")]
    pub period_minutes: Option<u32>,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TileManifest {
    #[serde(rename = "tileId")]
    pub tile_id: String,
    pub size: TileSize,
    pub content: TileContent,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub activation: Option<TileActivation>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub update: Option<TileUpdate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub badges: Vec<String>,
}

impl TileManifest {
    /// Regras do `tile-manifest.schema.json`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if !is_valid_tile_id(&self.tile_id) {
            return Err(err("tileId", format!("'{}' não casa com ^[a-z0-9][a-z0-9._-]{{0,127}}$", self.tile_id)));
        }
        self.content.validate()?;
        if let Some(link) = self.activation.as_ref().and_then(|a| a.deep_link.as_deref()) {
            if !link.starts_with("conexus://") {
                return Err(err("activation.deepLink", "não usa o esquema conexus://"));
            }
        }
        if let Some(update) = &self.update {
            // Mesma regra do schema: periodMinutes >= 15 sempre que presente, e obrigatório em periodic.
            match update.period_minutes {
                Some(m) if m < 15 => return Err(err("update.periodMinutes", "deve ser ≥ 15")),
                None if update.mode == UpdateMode::Periodic => {
                    return Err(err("update.periodMinutes", "obrigatório em modo periodic"))
                }
                _ => {}
            }
        }
        if let Some(bad) = self.badges.iter().find(|b| b.chars().count() > 3) {
            return Err(err("badges", format!("badge '{bad}' tem mais de 3 caracteres")));
        }
        Ok(())
    }
}

/// `^[a-z0-9][a-z0-9._-]{0,127}$`
pub(crate) fn is_valid_tile_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 128
        && id.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-'))
        && id.chars().next().map_or(false, |c| c.is_ascii_lowercase() || c.is_ascii_digit())
}
