//! Driver manager (ADR-009, ADR-011): instala pacotes de driver somente se
//! estiverem assinados pela chave de driver do projeto. Sem verificador
//! configurado, nada é aceito (fail closed).

use std::collections::BTreeMap;

use conexus_sdk::ValidationError;

/// Classe do driver: userspace é o padrão; módulo de kernel é exceção (ADR-009).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DriverKind {
    Userspace,
    KernelModule,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverManifest {
    /// Identificador do pacote, ex.: `pixel-8a-camera`.
    pub name: String,
    pub version: String,
    /// Componente de hardware atendido, ex.: `camera`, `touch`.
    pub component: String,
    pub kind: DriverKind,
    /// Versão do KMI exigida (só para módulo de kernel).
    pub kmi: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverPackage {
    pub manifest: DriverManifest,
    /// Bytes do pacote, usados para a verificação da assinatura.
    pub payload: Vec<u8>,
    pub signature: Option<Vec<u8>>,
}

/// Verificador de assinatura. A implementação real usa a chave de driver
/// (ed25519, a definir pelo A5); aqui a interface fica isolada para teste.
pub trait SignatureVerifier {
    fn verify(&self, payload: &[u8], signature: &[u8]) -> bool;
}

/// Verificador que recusa tudo. É o padrão de `DriverManager::new`.
#[derive(Debug, Default)]
pub struct RejectAll;

impl SignatureVerifier for RejectAll {
    fn verify(&self, _payload: &[u8], _signature: &[u8]) -> bool {
        false
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DriverError {
    Invalid(ValidationError),
    Unsigned(String),
    BadSignature(String),
    AlreadyInstalled(String),
}

impl From<ValidationError> for DriverError {
    fn from(e: ValidationError) -> Self {
        DriverError::Invalid(e)
    }
}

pub struct DriverManager<V: SignatureVerifier> {
    verifier: V,
    installed: BTreeMap<String, DriverManifest>,
}

impl Default for DriverManager<RejectAll> {
    fn default() -> Self {
        Self::new(RejectAll)
    }
}

impl<V: SignatureVerifier> DriverManager<V> {
    pub fn new(verifier: V) -> Self {
        Self { verifier, installed: BTreeMap::new() }
    }

    /// Instala um pacote após validar manifest e assinatura.
    pub fn install(&mut self, package: DriverPackage) -> Result<(), DriverError> {
        validate_manifest(&package.manifest)?;
        let name = package.manifest.name.clone();
        if self.installed.contains_key(&name) {
            return Err(DriverError::AlreadyInstalled(name));
        }
        let signature = package
            .signature
            .as_deref()
            .ok_or_else(|| DriverError::Unsigned(name.clone()))?;
        if !self.verifier.verify(&package.payload, signature) {
            return Err(DriverError::BadSignature(name));
        }
        self.installed.insert(name, package.manifest);
        Ok(())
    }

    pub fn get(&self, name: &str) -> Option<&DriverManifest> {
        self.installed.get(name)
    }

    pub fn list(&self) -> impl Iterator<Item = &DriverManifest> {
        self.installed.values()
    }
}

fn validate_manifest(m: &DriverManifest) -> Result<(), ValidationError> {
    let name_ok = !m.name.is_empty()
        && m.name.len() <= 128
        && m.name
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-');
    if !name_ok {
        return Err(ValidationError { field: "name", reason: format!("'{}' inválido", m.name) });
    }
    if m.component.is_empty() {
        return Err(ValidationError { field: "component", reason: "obrigatório".into() });
    }
    if m.kind == DriverKind::KernelModule && m.kmi.as_deref().map_or(true, str::is_empty) {
        return Err(ValidationError {
            field: "kmi",
            reason: "módulo de kernel precisa declarar o KMI".into(),
        });
    }
    if m.kind == DriverKind::Userspace && m.kmi.is_some() {
        return Err(ValidationError {
            field: "kmi",
            reason: "driver de userspace não declara KMI".into(),
        });
    }
    Ok(())
}
