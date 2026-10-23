//! Conexus SDK em Rust: tipos e validação dos manifests do M1.
//!
//! Os schemas de referência estão em `framework/sdk/shared/`. Este crate
//! implementa as mesmas regras em código, para que serviços do sistema
//! (tile-engine, notifications, app-manager) validem manifests da mesma forma.

pub mod app;
pub mod notification;
pub mod tile;

/// Erro de validação de um manifest.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidationError {
    pub field: &'static str,
    pub reason: String,
}

impl std::fmt::Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.field, self.reason)
    }
}

impl std::error::Error for ValidationError {}

pub(crate) fn err(field: &'static str, reason: impl Into<String>) -> ValidationError {
    ValidationError {
        field,
        reason: reason.into(),
    }
}
