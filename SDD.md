# Conexus Mobile OS — SDD (Software Design Document)

**Versão:** v0.2  
**Data:** 21 de outubro de 2026  
**Status:** Baseline — LineageOS 23.2 (Android 15) como upstream, Conexus Launcher como default

---

## 1. Resumo Executivo

O **Conexus Mobile OS** é um sistema operacional móvel **open source** (Apache-2.0), baseado no **LineageOS 23.2 (Android 15 QPR)**, que traz a experiência do Windows 10 Mobile (Live Tiles, navegação fluida, Action Center, teclado, integração entre apps) para dispositivos modernos, com o **Conexus Launcher** como launcher padrão de toda a experiência.

### Objetivos Principais
- **Base:** LineageOS 23.2 (Android 15 QPR) — upstream público, builds reproduzíveis, device trees mantidos
- **Launcher padrão:** Conexus Launcher (Live Tiles, pivôs, CommandBar, Metro/W10M design system) — substitui Launcher3/Trebuchet
- **Distribuição:** Open source (Apache-2.0), releases públicos via **GitHub Releases** + **OTA** próprio
- **Site público:** `conexus-mobile-os.github.io` na organização `conexus-mobile-os` — docs, downloads, OTA metadata, device support
- **Device primário:** Google Pixel 8a (`akita`) — Tensor G3, kernel 6.1 GKI mainline, LineageOS 23.2 oficial
- **Device backup:** Nothing Phone (2) (`pong`) — Snapdragon 8+ Gen 1, kernel source OEM
- **Trilha pesquisa:** Lumia 950/950 XL e Lumia 730/735 (experimental, não-produto; o 730 é apenas documental, ver ADR-002)

### Fora de Escopo Inicial
- Kernel próprio do zero
- Compatibilidade universal com todos celulares
- Garantia de telefonia/câmera/emergência em Lumia
- Reimplementar serviços proprietários Microsoft/Google
- Certificação GMS/Play no Alpha (microG como opção futura)

---

## 2. Arquitetura Técnica

### 2.1 Visão Geral em Camadas

```
┌─────────────────────────────────────────────────────────────┐
│                    CONEXUS EXPERIENCE                         │
│  Conexus Launcher (Live Tiles, Pivôs, CommandBar, Metro)     │
│  Start · Busca · Notificações · Multitarefa · Fluent UI      │
├─────────────────────────────────────────────────────────────┤
│                    CONEXUS FRAMEWORK                          │
│  Navigation API · Widgets API · Tema · Permissões            │
│  Deep Links · SDK (Kotlin + TypeScript/React Native)         │
├─────────────────────────────────────────────────────────────┤
│                 CONEXUS SYSTEM SERVICES                       │
│  App Manager · Notification Broker · Tile Scheduler          │
│  Search · Update · Policy · Security · Bridge-Android        │
├──────────────────────┬───────────────────────────────────────┤
│    ANDROID TARGET    │       LUMIA RESEARCH TARGET            │
│  LineageOS 23.2      │  UEFI · Linux ARM64 · Drivers          │
│  (AOSP + Lineage)    │  (experimental, per-model)             │
└──────────────────────┴───────────────────────────────────────┘
```

### 2.2 Componentes Principais

#### Shell (Experiência do Usuário) — **Conexus Launcher como Default**
- **Start:** Live Tiles redimensionáveis, grupos, pastas, layouts, animações (flip 3D 600ms)
- **App Launcher:** Lista alfabética, busca, favoritos, histórico (pivô `todos · favoritos · recentes`)
- **Action Center:** Notificações agrupadas, ações rápidas (CommandBar estilo Metro)
- **Navigation:** Gestos, botão voltar, pivôs, transições (sem bottom navigation Material)
- **Settings:** Categorias, busca, páginas de configurações (Personalização: tema, accent, reduzir animações)
- **Lock Screen:** Relógio, notificações permitidas, atalhos, autenticação
- **Input:** Teclado virtual, IME, idiomas, acessibilidade (Shape Writing, cursor control)
- **Fluent/Metro Visual System:** Tokens Metro/W10M (Selawik, accent configurável, raio 0, Mica/Acrylic)

#### Framework (SDK & Contratos)
- **Kotlin SDK:** Integração nativa Android (TileProvider, NavController, NotificationBuilder, PermissionManager, FluentTheme)
- **TypeScript/React Native SDK:** Cross-platform (Components, Hooks, Theme, Native Bridge JSI)
- **Shared Contracts:** JSON Schema para Tile Manifest, App Manifest, Notification Schema

#### System Services (Processos Isolados — ADR-007)
- **conexus-tile-engine:** Atualizações e renderização de tiles (flip 3D, badge, grupos)
- **conexus-notifications:** Recepção, agrupamento, ações (CommandBar style)
- **conexus-app-manager:** Catálogo, abertura, ciclo de vida (pivô `todos · favoritos · recentes`)
- **conexus-search:** Pesquisa local de apps e conteúdo autorizado
- **conexus-settings:** Preferências e configurações do shell (tema, accent, animações)
- **conexus-update:** Verificação, distribuição, rollback de updates (A/B + fallback)
- **conexus-security:** Políticas, permissões, auditoria (SELinux enforcing)
- **conexus-bridge-android:** Integração com recursos Android (LineageOS APIs)

---

## 3. Requisitos do Produto (Rastreáveis)

| ID | Requisito | Prioridade | Critério de Aceite |
|----|-----------|------------|-------------------|
| SYS-001 | Inicialização | P0 | Inicializar imagem LineageOS + Conexus em Pixel 8a |
| SYS-002 | Recuperação | P0 | Restaurar imagem funcional após falha de update (A/B rollback) |
| SYS-003 | Conexus Launcher Default | P0 | Launcher padrão no boot, substituindo Launcher3/Trebuchet |
| SEC-001 | Integridade | P0 | Validar imagens e pacotes antes da instalação (AVB v2) |
| SEC-002 | Permissões | P0 | Aplicar controles de acesso do sistema-base (SELinux enforcing) |
| UI-001 | Tela Inicial (Live Tiles) | P0 | Exibir tiles configuráveis, flip 3D, grupos, pastas, redimensionamento |
| UI-002 | Navegação (Pivôs/Gestos) | P0 | Pivôs, gestos de borda, voltar, task switcher funcionam |
| UI-003 | Notificações (CommandBar) | P1 | Agrupadas, ações rápidas, inline reply, dismiss |
| UI-004 | Configurações (Metro) | P1 | Tema, accent, animações, pivô de categorias, busca |
| APP-001 | Apps Android (LineageOS) | P0 | Instalar e abrir apps via Play/F-Droid/APK |
| APP-002 | Conexus SDK | P1 | Construir e executar app de referência (Kotlin + TS/RN) |
| TILE-001 | Live Tiles Engine | P1 | Flip 3D, badge, grupos, pastas, redimensionamento, agendamento |
| HW-001 | Touch e Tela | P0 | Operação estável em orientação/resolução suportadas (120Hz) |
| HW-002 | Telefonia | P1 | Chamadas e SMS validados em Pixel 8a (LineageOS RIL) |
| HW-003 | Energia | P1 | Suspensão, retomada, consumo medidos (≤1.5%/h idle, ≤8%/h ativo) |
| UPD-001 | Atualização Segura (A/B) | P1 | OTA assinada, rollback automático, rollback manual via recovery |

**P0** = necessário para primeiro protótipo instalável (M2 Gate)  
**P1** = recursos da primeira versão utilizável Alpha (M4 Gate)

---

## 4. Distribuição Open Source & Release Process

### 4.1 Licenciamento & Organização GitHub
- **Licença:** Apache-2.0 (compatível com LineageOS/AOSP)
- **Organização GitHub:** `conexus-mobile-os` (pública)
- **Repositórios principais:**
  - `conexus-mobile-os` — monorepo principal (source, docs, tools)
  - `platforms-android-manifest` — manifest repo LineageOS + Conexus customizations
  - `conexus-launcher` — launcher standalone (para uso em outros ROMs)
  - `conexus-sdk` — Kotlin + TS/RN SDK packages
  - `conexus-ota-server` — servidor OTA metadata + artifacts (opcional, pode usar GitHub Releases)

### 4.2 Site Público — `conexus-mobile-os.github.io`
**Deploy:** GitHub Pages (branch `gh-pages` ou custom domain)  
**Conteúdo:**
| Página | Descrição |
|--------|-----------|
| `/` | Landing: hero, features (Live Tiles, Metro UI, Open Source), download latest |
| `/devices` | Matriz de suporte (Pixel 8a, Nothing Phone 2, etc.) + status por device |
| `/download` | Latest release + historical, OTA sideload, factory images, checksums |
| `/docs` | Documentação técnica (SDD, API reference, build guide, porting guide) |
| `/ota` | OTA metadata JSON (`ota_metadata.json`) para `conexus-update` client |
| `/blog` | Release notes, changelogs, anúncios de versão |
| `/contribute` | Guia de contribuição, code of conduct, issue templates |

**Stack:** Jekyll/Hugo + GitHub Actions deploy, i18n (pt-BR/en), tema custom Conexus

### 4.3 Release Cadence & Versioning
| Canal | Frequência | Versioning | Descrição |
|-------|------------|------------|-----------|
| **Nightly** | Diária (automática) | `YYYY.MM.DD-nightly` | Build automático CI, testes básicos, para desenvolvedores |
| **Weekly** | Semanal | `YYYY.WW-weekly` | Build testado, changelog gerado, para testers |
| **Monthly** | Mensal | `vX.Y.0-monthly` | Stable candidate, full test suite, OTA rollout 5% → 100% |
| **Stable** | Trimestral | `vX.Y.Z` | Tagged release, assinado produção, suporte 6 meses |

**Versioning:** `v<major>.<minor>.<patch>-<channel>` (ex: `v1.2.0-monthly`, `v1.2.1-stable`)

### 4.4 Release Artifacts (por release)
| Artifact | Descrição | Publicação |
|----------|-----------|------------|
| `boot.img` / `dtbo.img` / `vbmeta.img` / `vendor_boot.img` / `super.img` / `recovery.img` | Partition images | GitHub Release + OTA server |
| `lineage_akita-target_files-<version>.zip` | Target files para OTA | GitHub Release (assets) |
| `conexus-akita-<version>.zip` | Full OTA package (sideload) | GitHub Release + OTA metadata |
| `SHA256SUMS` | Hashes de todos artifacts | GitHub Release |
| `conexus-akita-<version>.asc` / `.sig` | Assinatura minisign/cosign | GitHub Release |
| `SBOM.spdx.json` | Software Bill of Materials | GitHub Release + OTA metadata |
| `RELEASE_NOTES.md` | Changelog, known issues, upgrade path | GitHub Release body + `/blog` |
| `ANDROID_INSTALL.md` | Procedimento completo por device | `/docs` + `/download` |
| `ota_metadata.json` | Metadata para `conexus-update` client | `/ota/ota_metadata.json` |

### 4.5 OTA Update Server (GitHub Releases + Pages)
- **Metadata endpoint:** `https://conexus-mobile-os.github.io/ota/ota_metadata.json`
- **Payload URLs:** GitHub Release asset URLs (CDN global, assinados AVB v2)
- **Channels:** `nightly`, `weekly`, `monthly`, `stable` (client seleciona via Settings)
- **Rollout:** Staged (1% → 5% → 25% → 100%) via `rollout_percentage` no metadata
- **Rollback:** Automático A/B (boot fail) + manual via recovery (Non-A/B)

### 4.6 Assinatura & Supply Chain
| Ambiente | Chave | Armazenamento | Rotação |
|----------|-------|---------------|---------|
| Dev/Nightly | `dev-key.pem` | Repo (age/git-crypt) | — |
| CI/Weekly | `ci-key.pem` | GitHub Environments secrets | 90 dias |
| Monthly/Stable | `release-key.pem` | **Offline HSM / YubiKey (Tiago)** | 180/365 dias |

**Regra:** Chaves produção **nunca** em CI, repo, agents. Assinatura release = processo manual documentado (`scripts/sign_release.sh`).

---

## 5. Base Técnica — LineageOS 23.2 (Android 15)

### 5.1 Target Específico
| Componente | Versão/Configuração |
|------------|---------------------|
| **Upstream** | LineageOS 23.2 (branch `lineage-23.2`) |
| **Android** | 15 QPR (`android-15.0.0_r1`) |
| **Device** | Google Pixel 8a (`akita`) |
| **SoC** | Google Tensor G3 (GS101) |
| **RAM** | 8 GB LPDDR5X |
| **Storage** | 128/256 GB UFS 3.1 |
| **Display** | 6.1" 1080x2400 OLED, 120 Hz, 431 PPI |
| **Kernel** | 6.1 GKI (`android-gs-akita-6.1-android16`) |
| **Particionamento** | A/B + Dynamic Partitions (`super`) |
| **Bootloader** | `fastboot flashing unlock` (oficial, sem wait) |
| **Recovery** | LineageOS Recovery oficial (weekly builds) |
| **Vendor Blobs** | `android_vendor_google_akita` (LineageOS) |
| **Device Tree** | `android_device_google_akita` (LineageOS upstream) |

### 5.2 Manifest Repo Conexus (`platforms-android-manifest/default.xml`)
```xml
<!-- Minimal subset: AOSP + LineageOS + Conexus customizations -->
<manifest>
  <remote name="aosp" fetch="https://android.googlesource.com/" />
  <remote name="lineageos" fetch="https://github.com/LineageOS/" />
  <remote name="google" fetch="https://android.googlesource.com/device/google/" />
  <remote name="conexus" fetch="https://github.com/conexus-mobile-os/" />
  <default revision="lineage-23.2" remote="lineageos" sync-j="4" />
  <!-- AOSP minimal + LineageOS device trees + Conexus packages -->
  <project path="packages/apps/ConexusLauncher" name="conexus-launcher" remote="conexus" revision="main" />
  <project path="packages/services/ConexusServices" name="conexus-services" remote="conexus" revision="main" />
  <project path="frameworks/sdk/conexus-sdk-kotlin" name="conexus-sdk-kotlin" remote="conexus" revision="main" />
  <project path="frameworks/support/conexus-fluent" name="conexus-fluent-compose" remote="conexus" revision="main" />
  <project path="packages/services/ConexusUpdate" name="conexus-update-service" remote="conexus" revision="main" />
  <project path="device/conexus/sepolicy" name="conexus-sepolicy" remote="conexus" revision="main" />
  <remove-project name="platform/packages/apps/Launcher3" />
  <remove-project name="platform/packages/apps/Trebuchet" />
</manifest>
```

### 5.3 Build Targets
| Target | Uso |
|--------|-----|
| `lineage_akita-userdebug` | Desenvolvimento diário (root, debuggable, SELinux permissive opcional) |
| `lineage_akita-user` | Pré-release (sem root, SELinux enforcing) |
| `lineage_akita-eng` | Bring-up inicial (máximo debug) |

### 5.4 Build Process (Resumo)
```bash
# 1. Init
repo init -u https://github.com/conexus-mobile-os/platforms-android-manifest.git -b main --git-lfs
repo sync -j$(nproc) --force-sync --no-clone-bundle --no-tags

# 2. Vendor blobs (factory image Pixel 8a)
./device/google/akita-lineage/extract-files.sh /path/to/akita-factory-image

# 3. Kernel GKI
cd kernel/google/gs/akita && ./build_akita.sh

# 4. Build completo
source build/envsetup.sh
lunch lineage_akita-userdebug
m -j$(nproc) 2>&1 | tee build.log

# 5. Output em out/target/product/akita/
# boot.img, dtbo.img, vbmeta.img, vendor_boot.img, super.img, recovery.img
```

---

## 6. Conexus Launcher — Default Experience

### 6.1 Substituição do Launcher Padrão
- **Removido:** `Launcher3` (AOSP), `Trebuchet` (LineageOS)
- **Adicionado:** `ConexusLauncher` como `default_home` no `product_packages`
- **Configuração:** `ro.conexus.launcher.default=true` + `persist.conexus.launcher.enabled=true`

### 6.2 Componentes do Launcher (`packages/apps/ConexusLauncher`)
| Módulo | Descrição |
|--------|-----------|
| `StartScreen` | Grid 4/6 colunas, Live Tiles (S/M/W/L), grupos, pastas, drag-drop, flip 3D |
| `AppDrawer` | Pivô `todos · favoritos · recentes`, busca integrada, jump list A-Z |
| `ActionCenter` | Swipe top-edge, Quick Actions 4-col (CommandBar), notificações agrupadas |
| `Navigation` | Gestos borda (left=back, right=action center), task switcher card stack |
| `SettingsApp` | Metro UI: header grande, pivô categorias, busca, Personalização (tema/accent/animações) |
| `LockScreen` | Relógio tipográfico, notificações privacidade, camera quick-launch, PIN/biometric |
| `Input/Keyboard` | Shape Writing, cursor magnifier, multi-idioma, acessibilidade |

### 6.3 Design System — Metro/W10M (Selawik + Accent)
| Token | Escuro | Claro |
|-------|--------|-------|
| `bg` | `#000000` | `#FFFFFF` |
| `surface` | `#1F1F1F` | `#F2F2F2` |
| `text` | `#FFFFFF` | `#000000` |
| `accent` | Configurável (21 cores + custom) | Configurável |
| `font` | **Selawik** (OFL, métrica Segoe UI) | **Selawik** |
| `radius` | 0 (botão, campo, tile) | 0 |
| `motion` | 120ms/250ms/350ms | 120ms/250ms/350ms |

**Accent:** 21 cores padrão + custom hex (AA contraste auto-ajustado). Persistido em Settings.

---

## 7. System Services — Arquitetura Isolada (ADR-007)

### 7.1 IPC Abstraction Layer
| Target | IPC | Registration |
|--------|-----|--------------|
| **Android** | Binder (AIDL) | `ServiceManager.addService()` / `ConexusServiceManager` |
| **Linux (Lumia)** | D-Bus (systemd) | systemd units + D-Bus policy |
| **Test** | In-process | Direct calls |

### 7.2 Serviços — AIDL Interfaces (`framework/sdk/kotlin/aidl/`)
| Service | Interface | Responsabilidade |
|---------|-----------|------------------|
| **Tile Engine** | `ITileEngine`, `ITileProvider` | Flip 3D, badge, grupos, agendamento, push/periodic |
| **Nav Controller** | `INavController` | Pivôs, gestos, deep links, task switcher, edge swipes |
| **Notification Broker** | `INotificationBroker` | Post/update/cancel, actions (inline reply), channels |
| **Permission Manager** | `IPermissionManager` | Runtime perms + Conexus perms (`tileUpdate`, `deepLinkHandle`) |
| **App Manager** | `IAppManager` | Catalog, launch, lifecycle, pivô `todos·favoritos·recentes` |
| **Search** | `ISearchService` | Local indexing, deep link results, app content |
| **Update** | `IUpdateService` | OTA A/B, rollback, staged rollout, metadata |
| **Security** | `ISecurityService` | Audit, attestation, verified boot status |
| **Bridge Android** | `IBridgeAndroid` | LineageOS APIs, hardware abstraction |

---

## 8. Framework — SDK Contracts

### 8.1 Shared Contracts (`framework/shared/`)
| Schema | Descrição |
|--------|-----------|
| `tile-manifest.json` | Tile: `tileId`, `size` (S/M/W/L), `content` (template+data), `activation`, `update`, `badges` |
| `app-manifest.json` | App: `appId` (reverse domain), `version` (SemVer), `entryPoint` (kotlin/ts/web/native), `permissions[]`, `deepLinks[]`, `tiles[]` |
| `notification.json` | Notification: `actions[]` (button/input/inlineReply/deepLink), `style` (messaging/media/progress), `badge` |

### 8.2 Kotlin SDK (`framework/sdk/kotlin/`)
- `tile/`, `navigation/`, `notifications/`, `permissions/`, `theming/`, `aidl/`

### 8.3 TypeScript/React Native SDK (`framework/sdk/typescript/src/`)
- `types/`, `theme/`, `components/`, `hooks/`, `api/` (JSI bridge)

---

## 9. Segurança

| Camada | Implementação |
|--------|---------------|
| **Boot** | AVB v2, rollback protection (anti-rollback vbmeta), bootloader re-lock opcional |
| **Kernel** | Hardened: `KPTR_RESTRICT=2`, `MODULES_DISABLE=1` (prod), KASLR, MTE (HW), GKI 6.1 |
| **SELinux** | Enforcing, policy custom `device/conexus/sepolicy`, `neverallow` rules |
| **Process Isolation** | Services = processos separados (Binder/D-Bus), least privilege, namespace sandbox |
| **App Sandbox** | UID per app, SELinux `untrusted_app`, scoped storage, scoped permissions |
| **FBE** | File-Based Encryption per-user, hardware-backed keys |
| **Network** | TLS 1.3 only, cert pinning (API BFF, OTA server), Private DNS DoT/DoH |
| **Updates** | AVB v2 signed, integrity check, anti-rollback, staged rollout |

**Chaves produção:** `prod-key.pem` — **apenas Tiago, offline, air-gapped**. Assinatura release = manual.

---

## 10. Estratégia de Testes

| Milestone | Unit | Integration | Device | Performance | Security |
|-----------|------|-------------|--------|-------------|----------|
| M1 | ✅ | — | — | Baseline | — |
| M2 | ✅ | ✅ | Boot/Recovery | Startup | Boot integrity |
| M3 | ✅ | ✅ | Touch/Net/Audio | Tile perf, Mem | Permissions |
| M4 | ✅ | ✅ | Full matrix | Battery, Sustained | Full audit |

**Critérios P0:** 10/10 boots, 5/5 recovery, tile update <100ms, nav 60fps, shell <150MB RSS

---

## 11. Device Support Matrix

| Device | Codename | SoC | Bootloader | Kernel | LineageOS 23.2 | A/B | Status |
|--------|----------|-----|------------|--------|----------------|-----|--------|
| **Pixel 8a** | `akita` | Tensor G3 | ✅ Oficial | 6.1 GKI | ✅ Oficial | ✅ | **Primário** |
| **Nothing Phone 2** | `pong` | SD 8+ Gen1 | ✅ Oficial | 5.15/6.1 | ⚠️ Unofficial | ✅ | **Backup** |
| OnePlus 12 | `waffle` | SD 8 Gen3 | ✅ (exceto T-Mobile) | 5.15/6.1 | ⚠️ Unofficial | ✅ | 3ª opção |
| Xiaomi 14 | `houji` | SD 8 Gen3 | ❌ Wait 7-168h + limite | Stock | ❌ Closed | ✅ | Rejeitado |

### Lumia Research (Não-Produto)
| Modelo | UEFI | Linux Boot | Touch | Display | Modem | Camera | Audio | Sensors |
|--------|------|------------|-------|---------|-------|--------|-------|---------|
| Lumia 950 XL | ✅ Working | ✅ Partial | ✅ | ✅ | ❌ | ❌ | ⚠️ | ✅ |

---

## 11. Milestones & Gates

### M0 — Viabilidade (Semanas 1-2) — **ATUAL**
- [x] ADRs 001-008 aprovados (LineageOS base, Conexus Launcher default, Open Source)
- [x] Device selection: Pixel 8a primário, Nothing Phone 2 backup
- [ ] **Comprar 2x Pixel 8a** (~$1.050)
- [ ] Factory image test → stock restore → bootloader unlock → LineageOS recovery
- [ ] Legal review `LEGAL_BOUNDARIES.md` (clean room)
- [ ] ADRs assinados
- [ ] **Criar org GitHub `conexus-mobile-os` + repos + Pages site**

### M1 — Spec + Visual Prototype (Semanas 3-4)
- [ ] SDD v1.0 assinado
- [ ] Design tokens Metro/W10M (`shell/fluent/tokens/`)
- [ ] Shell prototype (desktop/emulator): Start, Tiles, Nav, Action Center, Settings
- [ ] SDK contracts (`framework/sdk/contracts/`)
- [ ] Behavior matrix completa (P0/P1)

### M2 — Primeira Imagem LineageOS + Conexus (Semanas 5-8)
- [ ] Build reproduzível (`lineage_akita-userdebug` + ConexusLauncher default)
- [ ] Shell integrado no system image (`product` partition)
- [ ] Verified Boot (AVB/dm-verity) + SELinux enforcing
- [ ] Boot + Recovery em hardware real (10/10 boots)
- [ ] `ANDROID_INSTALL.md` completo

### M3 — Framework + Apps (Semanas 9-12)
- [ ] Tile Engine + Notifications + App Manager + Search
- [ ] SDK Kotlin + TS publicados (Maven Central + npm/GHCR)
- [ ] Reference app: tile live update + notification + nav
- [ ] Schema validation para manifests

### M4 — Alpha Público (Semanas 13-16)
- [ ] Extended tests (7 dias): zero critical crashes, battery ≤ metas
- [ ] OTA update: signed, integrity, rollback testado (5 ciclos A/B)
- [ ] Security audit: zero critical vulns, SBOM, license compliance
- [ ] **Alpha release no GitHub Releases + OTA metadata + site update**
- [ ] Chaves produção usadas (isoladas de dev)

### M5 — Lumia Research (Semana 17+)
- [ ] Linux ARM64 boot validation
- [ ] Driver matrix per component
- [ ] `LUMIA_RESEARCH.md` — procedimento, limitações, riscos
- [ ] Feasibility report: Go/No-go

---

## 12. Próximas Ações Imediatas (M0 — Esta Semana)

| Ação | Responsável | Prazo | Status |
|------|-------------|-------|--------|
| Aprovar ADRs 001-008 (LineageOS base, Launcher default, Open Source) | Tech Lead | 2026-10-22 | 🔄 **Pendente** |
| **Criar org GitHub `conexus-mobile-os` + repos + Pages** | Tech Lead | 2026-10-22 | 🔄 **Pendente** |
| Comprar 2x Pixel 8a (128GB/256GB) | Tech Lead | 2026-10-23 | 🔄 **Pendente** |
| Registrar IMEI/serial numbers | Tech Lead | 2026-10-23 | ⬜ |
| Agendar revisão legal `LEGAL_BOUNDARIES.md` | Tech Lead | 2026-10-24 | 🔄 **Pendente** |
| Baixar Factory Image Pixel 8a + validar SHA-256 | A2 (Platform) | 2026-10-23 | ⬜ |
| Stock restore test (flash factory → boot) | A2 | 2026-10-24 | ⬜ |
| Bootloader unlock test | A2 | 2026-10-24 | ⬜ |
| LineageOS recovery flash + boot test | A2 | 2026-10-25 | ⬜ |
| Documentar `ANDROID_INSTALL.md` | A2 | 2026-10-26 | ⬜ |
| Iniciar `REFERENCE_INVENTORY.md` + `SCREEN_CATALOG.md` (Win10M 15254.603) | A1 (Research) | 2026-10-25 | ⬜ |
| Threat model baseline (STRIDE) + signing key policy | A5 (Security) | 2026-10-25 | ⬜ |
| **Setup GitHub Pages site (`conexus-mobile-os.github.io`)** | A0/A7 | 2026-10-25 | ⬜ |

---

## 13. Referências Completas

- **LineageOS:** [akita wiki](https://wiki.lineageos.org/devices/akita), [device tree](https://github.com/LineageOS/android_device_google_akita), [vendor](https://github.com/LineageOS/android_vendor_google_akita), [building](https://wiki.lineageos.org/devices/akita/build)
- **AOSP:** [Building for Pixel](https://source.android.com/docs/setup/build/building-pixel-kernels), [AVB](https://source.android.com/security/verifiedboot), [update_engine](https://source.android.com/docs/core/ota/update_engine)
- **Metro/W10M UI:** [Skill metro-w10m-ui](.agents/skills/metro-w10m-ui/), [Fluent 2 tokens](https://github.com/microsoft/fluentui/tree/master/packages/design-tokens), [Fluent System Icons](https://github.com/microsoft/fluentui-system-icons)
- **LumiaWOA:** [Project](https://github.com/WOA-Project/LumiaWOA), [UEFI](https://github.com/WOA-Project/Lumia950XLPkg)
- **CONEXUS Patterns:** `platform-infra`, `conexus-bff-stack`, `app-fleet` skills