use std::collections::BTreeMap;

use conexus_sdk::tile::{TileContent, TileManifest};
use conexus_sdk::ValidationError;

/// Erros do tile engine.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TileError {
    Invalid(ValidationError),
    AlreadyRegistered(String),
    NotFound(String),
}

impl From<ValidationError> for TileError {
    fn from(e: ValidationError) -> Self {
        TileError::Invalid(e)
    }
}

/// Estado de um tile no sistema. Guarda o manifest e o último conteúdo publicado.
#[derive(Debug, Clone, PartialEq)]
pub struct TileState {
    pub manifest: TileManifest,
    pub content: TileContent,
    pub revision: u64,
}

/// Tile engine: registro e atualização de tiles, em ordem estável por `tileId`.
#[derive(Debug, Default)]
pub struct TileEngine {
    tiles: BTreeMap<String, TileState>,
}

impl TileEngine {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registra um tile novo após validar o manifest.
    pub fn register(&mut self, manifest: TileManifest) -> Result<(), TileError> {
        manifest.validate()?;
        if self.tiles.contains_key(&manifest.tile_id) {
            return Err(TileError::AlreadyRegistered(manifest.tile_id));
        }
        let content = manifest.content.clone();
        let id = manifest.tile_id.clone();
        self.tiles.insert(id, TileState { manifest, content, revision: 0 });
        Ok(())
    }

    /// Publica conteúdo novo para um tile já registrado. O template não pode mudar.
    pub fn update(&mut self, tile_id: &str, content: TileContent) -> Result<u64, TileError> {
        content.validate()?;
        let state = self
            .tiles
            .get_mut(tile_id)
            .ok_or_else(|| TileError::NotFound(tile_id.to_string()))?;
        if content.template != state.manifest.content.template {
            return Err(TileError::Invalid(ValidationError {
                field: "content.template",
                reason: "não pode mudar após o registro".into(),
            }));
        }
        state.content = content;
        state.revision += 1;
        Ok(state.revision)
    }

    pub fn remove(&mut self, tile_id: &str) -> Result<TileState, TileError> {
        self.tiles
            .remove(tile_id)
            .ok_or_else(|| TileError::NotFound(tile_id.to_string()))
    }

    pub fn get(&self, tile_id: &str) -> Option<&TileState> {
        self.tiles.get(tile_id)
    }

    /// Lista os tiles em ordem de `tileId`.
    pub fn list(&self) -> impl Iterator<Item = &TileState> {
        self.tiles.values()
    }
}

impl std::fmt::Display for TileError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for TileError {}
