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
///
/// `message` é sempre o resultado de [`signing_message`]: cobre o manifest, os metadados
/// da variante e o payload, e não só o payload.
pub trait SignatureVerifier {
    fn verify(&self, message: &[u8], signature: &[u8]) -> bool;
}

/// Encoding canônico do que é assinado: prefixo de versão, cada campo com tamanho
/// de 4 bytes (big-endian) e o payload com tamanho de 8 bytes. Trocar `kmi`, `kind`,
/// `name`, `device` ou `kernel_release` de um pacote assinado invalida a assinatura.
/// Pacotes fora de bundle usam `device` e `kernel_release` vazios.
pub fn signing_message(manifest: &DriverManifest, device: &str, kernel_release: &str, payload: &[u8]) -> Vec<u8> {
    fn field(out: &mut Vec<u8>, bytes: &[u8]) {
        out.extend_from_slice(&(bytes.len() as u32).to_be_bytes());
        out.extend_from_slice(bytes);
    }
    let mut out = b"conexus-driver-v1\0".to_vec();
    field(&mut out, manifest.name.as_bytes());
    field(&mut out, manifest.version.as_bytes());
    field(&mut out, manifest.component.as_bytes());
    field(&mut out, &[match manifest.kind {
        DriverKind::Userspace => 0,
        DriverKind::KernelModule => 1,
    }]);
    field(&mut out, manifest.kmi.as_deref().unwrap_or("").as_bytes());
    field(&mut out, device.as_bytes());
    field(&mut out, kernel_release.as_bytes());
    out.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    out.extend_from_slice(payload);
    out
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
    /// Nenhuma variante do bundle serve para este aparelho e este kernel.
    NoMatchingVariant { device: String, kernel_release: String },
    /// Mais de uma variante serve; o bundle é ambíguo e é recusado.
    AmbiguousVariant { device: String, kernel_release: String },
    /// O KMI do módulo não é o do kernel instalado.
    KmiMismatch { expected: String, found: String },
}

/// Identificação do aparelho onde o bundle será instalado.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeviceInfo {
    /// Codinome do device, ex.: `akita`.
    pub device: String,
    /// `uname -r` do kernel em execução.
    pub kernel_release: String,
    /// KMI do kernel em execução, ex.: `android14-6.1`.
    pub kmi: String,
}

/// Uma variante do bundle: pacote compilado para um device e um kernel exatos.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverVariant {
    pub device: String,
    pub kernel_release: String,
    pub package: DriverPackage,
}

/// Contêiner entregue num APK (ADR-014): reúne as variantes de uma marca.
/// O APK só transporta o bundle; quem verifica e carrega é o driver manager,
/// serviço de sistema. O APK em si não recebe privilégio de kernel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DriverBundle {
    pub brand: String,
    pub variants: Vec<DriverVariant>,
}

impl From<ValidationError> for DriverError {
    fn from(e: ValidationError) -> Self {
        DriverError::Invalid(e)
    }
}

#[derive(Debug)]
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

    /// Instala um pacote avulso (fora de bundle).
    pub fn install(&mut self, package: DriverPackage) -> Result<(), DriverError> {
        self.install_with(&package, "", "")
    }

    /// Ordem das checagens: assinatura → manifest → duplicata. Um pacote não assinado
    /// não descobre se o nome já existe. Só o manifest é guardado; o payload não é copiado.
    pub(crate) fn install_with(
        &mut self,
        package: &DriverPackage,
        device: &str,
        kernel_release: &str,
    ) -> Result<(), DriverError> {
        self.check_install(package, device, kernel_release)?;
        let manifest = &package.manifest;
        self.installed.insert(manifest.name.clone(), manifest.clone());
        Ok(())
    }

    /// Faz todas as checagens de `install_with` sem alterar o estado.
    fn check_install(&self, package: &DriverPackage, device: &str, kernel_release: &str) -> Result<(), DriverError> {
        let manifest = &package.manifest;
        let signature = package
            .signature
            .as_deref()
            .ok_or_else(|| DriverError::Unsigned(manifest.name.clone()))?;
        let message = signing_message(manifest, device, kernel_release, &package.payload);
        if !self.verifier.verify(&message, signature) {
            return Err(DriverError::BadSignature(manifest.name.clone()));
        }
        validate_manifest(manifest)?;
        if self.installed.contains_key(&manifest.name) {
            return Err(DriverError::AlreadyInstalled(manifest.name.clone()));
        }
        Ok(())
    }

    /// Instala **todas** as variantes do bundle que correspondem ao aparelho e ao kernel
    /// (um driver por componente, ADR-009), de forma atômica: se qualquer uma falhar nas
    /// checagens, nenhuma é instalada. Duas variantes com o mesmo nome tornam o bundle ambíguo.
    pub fn install_bundle(&mut self, bundle: &DriverBundle, device: &DeviceInfo) -> Result<(), DriverError> {
        let selected: Vec<&DriverVariant> = bundle
            .variants
            .iter()
            .filter(|v| v.device == device.device && v.kernel_release == device.kernel_release)
            .collect();
        if selected.is_empty() {
            return Err(DriverError::NoMatchingVariant {
                device: device.device.clone(),
                kernel_release: device.kernel_release.clone(),
            });
        }
        for (i, v) in selected.iter().enumerate() {
            if selected[..i].iter().any(|o| o.package.manifest.name == v.package.manifest.name) {
                return Err(DriverError::AmbiguousVariant {
                    device: device.device.clone(),
                    kernel_release: device.kernel_release.clone(),
                });
            }
        }
        for v in &selected {
            let m = &v.package.manifest;
            if m.kind == DriverKind::KernelModule {
                match m.kmi.as_deref() {
                    Some(kmi) if kmi != device.kmi => {
                        return Err(DriverError::KmiMismatch { expected: device.kmi.clone(), found: kmi.to_string() })
                    }
                    Some(_) => {}
                    None => return Err(DriverError::Invalid(ValidationError { field: "kmi", reason: "módulo de kernel precisa declarar o KMI".into() })),
                }
            }
            self.check_install(&v.package, &v.device, &v.kernel_release)?;
        }
        for v in selected {
            self.installed.insert(v.package.manifest.name.clone(), v.package.manifest.clone());
        }
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
    let token_ok = |s: &str| {
        !s.is_empty() && s.len() <= 64 && s.chars().all(|c| c.is_ascii_alphanumeric() || matches!(c, '.' | '-' | '_' | '+'))
    };
    if !token_ok(&m.version) {
        return Err(ValidationError { field: "version", reason: format!("'{}' inválida", m.version) });
    }
    if !token_ok(&m.component) {
        return Err(ValidationError { field: "component", reason: "obrigatório, até 64 caracteres [A-Za-z0-9._+-]".into() });
    }
    if m.kmi.as_deref().map_or(false, |k| !token_ok(k)) {
        return Err(ValidationError { field: "kmi", reason: "KMI inválido".into() });
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

impl std::fmt::Display for DriverError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{self:?}")
    }
}

impl std::error::Error for DriverError {}
