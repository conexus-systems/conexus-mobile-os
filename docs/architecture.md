# Arquitetura Revisada — Conexus Harmattan

**Projeto:** Conexus Harmattan (Rust Multiplatform)  
**Data:** 22 de outubro de 2026  
**Fase:** 0 — Auditoria  
**Base:** SDD-HARMATTAN.md + Auditoria (`docs/audit.md`) + Repositórios Referência (`docs/repository-matrix.md`)

---

## 1. Visão Arquitetural Atualizada

### 1.1 Princípios Fundamentais (Confirmados)

| Princípio | Descrição | Status |
|-----------|-----------|--------|
| **Core Rust Compartilhado** | Lógica de domínio, serviços, contratos em Rust | ✅ Parcialmente implementado (SDK + Core) |
| **Adaptadores de Plataforma** | Linux + Android via traits abstratos | ❌ Não implementado |
| **UI Rust-Native** | Framework gráfico Rust (Slint/Iced/egui) | ❌ Não decidido (ADR-008) |
| **Launcher Primeiro** | Aplicação executável antes de sistema próprio | ✅ Confirmado (ADR-005) |
| **Kernel Linux Mantido** | Sem reescrita de kernel | ✅ Confirmado (ADR-001) |
| **Arquitetura Modular** | Crates separados, processos só quando justificado | ✅ Parcial (workspace Cargo) |

---

## 2. Crate Graph Proposto (Expandido)

```
conexus-mobile-os/ (workspace root)
│
├── conexus-error/              # 🆕 Unified error types
│   ├── ConexusError, Result<T>
│   └── PlatformError, ConfigError, etc.
│
├── conexus-config/             # 🆕 Persistent config + migrations
│   ├── ConfigStore trait
│   ├── FileConfigStore (Linux/Android)
│   └── Migration framework
│
├── conexus-events/             # 🆕 Event bus + internal communication
│   ├── Event trait + typed events
│   ├── EventBus (async channels)
│   └── Subscription management
│
├── conexus-diagnostics/        # 🆕 Logs, metrics, diagnostics
│   ├── Logger, Metrics, DiagnosticsCollector
│   ├── PlatformDiagnostics trait
│   └── Structured logging (tracing)
│
├── conexus-platform/           # 🆕 Platform abstraction traits
│   ├── AppDiscovery
│   ├── NotificationBackend
│   ├── PermissionManager
│   ├── ConfigStore
│   └── PlatformDiagnostics
│
├── conexus-platform-linux/     # 🆕 Linux implementations
│   ├── desktop-entry, xdg, gio
│   ├── notify-rust + zbus (DBus)
│   ├── polkit/portals (permissions)
│   └── sysinfo, /proc, /sys
│
├── conexus-platform-android/   # 🆕 Android implementations (JNI)
│   ├── PackageManager → AppDiscovery
│   ├── NotificationManager → NotificationBackend
│   ├── Context/SharedPreferences → ConfigStore
│   └── BatteryManager, ConnectivityManager → Diagnostics
│
├── conexus-sdk/                # ✅ EXISTENTE — schemas validation
│   ├── app, tile, notification
│   └── ValidationError
│
├── conexus-core/               # ✅ EXISTENTE — services logic
│   ├── tiles (engine, flip, badge, groups)
│   ├── notifications (broker, grouping, actions)
│   ├── apps (catalog, launch, lifecycle)
│   └── drivers (manager, ADR-009/014/016)
│
├── conexus-profiles/           # 🆕 Profile management
│   ├── ProfileManager
│   ├── Rules engine (location, time, wifi, battery)
│   └── Transitions + persistence
│
├── conexus-apps/               # 🆕 App catalog + launch (wraps platform)
│   ├── AppCatalog (metadata, search)
│   ├── LaunchRequest → Platform.AppDiscovery
│   └── Recents/Favorites tracking
│
├── conexus-permissions/        # 🆕 Policy engine
│   ├── PermissionPolicy
│   ├── Validation + caching
│   └── Platform.PermissionManager wrapper
│
├── conexus-ui/                 # 🆕 UI framework (framework TBD)
│   ├── Components (Button, Card, Grid, List, Navigation, etc)
│   ├── NavigationController trait
│   ├── ThemeProvider trait
│   ├── State management
│   └── Animation system
│
├── conexus-launcher/           # 🆕 Entry point (binary)
│   ├── Wiring: Core + UI + Platform
│   ├── Platform detection
│   └── Main loop
│
└── conexus-daemon/             # 🆕 System service (Linux, opt-in)
    ├── systemd integration
    ├── D-Bus service exposure
    └── Background tasks (tile updates, sync)
```

---

## 3. Contratos Internos (Platform Traits)

### 3.1 `conexus-platform` — Traits Principais

```rust
// App Discovery & Launch
#[async_trait]
pub trait AppDiscovery: Send + Sync {
    async fn list_apps(&self) -> Result<Vec<AppMetadata>>;
    async fn get_app(&self, id: &str) -> Result<Option<AppMetadata>>;
    async fn launch_app(&self, id: &str, args: LaunchArgs) -> Result<()>;
    fn capabilities(&self) -> AppDiscoveryCapabilities;
}

// Notifications
#[async_trait]
pub trait NotificationBackend: Send + Sync {
    async fn show(&self, notification: Notification) -> Result<()>;
    async fn dismiss(&self, id: &str) -> Result<()>;
    async fn list_active(&self) -> Result<Vec<Notification>>;
    fn capabilities(&self) -> NotificationCapabilities;
}

// Permissions
#[async_trait]
pub trait PermissionManager: Send + Sync {
    async fn check(&self, permission: Permission) -> Result<PermissionStatus>;
    async fn request(&self, permission: Permission) -> Result<PermissionStatus>;
    fn supported_permissions(&self) -> Vec<Permission>;
}

// Config Store
#[async_trait]
pub trait ConfigStore: Send + Sync {
    async fn get(&self, key: &str) -> Result<Option<String>>;
    async fn set(&self, key: &str, value: &str) -> Result<()>;
    async fn delete(&self, key: &str) -> Result<()>;
    async fn list_keys(&self, prefix: &str) -> Result<Vec<String>>;
}

// Platform Diagnostics
#[async_trait]
pub trait PlatformDiagnostics: Send + Sync {
    async fn system_info(&self) -> Result<SystemInfo>;
    async fn resource_usage(&self) -> Result<ResourceUsage>;
    fn capabilities(&self) -> DiagnosticsCapabilities;
}
```

### 3.2 Capabilities Structs (Feature Detection)

```rust
pub struct AppDiscoveryCapabilities {
    pub supports_launch_intent: bool,
    pub supports_app_shortcuts: bool,
    pub supports_app_categories: bool,
    pub max_apps_listed: Option<usize>,
}

pub struct NotificationCapabilities {
    pub supports_actions: bool,
    pub supports_inline_reply: bool,
    pub supports_images: bool,
    pub supports_progress: bool,
    pub supports_channels: bool,
    pub max_action_buttons: usize,
}

pub struct PermissionCapabilities {
    pub supports_runtime_request: bool,
    pub supports_special_permissions: bool,
    pub supported_permissions: Vec<Permission>,
}
```

---

## 4. Platform Implementations — Detalhamento

### 4.1 Linux (`conexus-platform-linux`)

| Trait | Implementação | Dependências |
|-------|---------------|--------------|
| `AppDiscovery` | Parse `.desktop` files (`/usr/share/applications`, `~/.local/share/applications`), `gio app-info`, `xdg-desktop-portal` | `desktop-entry`, `xdg`, `zbus` |
| `NotificationBackend` | Freedesktop Notifications via DBus (`libnotify` compat), `zbus` | `zbus`, `notify-rust` |
| `PermissionManager` | Polkit (system), Flatpak permissions, XDG Desktop Portals | `zbus`, `polkit` |
| `ConfigStore` | XDG Config (`~/.config/conexus/`), opcional `gsettings` | `xdg`, `dirs` |
| `PlatformDiagnostics` | `/proc`, `/sys`, `systemd` (zbus), `upower`, `lm-sensors` | `sysinfo`, `zbus` |

**Async Runtime:** `tokio` (multi-thread)

**DBus Services expostos (opcional, via `conexus-daemon`):**
- `com.conexus.TileEngine`
- `com.conexus.NotificationBroker`
- `com.conexus.AppManager`

---

### 4.2 Android (`conexus-platform-android`)

**Arquitetura JNI:**

```
conexus-platform-android/
├── src/
│   ├── lib.rs                    # Rust entry, JNI bindings
│   ├── app_discovery.rs          # Impl AppDiscovery via JNI
│   ├── notifications.rs          # Impl NotificationBackend via JNI
│   ├── permissions.rs            # Impl PermissionManager via JNI
│   ├── config_store.rs           # Impl ConfigStore via SharedPreferences
│   └── diagnostics.rs            # Impl PlatformDiagnostics via JNI
├── android/
│   └── src/main/java/com/conexus/platform/
│       ├── PlatformBridge.kt           # Kotlin side - Android SDK calls
│       ├── AppDiscoveryBridge.kt
│       ├── NotificationBridge.kt
│       ├── PermissionBridge.kt
│       ├── ConfigBridge.kt
│       └── DiagnosticsBridge.kt
├── build.rs                      # Gera JNI bindings (jni-rs)
└── Cargo.toml                    # deps: jni, ndk-glue, ndk-sys
```

| Trait | Implementação Kotlin | Android APIs |
|-------|---------------------|--------------|
| `AppDiscovery` | `PackageManager.queryIntentActivities`, `getLaunchIntentForPackage` | API 21+ |
| `NotificationBackend` | `NotificationManager`, `NotificationChannel`, `NotificationCompat` | API 26+ (channels) |
| `PermissionManager` | `ContextCompat.checkSelfPermission`, `ActivityCompat.requestPermissions` | API 23+ |
| `ConfigStore` | `SharedPreferences` / `DataStore` (Preferences) | API 1+ / 26+ |
| `PlatformDiagnostics` | `BatteryManager`, `ConnectivityManager`, `StorageManager`, `DisplayMetrics` | API 21+ |

**Build:** `cargo-ndk` ou `ndk-build` + Gradle integration

**Capabilities Matrix (Android):**
- Min SDK: 24 (Android 7.0)
- Target SDK: 34 (Android 14)
- Launcher registration: `CATEGORY_HOME` + `CATEGORY_DEFAULT`
- Notification channels: obrigatório API 26+
- Runtime permissions: API 23+

---

## 5. UI Architecture (Pendente ADR-008)

### 5.1 Framework Candidates

| Framework | Linux Support | Android Support | Licença | Maturidade UI | Rust-Native |
|-----------|---------------|-----------------|---------|---------------|-------------|
| **Slint** | ✅ Native (OpenGL/Vulkan/Software) | ✅ NDK (OpenGL ES) | MIT/Commercial | Alta | ✅ |
| **Iced** | ✅ Native (wgpu) | ⚠️ Experimental (wgpu-android) | MIT | Média | ✅ |
| **egui** | ✅ Native (wgpu/glow) | ⚠️ Via winit/ndk-glue | MIT | Média | ✅ |

### 5.2 Critérios ADR-008 (Decisão por Evidência)

**PoC Obrigatória — Mesma Tela nos Dois Alvos:**

```rust
// Tela: Home Grid com 3 tiles (S, M, W) + navegação + tema dark/light
struct HomeScreen {
    tiles: Vec<Tile>,
    selected: Option<usize>,
    theme: Theme,
}

// Requisitos PoC:
// 1. Compila: cargo build --target x86_64-unknown-linux-gnu
// 2. Compila: cargo build --target aarch64-linux-android
// 3. Executa Linux: window abre, render 60fps
// 4. Executa Android: APK instala, launcher aparece
// 5. Tema: dark/light toggle funcional
// 5. Navegação: teclado (Tab) + touch + mouse
// 6. Gestos: swipe horizontal (pivots), edge swipe (Linux: libinput)
```

### 5.3 Component Library (Base)

| Componente | Variants | States | A11y |
|------------|----------|--------|------|
| `Button` | filled, tonal, outlined, text | enabled, hovered, focused, pressed, disabled | `role=button` |
| `Card` | elevated, filled, outlined | default, hover, focus, selected | `role=article` |
| `Tile` | S/M/W/L, flip, badge, folder | default, focus, drag, long-press | `role=button` |
| `NavigationRail` | standard, modal | item: default, selected, focused | ARIA nav |
| `Grid` | responsive, drag-drop | reorder, folder create | drag a11y |
| `SearchBar` | persistent, modal | empty, typing, results, voice | combobox |
| `BottomSheet` | standard, modal | peek, half, full | focus trap |

---

## 6. Data Flow — Launcher Startup

```
┌─────────────────────────────────────────────────────────────────┐
│                     CONEXUS LAUNCHER STARTUP                    │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  1. Platform Detection                                          │
│     ├── Linux: detecta Wayland/X11, DBus session               │
│     └── Android: JNI_OnLoad, Context obtained                  │
│                                                                 │
│  2. Platform Adapter Instantiation                              │
│     ├── Linux:  AppDiscoveryLinux, NotificationLinux, etc     │
│     └── Android: AppDiscoveryAndroid (JNI), etc               │
│                                                                 │
│  3. Core Services Initialization                                │
│     ├── ConfigStore → carrega preferências                     │
│     ├── EventBus → inicia                                      │
│     ├── Diagnostics → inicia coleta                            │
│     ├── TileEngine → carrega tiles salvos                     │
│     ├── NotificationBroker → registra backend                 │
│     ├── AppManager → descobre apps (Platform.AppDiscovery)   │
│     └── ProfileManager → avalia perfil ativo                  │
│                                                                 │
│  4. UI Initialization                                           │
│     ├── ThemeProvider → carrega tema (dark/light/dynamic)     │
│     ├── NavigationController → estado inicial (Home)          │
│     ├── ComponentRegistry → registra componentes              │
│     └── Render loop inicia (Slint/Iced/egui)                  │
│                                                                 │
│  5. Platform Integration                                        │
│     ├── Linux: registra DBus services (se daemon)             │
│     ├── Android: registra como HOME launcher                  │
│     └── Permissions: solicita permissões necessárias          │
│                                                                 │
│  6. Ready — Home Screen Rendered                                │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

---

## 7. IPC / Communication Strategy

### 7.1 Linux — D-Bus (System/Session)

| Serviço | Bus | Interface | Métodos |
|---------|-----|-----------|---------|
| `TileEngine` | Session | `com.conexus.TileEngine` | `UpdateTile`, `RegisterProvider`, `GetTiles` |
| `NotificationBroker` | Session | `com.conexus.Notifications` | `Post`, `Dismiss`, `List`, `Action` |
| `AppManager` | Session | `com.conexus.AppManager` | `Launch`, `GetCatalog`, `SetFavorite` |
| `ProfileManager` | Session | `com.conexus.Profiles` | `GetActive`, `SetProfile`, `ListRules` |

**Crate:** `zbus` (async, Rust-native)

### 7.2 Android — In-Process + System APIs

| Componente | Mecanismo | Detalhes |
|------------|-----------|----------|
| Core ↔ UI | In-process (Rust) | Canais `tokio::sync::mpsc`, `watch` |
| Launcher ↔ Apps | Android Intents | `startActivity`, `PackageManager` |
| Notifications | `NotificationManager` | System service |
| Tile Updates | `AppWidgetManager` (se widgets) | Periodic/RemoteViews |
| Config | `DataStore` / `SharedPreferences` | Async/Sync |

**Nota:** No Android, o launcher **não** roda como system service. IPC é via Android framework.

---

## 8. Security Boundaries

```
┌─────────────────────────────────────────────────────────────────┐
│                      SECURITY BOUNDARIES                        │
├─────────────────────────────────────────────────────────────────┤
│                                                                 │
│  ┌──────────────────┐    ┌──────────────────┐                  │
│  │   CORE (Rust)    │    │   PLATFORM       │                  │
│  │  - Business logic│    │   ADAPTERS       │                  │
│  │  - No unsafe     │    │  - FFI/JNI/DBus  │                  │
│  │  - Pure functions│    │  - Isolated      │                  │
│  │  - Testable      │    │  - Audited       │                  │
│  └────────┬─────────┘    └────────┬─────────┘                  │
│           │                       │                              │
│           │   TRAITS (contratos)  │                              │
│           │   Platform.AppDiscovery│                              │
│           │   Platform.Notifications│                             │
│           │   Platform.Permissions │                              │
│           │   Platform.ConfigStore │                              │
│           ▼                       ▼                              │
│  ┌─────────────────────────────────────────┐                    │
│  │           UI LAYER (Slint/Iced/egui)    │                    │
│  │  - Consome Core via traits               │                    │
│  │  - Não acessa Platform diretamente       │                    │
│  └─────────────────────────────────────────┘                    │
│                                                                 │
└─────────────────────────────────────────────────────────────────┘
```

**Regras:**
- Core **nunca** usa `unsafe` (exceto FFI isolado em `platform/*`)
- Platform adapters **isolados** em crates separadas
- UI **não** importa `platform/*` — apenas traits de `conexus-platform`
- Config sensível: **nunca** em arquivos — `ConfigStore` trait abstraction

---

## 9. Build & CI Strategy

### 9.1 Cargo Workspace Targets

```toml
# Cargo.toml (workspace)
[workspace]
members = [
    "conexus-error",
    "conexus-config",
    "conexus-events",
    "conexus-diagnostics",
    "conexus-platform",
    "conexus-platform-linux",
    "conexus-platform-android",
    "conexus-sdk",
    "conexus-core",
    "conexus-profiles",
    "conexus-apps",
    "conexus-permissions",
    "conexus-ui",
    "conexus-launcher",
    "conexus-daemon",
]

[profile.release]
opt-level = 3
lto = true
codegen-units = 1
panic = "abort"
strip = true

[target.aarch64-linux-android]
linker = "aarch64-linux-android-clang"

[target.x86_64-unknown-linux-gnu]
linker = "clang"
```

### 9.2 CI Pipeline (GitHub Actions)

```yaml
jobs:
  check:
    runs-on: ubuntu-latest
    steps:
      - cargo check --workspace --locked
      - cargo fmt --check --workspace
      - cargo clippy --workspace -- -D warnings
  
  test-linux:
    runs-on: ubuntu-latest
    steps:
      - cargo test --workspace --locked --target x86_64-unknown-linux-gnu
      - cargo test --workspace --locked --target aarch64-unknown-linux-gnu
  
  test-android:
    runs-on: ubuntu-latest
    steps:
      - cargo check --workspace --locked --target aarch64-linux-android
      - ./gradlew :conexus-platform-android:assembleDebug
      - cargo test --workspace --locked --target aarch64-linux-android
  
  security:
    runs-on: ubuntu-latest
    steps:
      - cargo audit
      - cargo deny check
      - cargo geiger --package conexus-core --package conexus-ui
  
  docs:
    runs-on: ubuntu-latest
    steps:
      - cargo doc --workspace --no-deps
```

---

## 10. ADRs Formalizados (Base)

### ADR-001: Kernel Linux Mantido ✅
**Decisão:** Manter kernel Linux existente, core em userspace Rust.
**Status:** Aceito.

### ADR-002: Core Compartilhado em Rust ✅
**Decisão:** Lógica de domínio e serviços em Rust.
**Status:** Aceito.

### ADR-003: Interface em Rust ✅
**Decisão:** Reconstruir UI com framework Rust.
**Status:** Aceito; framework pendente.

### ADR-004: Android + Linux como Alvos ✅
**Decisão:** Núcleo compartilhado + adaptadores específicos.
**Status:** Aceito.

### ADR-005: Launcher Primeiro ✅
**Decisão:** Iniciar com aplicação executável.
**Status:** Aceito.

### ADR-006: Repositórios Históricos como Referência ✅
**Decisão:** Auditar licenças antes de reutilizar.
**Status:** Aceito.

### ADR-007: Modular antes de Microserviços ✅
**Decisão:** Aplicação modular, processos só com justificativa.
**Status:** Aceito.

### ADR-008: UI Framework por Evidência ⏳
**Decisão:** Selecionar após PoC Linux+Android.
**Status:** **Pendente** — Blocker Fase 1.

---

## 11. Decisões Pendentes (Próximos ADRs)

| ADR | Título | Descrição | Prioridade |
|-----|--------|-----------|------------|
| ADR-009 | IPC Mechanism | D-Bus (Linux) vs Channel (Android) vs In-process | Alta |
| ADR-010 | Dynamic Color | Material You (Android) + Static Fallback (Linux) | Média |
| ADR-011 | Process Separation | Quando separar TileEngine/Notifications em processos | Média |
| ADR-012 | Testing Matrix | Cross-platform test strategy (unit/integration/device) | Alta |
| ADR-013 | Launcher Registration | Android HOME intent + Linux .desktop/DBus activation | Alta |
| ADR-014 | Config Encryption | Encrypt sensitive config at rest (SQLCipher/age) | Baixa |

---

## 12. Métricas de Sucesso (por Fase)

| Fase | Métrica | Target |
|------|---------|--------|
| 1 (PoC) | Framework compila+executa Linux+Android | ✅/❌ |
| 2 (Workspace) | `cargo check/test/clippy/fmt/audit/deny` pass | 100% |
| 3 (Core) | Core tests passam sem device | >90% coverage |
| 4 (Launcher) | Home grid + nav + search + theme toggle | Funcional nos 2 targets |
| 5 (Services) | Profiles + Apps + Permissions + matrix | Documentada |
| 6 (Migração) | Harmattan feature matrix complete | 100% mapeado |
| 7 (Distribuição) | APK + Linux package instaláveis | Reproduzível |

---

**Assinatura:** H1 (Architect)  
**Data:** 2026-10-22  
**Próxima revisão:** Após ADR-008 resolvido (Fase 1 completa)