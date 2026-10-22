//! Paridade com os JSON Schemas: lê as mesmas fixtures de `framework/sdk/fixtures`
//! que `framework/sdk/tests_schema_fixtures.py` valida contra os schemas.
//! valid/ precisa ser aceita (desserializa e valida); invalid/ precisa ser recusada.

use std::fs;
use std::path::Path;

use conexus_sdk::app::AppManifest;
use conexus_sdk::brand::BrandModule;
use conexus_sdk::notification::Notification;
use conexus_sdk::tile::TileManifest;

fn accepts(kind: &str, json: &str) -> bool {
    match kind {
        "tile" => serde_json::from_str::<TileManifest>(json).map_or(false, |m| m.validate().is_ok()),
        "app" => serde_json::from_str::<AppManifest>(json).map_or(false, |m| m.validate().is_ok()),
        "brand" => serde_json::from_str::<BrandModule>(json).map_or(false, |m| m.validate().is_ok()),
        "notification" => serde_json::from_str::<Notification>(json).map_or(false, |m| m.validate().is_ok()),
        other => panic!("tipo de fixture desconhecido: {other}"),
    }
}

fn check(folder: &str, expect: bool) -> usize {
    let dir = Path::new(env!("CARGO_MANIFEST_DIR")).join("../fixtures").join(folder);
    let mut n = 0;
    let mut divergent = Vec::new();
    for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
        let path = entry.unwrap().path();
        let name = path.file_name().unwrap().to_string_lossy().to_string();
        let kind = name.split('-').next().unwrap().to_string();
        let json = fs::read_to_string(&path).unwrap();
        n += 1;
        if accepts(&kind, &json) != expect {
            divergent.push(format!("{folder}/{name}"));
        }
    }
    assert!(divergent.is_empty(), "Rust diverge do schema em: {divergent:?}");
    n
}

#[test]
fn fixtures_validas_sao_aceitas() {
    assert!(check("valid", true) >= 10);
}

#[test]
fn fixtures_invalidas_sao_recusadas() {
    assert!(check("invalid", false) >= 30);
}

/// Todo descritor commitado em `brands/` precisa passar no schema e no Rust, e o `hal_source`
/// precisa existir no repositório. `verified` só com evidência, então nenhum marca verified sem critérios.
#[test]
fn descritores_de_marca_do_repositorio_sao_validos() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../../..");
    let dir = root.join("brands");
    let mut n = 0;
    for entry in fs::read_dir(&dir).unwrap_or_else(|e| panic!("{}: {e}", dir.display())) {
        let path = entry.unwrap().path();
        let json = fs::read_to_string(&path).unwrap();
        let m: BrandModule = serde_json::from_str(&json).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        m.validate().unwrap_or_else(|e| panic!("{}: {e}", path.display()));
        if let Some(src) = &m.hal_source {
            assert!(root.join(src).is_file(), "{}: hal_source inexistente: {src}", path.display());
        }
        n += 1;
    }
    assert!(n >= 8, "esperados descritores das 8 marcas, achei {n}");
}
