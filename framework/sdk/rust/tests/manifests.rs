use conexus_sdk::app::AppManifest;
use conexus_sdk::notification::Notification;
use conexus_sdk::tile::TileManifest;

const TILE: &str = r#"{
  "tileId": "clock.main",
  "size": "W",
  "content": { "template": "text", "data": { "text": "12:00" } },
  "update": { "mode": "periodic", "periodMinutes": 15 }
}"#;

const APP: &str = r#"{
  "appId": "systems.conexus.reference",
  "version": "0.1.0",
  "entryPoint": { "runtime": "kotlin" },
  "permissions": ["tileUpdate"],
  "deepLinks": ["conexus://ref/home"],
  "tiles": ["clock.main"]
}"#;

const NOTIFICATION: &str = r#"{
  "id": "n1",
  "channel": "mail",
  "title": "Olá",
  "actions": [{ "type": "deepLink", "label": "Abrir", "deepLink": "conexus://mail/inbox" }]
}"#;

#[test]
fn exemplos_do_schema_passam() {
    let tile: TileManifest = serde_json::from_str(TILE).unwrap();
    tile.validate().unwrap();

    let app: AppManifest = serde_json::from_str(APP).unwrap();
    app.validate().unwrap();

    let n: Notification = serde_json::from_str(NOTIFICATION).unwrap();
    n.validate().unwrap();
}

#[test]
fn tile_periodic_abaixo_de_15_minutos_e_rejeitado() {
    let json = TILE.replace("15", "5");
    let tile: TileManifest = serde_json::from_str(&json).unwrap();
    let err = tile.validate().unwrap_err();
    assert_eq!(err.field, "update.periodMinutes");
}

#[test]
fn tile_id_com_maiuscula_e_rejeitado() {
    let json = TILE.replace("clock.main", "Clock.Main");
    let tile: TileManifest = serde_json::from_str(&json).unwrap();
    assert_eq!(tile.validate().unwrap_err().field, "tileId");
}

#[test]
fn tamanho_de_tile_fora_do_enum_nao_desserializa() {
    let json = TILE.replace("\"W\"", "\"XL\"");
    assert!(serde_json::from_str::<TileManifest>(&json).is_err());
}

#[test]
fn app_id_sem_dominio_reverso_e_rejeitado() {
    let json = APP.replace("systems.conexus.reference", "reference");
    let app: AppManifest = serde_json::from_str(&json).unwrap();
    assert_eq!(app.validate().unwrap_err().field, "appId");
}

#[test]
fn app_version_que_nao_e_semver_e_rejeitada() {
    for v in ["1.0", "01.0.0", "1.0.0-", "v1.0.0"] {
        let json = APP.replace("\"0.1.0\"", &format!("\"{v}\""));
        let app: AppManifest = serde_json::from_str(&json).unwrap();
        assert_eq!(app.validate().unwrap_err().field, "version", "versão {v}");
    }
}

#[test]
fn app_version_com_pre_release_e_aceita() {
    let json = APP.replace("\"0.1.0\"", "\"1.2.3-rc.1+build.5\"");
    let app: AppManifest = serde_json::from_str(&json).unwrap();
    app.validate().unwrap();
}

#[test]
fn deep_link_sem_esquema_conexus_e_rejeitado() {
    let json = APP.replace("conexus://ref/home", "https://exemplo.com");
    let app: AppManifest = serde_json::from_str(&json).unwrap();
    assert_eq!(app.validate().unwrap_err().field, "deepLinks");
}

#[test]
fn notificacao_com_cinco_acoes_e_rejeitada() {
    let action = r#"{ "type": "button", "label": "x" }"#;
    let json = format!(
        r#"{{ "id": "n", "channel": "mail", "title": "t", "actions": [{a},{a},{a},{a},{a}] }}"#,
        a = action
    );
    let n: Notification = serde_json::from_str(&json).unwrap();
    assert_eq!(n.validate().unwrap_err().field, "actions");
}

#[test]
fn campo_desconhecido_e_rejeitado() {
    let json = APP.replace("\"tiles\"", "\"extra\": 1, \"tiles\"");
    assert!(serde_json::from_str::<AppManifest>(&json).is_err());
}

#[test]
fn tile_data_acima_do_limite_e_rejeitado() {
    use conexus_sdk::tile::MAX_TILE_DATA_BYTES;
    let big = "x".repeat(MAX_TILE_DATA_BYTES);
    let json = TILE.replace("\"12:00\"", &format!("\"{big}\""));
    let tile: TileManifest = serde_json::from_str(&json).unwrap();
    assert_eq!(tile.validate().unwrap_err().field, "content.data");
}
