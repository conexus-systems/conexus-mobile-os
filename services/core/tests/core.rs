use conexus_core::apps::{AppError, AppManager, LifecycleState};
use conexus_core::drivers::{
    DeviceInfo, DriverBundle, DriverError, DriverKind, DriverManager, DriverManifest, DriverPackage, DriverVariant,
    SignatureVerifier,
};
use conexus_core::notifications::{NotificationBroker, NotificationError, MAX_ACTIVE};
use conexus_core::tiles::{TileEngine, TileError};
use conexus_sdk::app::{AppManifest, EntryPoint, Permission, Runtime};
use conexus_sdk::notification::{Notification, NotificationAction};
use conexus_sdk::tile::{TileContent, TileManifest, TileSize, TileTemplate, TileUpdate, UpdateMode};

fn tile(id: &str, template: TileTemplate) -> TileManifest {
    TileManifest {
        tile_id: id.into(),
        size: TileSize::M,
        content: TileContent { template, data: None },
        update: Some(TileUpdate { mode: UpdateMode::Manual, period_minutes: None }),
        badges: vec![],
    }
}

fn app(id: &str, perms: Vec<Permission>) -> AppManifest {
    AppManifest {
        app_id: id.into(),
        version: "1.0.0".into(),
        entry_point: EntryPoint { runtime: Runtime::Kotlin, main: None },
        permissions: perms,
        deep_links: vec![],
        tiles: vec![],
    }
}

fn note(id: &str) -> Notification {
    Notification {
        id: id.into(),
        channel: "mail".into(),
        title: "Olá".into(),
        body: None,
        style: None,
        badge: None,
        actions: vec![],
    }
}

// ---- tiles ----

#[test]
fn tile_registra_atualiza_e_incrementa_revisao() {
    let mut engine = TileEngine::new();
    engine.register(tile("clock.main", TileTemplate::Text)).unwrap();
    let rev = engine
        .update("clock.main", TileContent { template: TileTemplate::Text, data: None })
        .unwrap();
    assert_eq!(rev, 1);
    assert_eq!(engine.get("clock.main").unwrap().revision, 1);
}

#[test]
fn tile_nao_registra_duas_vezes() {
    let mut engine = TileEngine::new();
    engine.register(tile("a.b", TileTemplate::Icon)).unwrap();
    assert_eq!(
        engine.register(tile("a.b", TileTemplate::Icon)),
        Err(TileError::AlreadyRegistered("a.b".into()))
    );
}

#[test]
fn tile_nao_muda_de_template_apos_registro() {
    let mut engine = TileEngine::new();
    engine.register(tile("a.b", TileTemplate::Icon)).unwrap();
    let err = engine
        .update("a.b", TileContent { template: TileTemplate::Media, data: None })
        .unwrap_err();
    assert!(matches!(err, TileError::Invalid(ref e) if e.field == "content.template"));
}

#[test]
fn tile_manifest_invalido_nao_registra() {
    let mut engine = TileEngine::new();
    let mut bad = tile("a.b", TileTemplate::Icon);
    bad.tile_id = "Bad Id".into();
    assert!(matches!(engine.register(bad), Err(TileError::Invalid(_))));
    assert_eq!(engine.list().count(), 0);
}

#[test]
fn tile_list_em_ordem_estavel() {
    let mut engine = TileEngine::new();
    engine.register(tile("z.z", TileTemplate::Icon)).unwrap();
    engine.register(tile("a.a", TileTemplate::Icon)).unwrap();
    let ids: Vec<_> = engine.list().map(|t| t.manifest.tile_id.clone()).collect();
    assert_eq!(ids, vec!["a.a", "z.z"]);
}

// ---- notificações ----

#[test]
fn notificacao_mesmo_id_substitui() {
    let mut broker = NotificationBroker::new();
    broker.post(note("n1")).unwrap();
    let mut updated = note("n1");
    updated.title = "Novo".into();
    broker.post(updated).unwrap();
    assert_eq!(broker.list().count(), 1);
    assert_eq!(broker.get("n1").unwrap().title, "Novo");
}

#[test]
fn notificacao_respeita_limite_de_ativas() {
    let mut broker = NotificationBroker::new();
    for i in 0..MAX_ACTIVE {
        broker.post(note(&format!("n{i}"))).unwrap();
    }
    assert_eq!(broker.post(note("extra")), Err(NotificationError::Full));
    // Substituir um id existente continua permitido com a fila cheia.
    broker.post(note("n0")).unwrap();
}

#[test]
fn notificacao_com_acao_deeplink_invalida_nao_entra() {
    let mut broker = NotificationBroker::new();
    let mut n = note("n1");
    n.actions.push(NotificationAction {
        kind: conexus_sdk::notification::ActionType::DeepLink,
        label: "Abrir".into(),
        deep_link: Some("https://x".into()),
    });
    assert!(matches!(broker.post(n), Err(NotificationError::Invalid(_))));
    assert!(broker.get("n1").is_none());
}

#[test]
fn notificacao_cancel_remove_e_cancel_inexistente_falha() {
    let mut broker = NotificationBroker::new();
    broker.post(note("n1")).unwrap();
    broker.cancel("n1").unwrap();
    assert!(broker.get("n1").is_none());
    assert_eq!(broker.cancel("n1"), Err(NotificationError::NotFound("n1".into())));
}

// ---- apps ----

#[test]
fn app_instala_e_alterna_ciclo_de_vida() {
    let mut mgr = AppManager::new();
    mgr.install(app("systems.conexus.ref", vec![])).unwrap();
    assert_eq!(mgr.get("systems.conexus.ref").unwrap().state, LifecycleState::Stopped);
    mgr.launch("systems.conexus.ref").unwrap();
    assert_eq!(mgr.get("systems.conexus.ref").unwrap().state, LifecycleState::Running);
    mgr.stop("systems.conexus.ref").unwrap();
    assert_eq!(mgr.get("systems.conexus.ref").unwrap().state, LifecycleState::Stopped);
}

#[test]
fn app_nao_instala_duas_vezes() {
    let mut mgr = AppManager::new();
    mgr.install(app("systems.conexus.ref", vec![])).unwrap();
    assert_eq!(
        mgr.install(app("systems.conexus.ref", vec![])),
        Err(AppError::AlreadyInstalled("systems.conexus.ref".into()))
    );
}

#[test]
fn app_lancar_sem_instalar_falha() {
    let mut mgr = AppManager::new();
    assert_eq!(mgr.launch("systems.x.y"), Err(AppError::NotInstalled("systems.x.y".into())));
}

#[test]
fn app_permissao_so_existe_se_declarada() {
    let mut mgr = AppManager::new();
    mgr.install(app("systems.conexus.ref", vec![Permission::TileUpdate])).unwrap();
    mgr.check_permission("systems.conexus.ref", Permission::TileUpdate).unwrap();
    assert!(matches!(
        mgr.check_permission("systems.conexus.ref", Permission::Camera),
        Err(AppError::PermissionDenied { .. })
    ));
}

// ---- drivers ----

/// Verificador de teste: aceita só a assinatura `b"ok"` sobre qualquer payload.
struct AcceptOk;
impl SignatureVerifier for AcceptOk {
    fn verify(&self, _payload: &[u8], signature: &[u8]) -> bool {
        signature == b"ok"
    }
}

fn driver(name: &str, kind: DriverKind, kmi: Option<&str>, sig: Option<&[u8]>) -> DriverPackage {
    DriverPackage {
        manifest: DriverManifest {
            name: name.into(),
            version: "1.0.0".into(),
            component: "camera".into(),
            kind,
            kmi: kmi.map(str::to_string),
        },
        payload: b"bytes".to_vec(),
        signature: sig.map(<[u8]>::to_vec),
    }
}

#[test]
fn driver_sem_verificador_configurado_nao_instala() {
    let mut mgr = DriverManager::default();
    let err = mgr
        .install(driver("pixel-8a-camera", DriverKind::Userspace, None, Some(b"ok")))
        .unwrap_err();
    assert!(matches!(err, DriverError::BadSignature(_)));
}

#[test]
fn driver_sem_assinatura_nao_instala() {
    let mut mgr = DriverManager::new(AcceptOk);
    let err = mgr
        .install(driver("pixel-8a-camera", DriverKind::Userspace, None, None))
        .unwrap_err();
    assert_eq!(err, DriverError::Unsigned("pixel-8a-camera".into()));
}

#[test]
fn driver_com_assinatura_valida_instala_e_nao_duplica() {
    let mut mgr = DriverManager::new(AcceptOk);
    mgr.install(driver("pixel-8a-camera", DriverKind::Userspace, None, Some(b"ok"))).unwrap();
    assert!(mgr.get("pixel-8a-camera").is_some());
    assert_eq!(
        mgr.install(driver("pixel-8a-camera", DriverKind::Userspace, None, Some(b"ok"))),
        Err(DriverError::AlreadyInstalled("pixel-8a-camera".into()))
    );
}

#[test]
fn modulo_de_kernel_exige_kmi() {
    let mut mgr = DriverManager::new(AcceptOk);
    let err = mgr
        .install(driver("pixel-8a-touch", DriverKind::KernelModule, None, Some(b"ok")))
        .unwrap_err();
    assert!(matches!(err, DriverError::Invalid(ref e) if e.field == "kmi"));
}

#[test]
fn userspace_nao_declara_kmi() {
    let mut mgr = DriverManager::new(AcceptOk);
    let err = mgr
        .install(driver("pixel-8a-touch", DriverKind::Userspace, Some("android14-6.1"), Some(b"ok")))
        .unwrap_err();
    assert!(matches!(err, DriverError::Invalid(ref e) if e.field == "kmi"));
}

#[test]
fn modulo_de_kernel_com_kmi_e_assinatura_instala() {
    let mut mgr = DriverManager::new(AcceptOk);
    mgr.install(driver("pixel-8a-touch", DriverKind::KernelModule, Some("android14-6.1"), Some(b"ok")))
        .unwrap();
    assert_eq!(mgr.get("pixel-8a-touch").unwrap().kmi.as_deref(), Some("android14-6.1"));
}

// ---- bundles (APK de uma marca, ADR-014) ----

fn variant(device: &str, kernel: &str, kmi: &str, name: &str, sig: Option<&[u8]>) -> DriverVariant {
    DriverVariant {
        device: device.into(),
        kernel_release: kernel.into(),
        package: driver(name, DriverKind::KernelModule, Some(kmi), sig),
    }
}

fn akita() -> DeviceInfo {
    DeviceInfo { device: "akita".into(), kernel_release: "6.1.99-android14".into(), kmi: "android14-6.1".into() }
}

#[test]
fn bundle_instala_so_a_variante_do_aparelho() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![
            variant("akita", "6.1.99-android14", "android14-6.1", "akita-touch", Some(b"ok")),
            variant("other", "5.15.0", "android13-5.15", "other-touch", Some(b"ok")),
        ],
    };
    let mut mgr = DriverManager::new(AcceptOk);
    mgr.install_bundle(&bundle, &akita()).unwrap();
    assert!(mgr.get("akita-touch").is_some());
    assert!(mgr.get("other-touch").is_none());
}

#[test]
fn bundle_sem_variante_para_o_aparelho_e_recusado() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![variant("other", "5.15.0", "android13-5.15", "other-touch", Some(b"ok"))],
    };
    let mut mgr = DriverManager::new(AcceptOk);
    assert!(matches!(
        mgr.install_bundle(&bundle, &akita()),
        Err(DriverError::NoMatchingVariant { .. })
    ));
}

#[test]
fn bundle_com_kernel_diferente_nao_serve() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![variant("akita", "6.1.50-android14", "android14-6.1", "akita-touch", Some(b"ok"))],
    };
    let mut mgr = DriverManager::new(AcceptOk);
    assert!(matches!(
        mgr.install_bundle(&bundle, &akita()),
        Err(DriverError::NoMatchingVariant { .. })
    ));
}

#[test]
fn bundle_ambiguo_e_recusado() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![
            variant("akita", "6.1.99-android14", "android14-6.1", "akita-touch", Some(b"ok")),
            variant("akita", "6.1.99-android14", "android14-6.1", "akita-camera", Some(b"ok")),
        ],
    };
    let mut mgr = DriverManager::new(AcceptOk);
    assert!(matches!(
        mgr.install_bundle(&bundle, &akita()),
        Err(DriverError::AmbiguousVariant { .. })
    ));
    assert_eq!(mgr.list().count(), 0);
}

#[test]
fn bundle_com_kmi_diferente_do_kernel_e_recusado() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![variant("akita", "6.1.99-android14", "android13-5.15", "akita-touch", Some(b"ok"))],
    };
    let mut mgr = DriverManager::new(AcceptOk);
    assert!(matches!(
        mgr.install_bundle(&bundle, &akita()),
        Err(DriverError::KmiMismatch { .. })
    ));
}

#[test]
fn bundle_sem_assinatura_e_recusado_mesmo_com_variante_certa() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![variant("akita", "6.1.99-android14", "android14-6.1", "akita-touch", None)],
    };
    let mut mgr = DriverManager::new(AcceptOk);
    assert_eq!(mgr.install_bundle(&bundle, &akita()), Err(DriverError::Unsigned("akita-touch".into())));
    assert_eq!(mgr.list().count(), 0);
}

#[test]
fn bundle_sem_verificador_configurado_nao_instala() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![variant("akita", "6.1.99-android14", "android14-6.1", "akita-touch", Some(b"ok"))],
    };
    let mut mgr = DriverManager::default();
    assert!(matches!(mgr.install_bundle(&bundle, &akita()), Err(DriverError::BadSignature(_))));
}
