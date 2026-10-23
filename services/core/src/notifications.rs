use std::collections::BTreeMap;

use conexus_sdk::notification::Notification;
use conexus_sdk::ValidationError;

/// Limite de notificações ativas por sistema (regra de estabilidade do shell).
pub const MAX_ACTIVE: usize = 100;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotificationError {
    Invalid(ValidationError),
    Full,
    NotFound(String),
}

impl From<ValidationError> for NotificationError {
    fn from(e: ValidationError) -> Self {
        NotificationError::Invalid(e)
    }
}

/// Broker de notificações: post, atualização por `id` e cancelamento.
#[derive(Debug, Default)]
pub struct NotificationBroker {
    active: BTreeMap<String, Notification>,
}

impl NotificationBroker {
    pub fn new() -> Self {
        Self::default()
    }

    /// Publica uma notificação. Se o `id` já existe, substitui a anterior.
    pub fn post(&mut self, notification: Notification) -> Result<(), NotificationError> {
        notification.validate()?;
        if !self.active.contains_key(&notification.id) && self.active.len() >= MAX_ACTIVE {
            return Err(NotificationError::Full);
        }
        self.active.insert(notification.id.clone(), notification);
        Ok(())
    }

    pub fn cancel(&mut self, id: &str) -> Result<Notification, NotificationError> {
        self.active
            .remove(id)
            .ok_or_else(|| NotificationError::NotFound(id.to_string()))
    }

    pub fn get(&self, id: &str) -> Option<&Notification> {
        self.active.get(id)
    }

    pub fn list(&self) -> impl Iterator<Item = &Notification> {
        self.active.values()
    }
}

impl std::fmt::Display for NotificationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for NotificationError {}
