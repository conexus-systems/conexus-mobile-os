use conexus_core::apps::{AppError, AppManager, LifecycleState};
use conexus_core::drivers::{
    signing_message, DeviceInfo, DriverBundle, DriverError, DriverKind, DriverManager, DriverManifest, DriverPackage,
    DriverVariant, SignatureVerifier,
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
        activation: None,
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
fn bundle_instala_todos_os_componentes_do_aparelho() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![
            variant("akita", "6.1.99-android14", "android14-6.1", "akita-touch", Some(b"ok")),
            variant("akita", "6.1.99-android14", "android14-6.1", "akita-camera", Some(b"ok")),
            variant("other", "5.15.0", "android13-5.15", "other-touch", Some(b"ok")),
        ],
    };
    let mut mgr = DriverManager::new(AcceptOk);
    mgr.install_bundle(&bundle, &akita()).unwrap();
    assert!(mgr.get("akita-touch").is_some() && mgr.get("akita-camera").is_some());
    assert!(mgr.get("other-touch").is_none());
}

#[test]
fn bundle_com_dois_pacotes_de_mesmo_nome_e_ambiguo() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![
            variant("akita", "6.1.99-android14", "android14-6.1", "akita-touch", Some(b"ok")),
            variant("akita", "6.1.99-android14", "android14-6.1", "akita-touch", Some(b"ok")),
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
fn bundle_e_atomico_se_um_componente_falha() {
    let bundle = DriverBundle {
        brand: "google".into(),
        variants: vec![
            variant("akita", "6.1.99-android14", "android14-6.1", "akita-touch", Some(b"ok")),
            variant("akita", "6.1.99-android14", "android14-6.1", "akita-camera", None),
        ],
    };
    let mut mgr = DriverManager::new(AcceptOk);
    assert_eq!(mgr.install_bundle(&bundle, &akita()), Err(DriverError::Unsigned("akita-camera".into())));
    assert_eq!(mgr.list().count(), 0, "nada instalado quando um componente falha");
}

#[test]
fn rotulo_de_variante_trocado_invalida_a_assinatura() {
    // Assinado para o device `akita`; o atacante re-rotula a variante para `other`.
    let signed_for_akita = variant("akita", "6.1.99-android14", "android14-6.1", "akita-touch", Some(b"sig"));
    let message = signing_message(
        &signed_for_akita.package.manifest, "akita", "6.1.99-android14", &signed_for_akita.package.payload,
    );
    let mut relabeled = signed_for_akita.clone();
    relabeled.device = "other".into();
    let bundle = DriverBundle { brand: "google".into(), variants: vec![relabeled] };
    let mut mgr = DriverManager::new(ExactMessage(message));
    let other = DeviceInfo { device: "other".into(), kernel_release: "6.1.99-android14".into(), kmi: "android14-6.1".into() };
    assert!(matches!(mgr.install_bundle(&bundle, &other), Err(DriverError::BadSignature(_))));
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

// ---- assinatura cobre manifest e variante ----

/// Aceita só a mensagem exata que foi "assinada": simula uma assinatura real sobre `signing_message`.
struct ExactMessage(Vec<u8>);
impl SignatureVerifier for ExactMessage {
    fn verify(&self, message: &[u8], _signature: &[u8]) -> bool {
        message == self.0.as_slice()
    }
}

#[test]
fn assinatura_invalida_se_o_kmi_do_manifest_muda() {
    let original = driver("pixel-8a-touch", DriverKind::KernelModule, Some("android14-6.1"), Some(b"sig"));
    let signed = signing_message(&original.manifest, "", "", &original.payload);
    let mut mgr = DriverManager::new(ExactMessage(signed.clone()));
    // Mesmo payload e mesma assinatura, KMI trocado: tem que ser recusado.
    let mut tampered = original.clone();
    tampered.manifest.kmi = Some("android13-5.15".into());
    assert!(matches!(mgr.install(tampered), Err(DriverError::BadSignature(_))));
    // O original passa.
    mgr.install(original).unwrap();
}

#[test]
fn assinatura_invalida_se_kind_ou_nome_mudam() {
    let original = driver("pixel-8a-touch", DriverKind::KernelModule, Some("android14-6.1"), Some(b"sig"));
    let signed = signing_message(&original.manifest, "", "", &original.payload);
    for tweak in [
        |p: &mut DriverPackage| p.manifest.kind = DriverKind::Userspace,
        |p: &mut DriverPackage| p.manifest.name = "other-driver".into(),
        |p: &mut DriverPackage| p.payload.push(0),
    ] {
        let mut mgr = DriverManager::new(ExactMessage(signed.clone()));
        let mut t = original.clone();
        tweak(&mut t);
        assert!(matches!(mgr.install(t), Err(DriverError::BadSignature(_))));
    }
}

#[test]
fn pacote_nao_assinado_nao_revela_que_o_nome_existe() {
    let mut mgr = DriverManager::new(AcceptOk);
    mgr.install(driver("pixel-8a-camera", DriverKind::Userspace, None, Some(b"ok"))).unwrap();
    let err = mgr.install(driver("pixel-8a-camera", DriverKind::Userspace, None, None)).unwrap_err();
    assert_eq!(err, DriverError::Unsigned("pixel-8a-camera".into()));
}

#[test]
fn tile_update_com_data_grande_e_rejeitado() {
    let mut engine = TileEngine::new();
    engine.register(tile("a.b", TileTemplate::Text)).unwrap();
    let big = serde_json::json!({ "t": "x".repeat(conexus_sdk::tile::MAX_TILE_DATA_BYTES) });
    let err = engine
        .update("a.b", TileContent { template: TileTemplate::Text, data: Some(big) })
        .unwrap_err();
    assert!(matches!(err, TileError::Invalid(ref e) if e.field == "content.data"));
}

#[test]
fn erros_implementam_display_e_error() {
    fn assert_error<E: std::error::Error>(_: &E) {}
    let e = TileError::NotFound("x".into());
    assert_error(&e);
    assert!(e.to_string().contains("NotFound"));
}
