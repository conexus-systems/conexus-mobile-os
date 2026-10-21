use std::collections::BTreeMap;

use conexus_sdk::app::{AppManifest, Permission};
use conexus_sdk::ValidationError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum AppError {
    Invalid(ValidationError),
    AlreadyInstalled(String),
    NotInstalled(String),
    PermissionDenied { app_id: String, permission: Permission },
}

impl From<ValidationError> for AppError {
    fn from(e: ValidationError) -> Self {
        AppError::Invalid(e)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LifecycleState {
    Stopped,
    Running,
}

#[derive(Debug, Clone, PartialEq)]
pub struct InstalledApp {
    pub manifest: AppManifest,
    pub state: LifecycleState,
}

/// App manager: catálogo de apps instalados, ciclo de vida e checagem de permissão.
#[derive(Debug, Default)]
pub struct AppManager {
    apps: BTreeMap<String, InstalledApp>,
}

impl AppManager {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn install(&mut self, manifest: AppManifest) -> Result<(), AppError> {
        manifest.validate()?;
        if self.apps.contains_key(&manifest.app_id) {
            return Err(AppError::AlreadyInstalled(manifest.app_id));
        }
        let app_id = manifest.app_id.clone();
        self.apps.insert(app_id, InstalledApp { manifest, state: LifecycleState::Stopped });
        Ok(())
    }

    pub fn launch(&mut self, app_id: &str) -> Result<(), AppError> {
        let app = self
            .apps
            .get_mut(app_id)
            .ok_or_else(|| AppError::NotInstalled(app_id.to_string()))?;
        app.state = LifecycleState::Running;
        Ok(())
    }

    pub fn stop(&mut self, app_id: &str) -> Result<(), AppError> {
        let app = self
            .apps
            .get_mut(app_id)
            .ok_or_else(|| AppError::NotInstalled(app_id.to_string()))?;
        app.state = LifecycleState::Stopped;
        Ok(())
    }

    /// Verifica se o app declarou a permissão no manifest. Não concede nada.
    pub fn check_permission(&self, app_id: &str, permission: Permission) -> Result<(), AppError> {
        let app = self
            .apps
            .get(app_id)
            .ok_or_else(|| AppError::NotInstalled(app_id.to_string()))?;
        if app.manifest.permissions.contains(&permission) {
            Ok(())
        } else {
            Err(AppError::PermissionDenied { app_id: app_id.to_string(), permission })
        }
    }

    pub fn get(&self, app_id: &str) -> Option<&InstalledApp> {
        self.apps.get(app_id)
    }

    pub fn list(&self) -> impl Iterator<Item = &InstalledApp> {
        self.apps.values()
    }
}
