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
    pub update: Option<TileUpdate>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub badges: Vec<String>,
}

impl TileManifest {
    /// Regras do `tile-manifest.schema.json`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        let id = &self.tile_id;
        let valid_id = !id.is_empty()
            && id.len() <= 128
            && id
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-'))
            && id.chars().next().map_or(false, |c| c.is_ascii_lowercase() || c.is_ascii_digit());
        if !valid_id {
            return Err(err("tileId", format!("'{id}' não casa com ^[a-z0-9][a-z0-9._-]{{0,127}}$")));
        }
        if let Some(update) = &self.update {
            if update.mode == UpdateMode::Periodic {
                match update.period_minutes {
                    Some(m) if m >= 15 => {}
                    _ => return Err(err("update.periodMinutes", "obrigatório e ≥ 15 em modo periodic")),
                }
            }
        }
        if let Some(bad) = self.badges.iter().find(|b| b.chars().count() > 3) {
            return Err(err("badges", format!("badge '{bad}' tem mais de 3 caracteres")));
        }
        Ok(())
    }
}
