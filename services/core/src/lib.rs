//! Core de sistema do Conexus Mobile OS em Rust.
//!
//! Cada módulo corresponde a um serviço do SDD §7 (`conexus-tile-engine`,
//! `conexus-notifications`, `conexus-app-manager`) e ao driver manager do
//! ADR-009. Esta crate contém só a lógica de estado e as regras de validação,
//! sem IPC. A camada Binder/AIDL (Android) e D-Bus (Lumia) fica por cima.

pub mod apps;
pub mod drivers;
pub mod notifications;
pub mod tiles;

pub use conexus_sdk::ValidationError;
