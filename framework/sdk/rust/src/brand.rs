//! Descritor de módulo de marca (`brands/<marca>.json`, `brand-module.schema.json`).
//!
//! O descritor aponta para o código do `driver-core` e registra a evidência por modelo.
//! Um módulo só pode ser `verified` com os cinco critérios do ADR-010 (C1–C5) aprovados,
//! cada um com fonte HTTPS e data. Sem isso, o máximo é `unverified`.

use serde::{Deserialize, Serialize};

use crate::{err, ValidationError};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Brand {
    Google,
    Nothing,
    Oneplus,
    Oppo,
    Samsung,
    Xiaomi,
    Motorola,
    Tcl,
    Realme,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ModuleStatus {
    Planned,
    Unverified,
    Verified,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Evidence {
    pub ok: bool,
    pub source: String,
    pub date: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Criteria {
    pub c1: Evidence,
    pub c2: Evidence,
    pub c3: Evidence,
    pub c4: Evidence,
    pub c5: Evidence,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrandDevice {
    pub codename: String,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub criteria: Option<Criteria>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BrandModule {
    pub brand: Brand,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub sub_brands: Vec<String>,
    pub status: ModuleStatus,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub hal_source: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub devices: Option<Vec<BrandDevice>>,
}

const HAL_PREFIX: &str = "driver-core/vendors/src/main/java/com/conexus/driver/vendors/";

impl BrandModule {
    pub fn validate(&self) -> Result<(), ValidationError> {
        if let Some(src) = &self.hal_source {
            let rest = src.strip_prefix(HAL_PREFIX).unwrap_or("");
            let mut parts = rest.split('/');
            let (dir, file) = (parts.next().unwrap_or(""), parts.next().unwrap_or(""));
            let dir_ok = !dir.is_empty() && dir.chars().all(|c| c.is_ascii_lowercase());
            let file_ok = file.ends_with(".java")
                && file.len() > 5
                && file[..file.len() - 5].chars().all(|c| c.is_ascii_alphanumeric());
            if !(dir_ok && file_ok && parts.next().is_none()) {
                return Err(err("hal_source", format!("'{src}' fora do padrão do driver-core")));
            }
        }
        for s in &self.sub_brands {
            let ok = (2..=32).contains(&s.len())
                && s.starts_with(|c: char| c.is_ascii_lowercase())
                && s.chars().all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
            if !ok {
                return Err(err("sub_brands", format!("'{s}' inválida")));
            }
        }
        if self.sub_brands.iter().enumerate().any(|(i, s)| self.sub_brands[..i].contains(s)) {
            return Err(err("sub_brands", "valor repetido"));
        }
        match self.status {
            ModuleStatus::Planned if self.devices.is_some() => {
                Err(err("devices", "módulo planned não declara aparelhos"))
            }
            ModuleStatus::Verified => {
                let devices = self
                    .devices
                    .as_deref()
                    .filter(|d| !d.is_empty())
                    .ok_or_else(|| err("devices", "módulo verified exige ao menos um aparelho"))?;
                for d in devices {
                    let c = d
                        .criteria
                        .as_ref()
                        .ok_or_else(|| err("devices.criteria", format!("{}: critérios C1–C5 obrigatórios", d.codename)))?;
                    for (name, e) in [("c1", &c.c1), ("c2", &c.c2), ("c3", &c.c3), ("c4", &c.c4), ("c5", &c.c5)] {
                        if !e.ok {
                            return Err(err("devices.criteria", format!("{}: {name} não aprovado", d.codename)));
                        }
                    }
                }
                Self::check_devices(devices)
            }
            _ => self.devices.as_deref().map_or(Ok(()), Self::check_devices),
        }
    }

    fn check_devices(devices: &[BrandDevice]) -> Result<(), ValidationError> {
        if devices.len() > 256 {
            return Err(err("devices", "máximo de 256 aparelhos"));
        }
        for d in devices {
            let n = &d.codename;
            let ok = !n.is_empty()
                && n.len() <= 64
                && n.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '_' | '-' | '.'))
                && n.starts_with(|c: char| c.is_ascii_alphanumeric());
            if !ok {
                return Err(err("devices.codename", format!("'{n}' inválido")));
            }
            if let Some(c) = &d.criteria {
                for e in [&c.c1, &c.c2, &c.c3, &c.c4, &c.c5] {
                    if !e.source.starts_with("https://") {
                        return Err(err("devices.criteria", "source precisa ser https://"));
                    }
                    let b = e.date.as_bytes();
                    let date_ok = b.len() == 10
                        && b[4] == b'-'
                        && b[7] == b'-'
                        && b.iter().enumerate().all(|(i, c)| i == 4 || i == 7 || c.is_ascii_digit());
                    if !date_ok {
                        return Err(err("devices.criteria", format!("data '{}' fora de AAAA-MM-DD", e.date)));
                    }
                }
            }
        }
        Ok(())
    }
}
