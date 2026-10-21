use serde::{Deserialize, Serialize};

use crate::{err, ValidationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum NotificationStyle {
    Messaging,
    Media,
    Progress,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ActionType {
    Button,
    Input,
    InlineReply,
    DeepLink,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct NotificationAction {
    #[serde(rename = "type")]
    pub kind: ActionType,
    pub label: String,
    #[serde(rename = "deepLink", default, skip_serializing_if = "Option::is_none")]
    pub deep_link: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Notification {
    pub id: String,
    pub channel: String,
    pub title: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub body: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub style: Option<NotificationStyle>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub badge: Option<u32>,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub actions: Vec<NotificationAction>,
}

impl Notification {
    /// Regras do `notification.schema.json`.
    pub fn validate(&self) -> Result<(), ValidationError> {
        if self.id.is_empty() || self.id.chars().count() > 128 {
            return Err(err("id", "obrigatório, até 128 caracteres"));
        }
        let channel_ok = !self.channel.is_empty()
            && self.channel.len() <= 64
            && self
                .channel
                .chars()
                .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || matches!(c, '.' | '_' | '-'));
        if !channel_ok {
            return Err(err("channel", format!("'{}' inválido", self.channel)));
        }
        if self.title.is_empty() {
            return Err(err("title", "obrigatório"));
        }
        if self.title.chars().count() > 200 {
            return Err(err("title", "mais de 200 caracteres"));
        }
        if let Some(body) = &self.body {
            if body.chars().count() > 2000 {
                return Err(err("body", "mais de 2000 caracteres"));
            }
        }
        if self.actions.len() > 4 {
            return Err(err("actions", "máximo de 4 ações"));
        }
        for action in &self.actions {
            if action.label.chars().count() > 40 {
                return Err(err("actions.label", "mais de 40 caracteres"));
            }
            if let Some(link) = &action.deep_link {
                if !link.starts_with("conexus://") {
                    return Err(err("actions.deepLink", "não usa o esquema conexus://"));
                }
            }
        }
        Ok(())
    }
}
