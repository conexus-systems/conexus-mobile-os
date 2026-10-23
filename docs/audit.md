# Auditoria da Base — Conexus Harmattan

**Projeto:** Conexus Harmattan (Rust Multiplatform)  
**Data:** 22 de outubro de 2026  
**Fase:** 0 — Auditoria (conforme SDD-HARMATTAN.md)  
**Branch analisada:** `origin/feat/rust-core` (base Rust existente)  
**Commit:** `0d8977a` (Merge pull request #3 from conexus-systems/feat/rust-core)

---

## 1. Visão Geral do Repositório

### Estrutura Atual (branch `feat/rust-core`)

```
conexus-mobile-os/
├── .github/                    # GitHub Actions workflows
├── docs/                       # Documentação (SDD, ADRs, device specs)
├── driver-core/                # Driver manager Java/Kotlin (Android)
├── framework/
│   └── sdk/
│       ├── rust/               # 🦀 SDK Rust (conexus-sdk)
│       │   ├── Cargo.toml
│       │   ├── src/
│       │   │   ├── lib.rs
│       │   │   ├── app.rs
│       │   │   ├── tile.rs
│       │   │   └── notification.rs
│       │   └── tests/
│       ├── shared/             # JSON Schemas (tile, app, notification)
│       │   ├── tile-manifest.schema.json
│       │   ├── app-manifest.schema.json
│       │   └── notification.schema.json
│       └── fixtures/           # Test fixtures
├── services/
│   └── core/                   # 🦀 Core serviços (conexus-core)
│       ├── Cargo.toml
│       ├── src/
│       │   ├── lib.rs
│       │   ├── tiles.rs
│       │   ├── notifications.rs
│       │   ├── apps.rs
│       │   └── drivers.rs
│       └── tests/
├── shell/
│   └── fluent/                 # Design system Metro/W10M
│       └── tokens/
│           └── metro-w10m.tokens.json
├── scripts/                    # Build/install scripts (Python, Go)
├── site/                       # Documentação site (Jekyll/Hugo)
├── tools/
│   ├── installer/              # 🦀 Instalador Go (conexus-installer)
│   └── lumia-linux-port/       # Helper Lumia postmarketOS
├── Cargo.toml                  # 🦀 Workspace root (framework/sdk/rust, services/core)
├── build.gradle.kts            # Gradle Kotlin Multiplatform (driver-core)
├── settings.gradle.kts         # Gradle subprojects
├── turbo.json                  # Turborepo config
├── package.json                # Node.js workspaces
└── SDD-HARMATTAN.md            # Especificação nova (este projeto)
```

### Resumo de Linguagens e Tech Stack

| Camada | Linguagem | Build System | Status |
|--------|-----------|--------------|--------|
| **Core Rust (SDK + Services)** | Rust 2021 | Cargo Workspace | ✅ Funcional |
| **Design System (Tokens)** | JSON | — | ✅ Definido |
| **Driver Manager (Android)** | Kotlin/Java | Gradle KMP | ✅ Parcial |
| **Instalador** | Go | Go Modules | ✅ Funcional |
| **Site/Docs** | Markdown | Turborepo | ✅ Configurado |
| **UI/Launcher** | — | — | ❌ Não existe |

---

## 2. Análise Detalhada por Componente

### 2.1 Workspace Cargo (`Cargo.toml` raiz)

```toml
[workspace]
resolver = "2"
members = ["framework/sdk/rust", "services/core"]

[workspace.package]
edition = "2021"
license = "Apache-2.0"
repository = "https://github.com/conexus-systems/conexus-mobile-os"
rust-version = "1.75"
```

**Avaliação:** ✅ Bem configurado. Usa resolver v2, edition 2021, license Apache-2.0 consistente. Dois members atuais.

**Gaps para Harmattan:**
- Precisa adicionar: `conexus-ui`, `conexus-launcher`, `conexus-platform-linux`, `conexus-platform-android`, `conexus-config`, `conexus-events`, `conexus-diagnostics`, `conexus-error`, `conexus-profiles`, `conexus-permissions`, `conexus-daemon`

---

### 2.2 SDK Rust (`framework/sdk/rust/`)

**Crate:** `conexus-sdk` v0.1.0

**Módulos públicos:**
- `app` — `AppManifest`, validação
- `tile` — `TileManifest`, `TileSize` (S/M/W/L), `TileContent`, `TileUpdate`
- `notification` — `NotificationManifest`, `NotificationAction`, `NotificationStyle`

**Dependências:** `serde` (derive), `serde_json`

**Schemas de referência:** Em `framework/sdk/shared/` (JSON Schema Draft 2020-12)

**Avaliação:** ✅ Sólido. Tipos bem definidos, validação implementada, schemas versionados.

**Gaps para Harmattan:**
- Faltam traits de plataforma (`AppDiscovery`, `NotificationBackend`, etc.)
- Faltam tipos de erro unificados (`ConexusError`)
- Faltam contratos de configuração, eventos, diagnósticos
- Não há `conexus-platform` crate separada

---

### 2.3 Core Serviços (`services/core/`)

**Crate:** `conexus-core` v0.1.0

**Dependência:** `conexus-sdk` (path local)

**Módulos:**
- `tiles` — Tile engine logic (flip 3D, badge, groups, scheduling)
- `notifications` — Notification broker (grouping, actions, channels)
- `apps` — App manager (catalog, launch, lifecycle, pivots)
- `drivers` — Driver manager (ADR-009/014/016)

**Características:**
- Lógica pura, sem IPC
- Comentário: "A camada Binder/AIDL (Android) e D-Bus (Lumia) fica por cima"

**Avaliação:** ✅ Arquitetura limpa, separação de concerns correta.

**Gaps para Harmattan:**
- Não há `config`, `events`, `diagnostics`, `permissions`, `profiles` crates
- IPC layer não implementado (planejado para camada superior)
- Não há `PlatformAbstraction` traits

---

### 2.4 Design System (`shell/fluent/tokens/metro-w10m.tokens.json`)

**Tokens definidos:**
- **Font:** Selawik (OFL-1.1) — métrica Segoe UI
- **Radius:** 0 (botão, campo, tile) — estilo Metro/W10M
- **Motion:** 120/250/350ms (fast/standard/slow)
- **Accent:** 12 presets + custom, minContrastRatio 4.5
- **Themes:** Dark (`#000000` bg) + Light (`#FFFFFF` bg)
- **Comment:** "PLACEHOLDER: o SDD pede 21 cores, a lista final ainda não foi definida"

**Avaliação:** ✅ Base Metro/W10M presente. Tokens JSON estruturados.

**Gaps para Harmattan:**
- Apenas 12/21 accent presets
- Falta token system completo (spacing, typography scale, shadows, z-index)
- Falta integração com framework UI Rust (Slint/Iced/egui)
- Falta dark/light/high-contrast completo
- Falta dynamic color (Material You) support

---

### 2.5 Driver Manager (`driver-core/`)

**Stack:** Kotlin/Java + Gradle KMP

**Módulos:**
- `core` — HardwareAbstractions, HAL interfaces
- `driver-manager` — DependencyResolver, EventBus, DriverInstaller, RepositoryManager, TransactionManager, DriverVerifier
- `hal` — HalInterfaces
- `kernel` — Kernel version specific (4.19, 5.4, 5.10, 5.15, 6.1, 6.6)
- `vendors` — Vendor HALs (Google, Nothing, OnePlus, Samsung, Xiaomi)

**Avaliação:** ✅ Bem estruturado para Android. Multi-kernel version support.

**Gaps para Harmattan:**
- É Android-specific (Kotlin/JVM)
- Harmattan precisa de Rust core + platform adapters
- Pode servir como referência para `conexus-platform-android` implementation

---

### 2.6 Instalador (`tools/installer/`)

**Linguagem:** Go

**Funcionalidades:**
- Device detection (ADB/fastboot)
- Inventory (C1-C5 ADR-010 criteria)
- Extract vendor blobs (closed list paths)
- Release fetching (GitHub API + fallback mock)
- Flash partitions (boot, dtbo, vbmeta, vendor_boot, super, recovery)
- Dry-run mode
- Safety checks (battery, storage, bootloader status)

**Avaliação:** ✅ Completo, seguro, bem testado.

**Relevância Harmattan:** Pode ser adaptado para instalar launcher Rust no Android.

---

### 2.7 Lumia Linux Port (`tools/lumia-linux-port/`)

**Criado nesta sessão** — helper para postmarketOS no Lumia 730.

**Componentes:**
- `lumia-port.sh` — pmbootstrap wrapper (init/build/flash/status)
- `wpinternals-guide.md` — Windows unlock guide
- `pmbootstrap-config.yaml` — nokia-superman config
- `flash.sh` — manual flash
- `status-check.sh` — hardware verification
- `known-issues.md` — honest capabilities matrix

**Avaliação:** ✅ Documentação honesta do que funciona/não funciona.

---

## 3. Repositórios de Referência Harmattan — Análise Preliminar

| Repo | Descrição | Licença | Status | Relevância |
|------|-----------|---------|--------|------------|
| `harmattan/kernel-plus-harmattan` | Kernel patches N9 | GPL-2.0 | Archive | Kernel ref only |
| `harmattan/profilematic` | Perfis de contexto | BSD-3 | Archive | Behavior ref |
| `ajalkane/toggleavailability` | Quick toggles UI | BSD-3 | Archive | UI ref |
| `rschiang/moedict-meego` | Dictionary app | GPL-3 | Archive | App ref |
| `nemomobile/qt-components` | Qt Components | LGPL-2.1 | Active | **UI Components** |

### nemomobile/qt-components — Análise Detalhada

**Licença:** LGPL-2.1 (permite linking dinâmico, exige source distribution)
**Status:** Ativo (último commit ~2024)
**Componentes:** Buttons, Layouts, Navigation, Text, Media, etc.
**Stack:** Qt 5/6, QML, C++

**Implicações para Harmattan:**
- ✅ Pode ser referenciado como design reference
- ⚠️ **Não pode ser dependência direta** sem cumprir LGPL (source distribution, dynamic linking)
- ✅ Pode inspirar component API design em Rust
- ❌ Qt/QML não é alvo — alvo é Rust-native (Slint/Iced/egui)

---

## 4. Matriz de Licenças e Dependências

### Dependências Rust Atuais (workspace)

| Crate | Versão | Licença | Uso |
|-------|--------|---------|-----|
| `serde` | 1.x | MIT/Apache-2.0 | Serialization |
| `serde_json` | 1.x | MIT/Apache-2.0 | JSON |

**Avaliação:** ✅ Todas licenças permissivas (MIT/Apache-2.0), compatíveis com Apache-2.0 do projeto.

### Dependências Go (Instalador)

| Package | Licença |
|---------|---------|
| stdlib | BSD-3 |
| github.com/google/uuid | BSD-3 |
| golang.org/x/sys | BSD-3 |

**Avaliação:** ✅ Compatíveis.

### Dependências Kotlin/Gradle (driver-core)

| Package | Licença |
|---------|---------|
| kotlinx.coroutines | Apache-2.0 |
| kotlinx.serialization | Apache-2.0 |
| AndroidX | Apache-2.0 |

**Avaliação:** ✅ Compatíveis.

---

## 5. Gaps Críticos para Harmattan (Fase 0 → 1)

### 5.1 Arquitetura — Faltando (Prioridade P0)

| Componente | Descrição | Esforço |
|------------|-----------|---------|
| `conexus-platform` crate | Traits: `AppDiscovery`, `NotificationBackend`, `PermissionManager`, `ConfigStore`, `PlatformDiagnostics` | M |
| `conexus-config` crate | Persistent config + migrations, `ConfigStore` trait impl | M |
| `conexus-events` crate | Event bus, `Event` trait, async channels | M |
| `conexus-diagnostics` crate | Logs, metrics, structured diagnostics | M |
| `conexus-error` crate | Unified error types, `Result<T, ConexusError>` | S |
| Platform adapters | `conexus-platform-linux`, `conexus-platform-android` | L |

### 5.2 UI Framework — Decisão Pendente (ADR-008)

**Candidatos a avaliar (PoC obrigatória):**

| Framework | Linux | Android | Licença | Maturidade |
|-----------|-------|---------|---------|------------|
| **Slint** | ✅ Native | ✅ NDK | MIT/Commercial | High |
| **Iced** | ✅ Native | ⚠️ Experimental | MIT | Medium |
| **egui** | ✅ Native | ⚠️ Via winit | MIT | Medium |

**Critério ADR-008:** "um protótipo da mesma tela deverá compilar e executar nos dois alvos antes da adoção definitiva"

### 5.3 UI Crates — Faltando

| Crate | Responsabilidade |
|-------|------------------|
| `conexus-ui` | Views, Navigation, State, Theming, Components |
| `conexus-launcher` | Entry point, wiring core+UI+platform |

### 5.4 Service Crates — Faltando

| Crate | Responsabilidade |
|-------|------------------|
| `conexus-profiles` | ProfileManager, rules engine, transitions |
| `conexus-apps` | AppCatalog, metadata, launch requests (wraps platform) |
| `conexus-permissions` | Policy engine, validation |
| `conexus-daemon` | System service (Linux only, opt-in) |

---

## 6. Riscos Identificados

| Risco | Probabilidade | Impacto | Mitigação |
|-------|---------------|---------|-----------|
| **UI Framework não compila no Android** | Alta | Bloqueia Fase 1 | PoC cedo (Fase 1), fallback egui |
| **JNI complexity no Android adapter** | Alta | Atraso Fase 4 | Isolar em `platform/android/`, usar `jni-rs` + `ndk-glue` |
| **LGPL qt-components não usável** | Baixa | Design reference only | Usar apenas como reference visual |
| **Dynamic Color (Material You) no Linux** | Média | UX inconsistency | Fallback static tokens, documentar |
| **IPC D-Bus vs Android Intents** | Média | Architecture divergence | Traits abstratos, impls específicas |
| **Kernel drivers não portáveis** | Alta | Hardware features | Documentar em `known-issues.md`, não bloquear launcher |

---

## 7. Decisões Pendentes (ADRs)

| ADR | Título | Status | Decisão Necessária |
|-----|--------|--------|-------------------|
| ADR-008 | UI Framework Selection | **Pendente** | PoC Slint/Iced/egui Linux+Android |
| ADR-009 | IPC Mechanism | Pendente | D-Bus (Linux) vs Channel (Android) |
| ADR-010 | Dynamic Color Strategy | Pendente | Material You Android + static Linux |
| ADR-011 | Launcher vs System Service | Pendente | Quando separar processos |
| ADR-012 | Testing Strategy Cross-Platform | Pendente | Matrix Linux/Android |

---

## 8. Próximos Passos Imediatos (Fase 0 → 1)

### Ações do Orquestrador (H0)

1. **Criar tarefas para H1 (Architect):**
   - Formalizar ADRs 001-008 (baseados em SDD-HARMATTAN.md)
   - Definir `conexus-platform` traits em `docs/architecture.md`
   - Especificar critérios PoC UI Framework (ADR-008)

2. **Criar tarefas para H4 (Platform Adapter):**
   - Investigar `AppDiscovery` no Linux (desktop files, xdg, portal)
   - Investigar `NotificationBackend` Linux (libnotify, zbus)
   - Investigar Android JNI patterns (PackageManager, NotificationManager)

3. **Criar tarefas para H5 (UX/UI):**
   - Carregar skill `conexus-ux-2026`
   - Definir design tokens completos (21 accents, spacing, typography scale)
   - Criar component specs para home grid, navigation, search

4. **Criar tarefas para H2 (Implementer):**
   - Preparar workspace Cargo expandido (Fase 2)
   - Setup CI: `cargo check`, `test`, `clippy`, `fmt`, `audit`, `deny`

### Entregáveis Fase 0 (Checklist)

- [x] Auditoria da base completa (`docs/audit.md` — **este documento**)
- [ ] Matriz de repositórios Harmattan (`docs/repository-matrix.md`)
- [ ] Arquitetura revisada (`docs/architecture.md`)
- [ ] ADRs 001-008 formalizados (`docs/decisions/ADR-*.md`)
- [ ] Aprovação humana para iniciar Fase 1

---

## 9. Conclusão

**Estado atual:** Base Rust sólida e bem arquitetada (SDK + Core Services + Design Tokens), mas **launcher/UI inexistente**. O projeto Harmattan pode aproveitar ~60% da base existente (schemas, core logic, tokens, installer).

**Bloqueador principal:** Seleção do UI Framework (ADR-008) — requer PoC comprovada Linux + Android antes de qualquer implementação de UI.

**Recomendação:** Prosseguir para Fase 1 com foco exclusivo em PoC de UI Framework. Não iniciar implementação de crates de serviço até ADR-008 resolvido.

---

**Assinatura do Auditor:** H0 (Orchestrator)  
**Data:** 2026-10-22  
**Próxima revisão:** Após Fase 1 PoC completa