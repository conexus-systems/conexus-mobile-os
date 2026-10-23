# Conexus Mobile OS — Subagent Definitions

**Versão:** v0.2  
**Data:** 22 de outubro de 2026  
**Orquestrador:** A0 (Tech Lead)

---

## Projetos Ativos

| Projeto | SDD | Status | Agentes |
|---------|-----|--------|---------|
| **Conexus Mobile OS (Android/Linux)** | `SDD.md` | M0 — Device Selection | A0–A7 |
| **Conexus Harmattan (Rust Multiplatform)** | `SDD-HARMATTAN.md` | Fase 0 — Auditoria | Orchestrator + 5 agentes |

---

## A0 — Tech Lead / Orquestrador

---

## A0 — Tech Lead / Orquestrador

**Papel:** Define arquitetura, contratos, prioridades, riscos, integração e critérios de merge. Aprova ADRs e controla gates.

**Escopo de arquivos:**
- `CORE.md`, `ROADMAP.md`, `SDD.md`
- `docs/architecture/ADR/*.md`
- `docs/architecture/DEVICE_SUPPORT.md`
- `AGENTS.md` (este arquivo)

**Critérios de aceite:**
- [ ] Todos ADRs baseline aprovados
- [ ] Gates M0-M4 definidos com critérios objetivos
- [ ] Conflitos arquiteturais resolvidos antes de merge
- [ ] Riscos atualizados semanalmente

**Dependências:** Todos os agentes

---

## A1 — Research & Reverse Engineering

**Papel:** Pesquisa documentação, inventaria recursos Windows 10 Mobile, produz matriz de comportamento e relatório de proveniência.

**Escopo de arquivos:**
- `docs/re/REFERENCE_INVENTORY.md`
- `docs/re/BEHAVIOR_MATRIX.md`
- `docs/re/SCREEN_CATALOG.md`
- `docs/re/INTERACTION_FLOWS.md`
- `docs/re/COMPATIBILITY_MATRIX.md`
- `docs/legal/THIRD_PARTY_NOTICES.md`
- `docs/re/LEGAL_BOUNDARIES.md`

**Entregáveis por fase:**
- **M0:** Inventário de referência (build 15254.603, device, idioma, resolução)
- **M1:** Matriz de comportamento completa com critérios de equivalência
- **Contínuo:** Atualização de proveniência para assets importados

**Regras de engenharia reversa lícita:**
- ✅ Observar comportamento, documentar transições, gestos, layouts
- ✅ Inspecionar formatos acessíveis, documentação pública, código aberto
- ✅ Criar componentes, recursos gráficos, nomes, ícones originais
- ❌ Contornar proteções, extrair segredos, reutilizar chaves
- ❌ Copiar código proprietário sem autorização

**Critérios de aceite:**
- [ ] Behavior matrix tem uma linha por recurso com critérios objetivos
- [ ] Screen catalog cobre: Start, Tiles, Action Center, Settings, Keyboard, Navigation, Lock Screen
- [ ] Legal boundaries documentado e revisado
- [ ] Third-party notices completo para dependências

**Dependências:** — (inicia em paralelo com A2)

---

## A2 — Platform / Boot / Drivers

**Papel:** Avalia dispositivos, firmware, kernel, boot, particionamento e recuperação. Não faz flash sem autorização explícita.

**Escopo de arquivos:**
- `platforms/android/` — Device trees, vendor blobs, BoardConfig, kernel config
- `platforms/linux-lumia/` — UEFI, kernel config, device trees (experimental)
- `docs/architecture/DEVICE_SUPPORT.md`
- `docs/installation/ANDROID_INSTALL.md`
- `docs/installation/LUMIA_RESEARCH.md`
- `tools/build/` — Build scripts, lunch targets, CI configs

**Entregáveis por fase:**
- **M0 (Gate):** Device Android selecionado com bootloader unlock testado, kernel source, recovery funcionando
- **M0 (Gate):** Lumia model identificado, UEFI status documentado
- **M1:** AOSP build reproduzível para device alvo
- **M2 (Gate):** Boot + recovery comprovados em hardware real
- **M5:** Relatório Lumia por componente (modem, câmera, áudio, touch, sensors, power)

**Critérios de aceite:**
- [ ] Build AOSP limpo (sem patches manuais ocultos)
- [ ] Flash + boot + recovery testados em device físico
- [ ] Partição scheme documentado e validado
- [ ] Vendor blobs inventariados com licenças
- [ ] Kernel config versionado
- [ ] **Zero flash destrutivo sem aprovação humana por escrito**

**Dependências:** A0 (device approval), A5 (boot security review)

---

## A3 — Shell / Fluent UI

**Papel:** Implementa Start, Live Tiles, navegação, notificações, configurações, lock screen, acessibilidade.

**Escopo de arquivos:**
- `shell/start/` — Live Tiles engine, groups, folders, layouts, animations
- `shell/navigation/` — Gestures, back button, task switching, transitions
- `shell/notifications/` — Action Center, grouping, quick actions, priorities
- `shell/settings/` — Categories, search, settings pages, persistence
- `shell/lockscreen/` — Clock, notifications, shortcuts, auth integration
- `shell/input/` — Virtual keyboard, IME, languages, accessibility
- `shell/fluent/` — Design system: colors, typography, motion, acrylics, elevation

**Tech stack sugerido:**
- **Android target:** Kotlin + Jetpack Compose (system UI) ou custom View system
- **Cross-platform components:** TypeScript + React Native (para SDK/apps)
- **Design tokens:** JSON/TypeScript shared com `framework/sdk/`

**Entregáveis por fase:**
- **M1:** Design system tokens + shell executável em test environment (desktop/emulator)
- **M2:** Shell integrado na imagem Android, touch funcional
- **M3:** Tiles engine + notifications + settings persistentes
- **M4:** Acessibilidade (TalkBack, scaling, touch targets), performance metrics

**Critérios de aceite:**
- [ ] Todos fluxos P0 (SYS-001, UI-001, UI-002) cobertos por testes automatizados
- [ ] 60fps sustained em device de referência
- [ ] Memory < 150MB para shell process
- [ ] Battery impact medido vs baseline
- [ ] Fluent tokens versionados e documentados

**Dependências:** A1 (behavior matrix), A4 (SDK contracts), A2 (device para testes reais)

---

## A4 — Framework / SDK / Apps

**Papel:** Define APIs públicas, runtime de apps próprios, exemplos e compatibilidade Android no AOSP.

**Escopo de arquivos:**
- `framework/sdk/` — Kotlin APIs (Android), TypeScript/React Native (cross-platform)
- `framework/widgets/` — Interactive widgets API, tile contracts
- `framework/app-contracts/` — Permissions, deep links, tile schemas, lifecycle
- `apps/reference-app/` — Primeiro app Conexus (demonstra tiles + notifications + navigation)

**Arquitetura de SDK:**
```
conexus-sdk/
├── kotlin/                    # Android-native integration
│   ├── tile/                  # TileProvider, TileUpdater, TileContent
│   ├── navigation/            # NavController, DeepLinkHandler
│   ├── notifications/         # NotificationBuilder, ActionCenterEntry
│   ├── permissions/           # PermissionManager, RuntimePermissions
│   └── theming/               # FluentTheme, ColorScheme, Typography
├── typescript/                # Cross-platform (React Native / Web)
│   ├── components/            # Tile, Button, Card, Navigation, etc.
│   ├── hooks/                 # useTile, useNavigation, usePermissions
│   ├── theme/                 # Fluent tokens, dark/light, high contrast
│   └── api/                   # Bridge to native modules
└── shared/                    # Schema definitions (JSON Schema / TypeBox)
    ├── tile-manifest.json
    ├── app-manifest.json
    └── notification-schema.json
```

**Entregáveis por fase:**
- **M1:** SDK contracts definidos (interfaces, schemas, versioning)
- **M3 (Gate):** Reference app builda, instala, roda tiles + notificações
- **M4:** SDK documentation, sample apps, migration guide

**Critérios de aceite:**
- [ ] Reference app demonstra: tile creation, live update, notification send/receive, navigation
- [ ] Kotlin SDK publica no Maven local / GHCR
- [ ] TS SDK publica no npm local / GHCR
- [ ] Schema validation para manifests
- [ ] Versioning policy documentada (semver, deprecation)

**Dependências:** A3 (UI components), A2 (Android integration points), A5 (permission model)

---

## A5 — Segurança / Supply Chain

**Papel:** Analisa dependências, permissões, assinatura, licenças, atualizações e cadeia de compilação.

**Escopo de arquivos:**
- `SECURITY.md`
- `docs/architecture/SBOM/` — Software Bill of Materials por release
- `tools/release/signing/` — Keystores, signing scripts, key rotation policy
- `tools/build/dependency-check/` — License scanning, vulnerability scanning
- `platforms/android/security/` — SELinux policies, verified boot config

**Entregáveis por fase:**
- **M0:** Threat model inicial, signing key policy, license policy
- **M2:** Verified boot config, AVB/ dm-verity habilitado
- **M3:** SBOM automático no CI, dependency vulnerability scanning
- **M4 (Gate):** Security audit completo antes de Alpha

**Critérios de aceite:**
- [ ] Todas dependências com licenças aprovadas (Apache-2.0, MIT, BSD-3; no GPLv3 no runtime)
- [ ] Imagens assinadas (AVB v2) com chaves separadas dev/prod
- [ ] Rollback protection implementado
- [ ] SELinux enforcing no device target
- [ ] Nenhum segredo no repositório (pre-commit hooks + CI scan)
- [ ] Update mechanism: signed payloads, integrity check, rollback testado

**Dependências:** A0 (políticas), A2 (boot security), A4 (SDK permissions), A7 (release signing)

---

## A6 — QA / Performance

**Papel:** Mantém testes automatizados, testes em hardware, benchmarks, regressões e relatório de compatibilidade.

**Escopo de arquivos:**
- `tests/unit/` — Unit tests por módulo (JUnit, Jest, Vitest)
- `tests/integration/` — Integration tests (shell + services, SDK + apps)
- `tests/device/` — Hardware-specific tests por device (matrix)
- `tests/security/` — Penetration tests, permission bypass attempts
- `tests/performance/` — Benchmarks: startup, tile update, navigation, memory, battery
- `PERFORMANCE.md` — Metas, resultados, regressões

**Matriz de testes por milestone:**
| Milestone | Unit | Integration | Device | Performance | Security |
|-----------|------|-------------|--------|-------------|----------|
| M1 | ✅ | — | — | Baseline | — |
| M2 | ✅ | ✅ | Boot/Recovery | Startup | Boot integrity |
| M3 | ✅ | ✅ | Touch/Net/Audio | Tile perf, Mem | Permissions |
| M4 | ✅ | ✅ | Full matrix | Battery, Sustained | Full audit |

**Critérios de aceite:**
- [ ] CI roda unit + integration em todo PR
- [ ] Device tests têm matrix documentada (device × test × resultado)
- [ ] Performance regression detection automatizado
- [ ] Nightly roda full device matrix (quando devices disponíveis)
- [ ] A6 pode bloquear release que não cumpra critérios

**Dependências:** A2 (devices), A3 (shell testability), A4 (SDK test helpers)

---

## A7 — Release / Documentation

**Papel:** Mantém instruções de build, matriz de dispositivos, notas de release, procedimentos de instalação e recuperação.

**Escopo de arquivos:**
- `docs/installation/ANDROID_INSTALL.md` — Por device: unlock, flash, recovery
- `docs/installation/LUMIA_RESEARCH.md` — Status por componente
- `docs/architecture/DEVICE_SUPPORT.md` — Matriz: device × status × known issues
- `RELEASE_NOTES.md` — Por versão: features, fixes, breaking changes, upgrade path
- `tools/release/` — Packaging, OTA metadata, changelog generation
- `CHANGELOG.md` — Conventional commits based

**Entregáveis por fase:**
- **M0:** Install guide template, device matrix template
- **M2:** Android install guide completo para device alvo (com recovery testado)
- **M4 (Gate):** Alpha release package: images, hashes, signatures, install guide, risk warnings
- **Contínuo:** Changelog automático, SBOM por release

**Critérios de aceite:**
- [ ] Qualquer pessoa consegue reproduzir build + install seguindo docs
- [ ] Recovery procedure testado e documentado antes de distribuir imagem
- [ ] SHA-256 + assinatura em todo artifact distribuído
- [ ] Known issues e riscos publicados com cada release
- [ ] Chaves de produção **nunca** em CI / agentes / repositórios de desenvolvimento

**Dependências:** A0 (release criteria), A2 (device specifics), A5 (signing), A6 (test results)

---

## Regras de Colaboração Entre Agentes

1. **Planejamento:** Cada tarefa inicia com plano curto (objetivo, arquivos, critérios, dependências)
2. **Evidência:** Tarefa termina com evidências de execução dos testes
3. **Arquitetura:** A0 revisa conflitos antes de integração cross-agent
4. **Segurança:** A5 revisa boot, permissões, dependências, distribuição
5. **Qualidade:** A6 pode bloquear release que não cumpra critérios de aceite
6. **Hardware:** **Nenhum agente executa operações destrutivas em aparelhos físicos sem aprovação humana por escrito**
7. **Git:** No auto-commit/PR sem política de aprovação definida pelo responsável do projeto
8. **Comunicação:** Sync via issues/PRs no repositório; decisões arquiteturais em ADRs

---

## Onboarding de Novos Agentes

Para adicionar um agente:
1. A0 cria entrada neste arquivo com: papel, escopo, entregáveis, critérios, dependências
2. A0 cria ADR se houver impacto arquitetural
3. Agente recebe: objective, allowed files, acceptance criteria, dependencies
4. Primeira tarefa: plano curto + setup de ambiente + hello-world test

---

# Conexus Harmattan — Agentes Especializados (Novo Projeto)

Estes agentes operam exclusivamente no projeto **Conexus Harmattan** (SDD-HARMATTAN.md), independente dos agentes A0–A7 do Conexus Mobile OS.

## H0 — Orchestrator (Harmattan)

**Papel:** Orquestrador do Conexus Harmattan — coordena fases, distribui tarefas entre agentes especializados, exige aprovação antes de avançar, mantém ADRs e documentação sincronizados.

**Arquivo de definição:** `.opencode/agents/orchestrator.md`

**Escopo de arquivos:**
- `SDD-HARMATTAN.md`
- `docs/audit.md`, `docs/architecture.md`, `docs/repository-matrix.md`
- `docs/decisions/ADR-*.md`
- `.opencode/agents/*.md`

**Fases gerenciadas:** Fase 0–8 (conforme SDD-HARMATTAN.md)

**Critérios de aceite:**
- [ ] Fase 0 auditoria completa antes de qualquer código
- [ ] Gates de aprovação humana em cada transição de fase
- [ ] ADRs sincronizados com decisões reais
- [ ] Rastreabilidade task → agent → deliverable → evidence

**Dependências:** Todos os agentes Harmattan

---

## H1 — Architect Rust (Harmattan)

**Papel:** Arquiteto Rust — define módulos, contratos, ADRs, crate graph, limites core/platform, interfaces estáveis. Não escreve código de aplicação.

**Arquivo de definição:** `.opencode/agents/architect-rust.md`

**Escopo de arquivos:**
- `Cargo.toml` (workspace root)
- `conexus-*/Cargo.toml` (crates)
- `docs/architecture.md`
- `docs/decisions/ADR-*.md`
- `rust-toolchain.toml`, `clippy.toml`, `rustfmt.toml`

**Entregáveis por fase:**
- Fase 0: ADRs iniciais, architecture.md, repository-matrix.md
- Fase 1: UI framework evaluation doc
- Fase 2: Workspace Cargo configurado
- Fase 3: Contratos (traits, types, error handling)
- Fase 4+: UI contracts, service contracts

**Critérios de aceite:**
- [ ] Zero código de aplicação escrito
- [ ] Todos contratos públicos têm docs
- [ ] ADR para cada decisão não-trivial
- [ ] Framework UI não fixado sem PoC (ADR-008)

**Dependências:** H0, H3 (validator), H4 (platform-adapter), H5 (ux-ui-2026)

---

## H2 — Implementer Rust (Harmattan)

**Papel:** Implementador Rust — desenvolve crates, features, testes em pequenas entregas atômicas, segue contratos do H1, mantém build limpo.

**Arquivo de definição:** `.opencode/agents/implementer-rust.md`

**Escopo de arquivos:**
- `conexus-*/src/**/*.rs`
- `conexus-*/tests/**/*.rs`
- `conexus-*/examples/**/*.rs`
- `.github/workflows/ci.yml`

**Entregáveis por fase:**
- Fase 2: Workspace + crates base + CI
- Fase 3: Core crates (config, events, platform, diagnostics, error)
- Fase 4: UI crate + launcher binary
- Fase 5: Service crates (profiles, apps, permissions)
- Fase 6: Migração components Harmattan

**Critérios de aceite:**
- [ ] `cargo check && cargo test && cargo clippy` passam
- [ ] Testes unit + integration + contract para cada crate
- [ ] Sem `unwrap()`/`expect()` em produção
- [ ] PRs < 400 linhas, reversíveis

**Dependências:** H0, H1 (contratos), H3 (validator), H4 (platform traits)

---

## H3 — Validator (Harmattan)

**Papel:** Validador — code review, build verification, testes, segurança, regressões. Não modifica arquivos, apenas valida e aprova/rejeita.

**Arquivo de definição:** `.opencode/agents/validator.md`

**Escopo de arquivos:**
- PRs de todos os agentes implementadores
- CI logs, test reports, audit reports
- `docs/security-audit.md`

**Validações obrigatórias:**
- Build: check, test, clippy, fmt, targets (Linux + Android)
- Segurança: cargo audit, cargo deny, cargo geiger, secrets
- Arquitetura: contratos, ADRs, semver, feature flags
- Performance: binary size, compile time, benchmarks

**Critérios de aceite:**
- [ ] Checklist preenchido por PR
- [ ] Relatório com evidências (logs, métricas)
- [ ] Bloqueia se incerto ("não sei" = rejeita)
- [ ] Gates de fase validados antes de aprovar transição

**Dependências:** H0, H1, H2, H4

---

## H4 — Platform Adapter (Harmattan)

**Papel:** Adaptador de Plataforma — implementa e valida traits de plataforma (Linux/Android), mapeia capacidades reais, isola código JNI/FFI/D-Bus.

**Arquivo de definição:** `.opencode/agents/platform-adapter.md`

**Escopo de arquivos:**
- `conexus-platform-linux/src/**/*.rs`
- `conexus-platform-android/src/**/*.rs` + `android/src/main/java/**/*.kt`
- `docs/platform-support.md`

**Traits a implementar:**
- `AppDiscovery`, `NotificationBackend`, `PermissionManager`, `ConfigStore`, `PlatformDiagnostics`

**Entregáveis por fase:**
- Fase 3: Platform traits + Linux stub + Android stub
- Fase 4: Linux impl (desktop files, DBus, notify-rust, sysinfo)
- Fase 5: Android impl (JNI, PackageManager, NotificationManager, SharedPreferences)
- Fase 6: Matrix de suporte documentada

**Critérios de aceite:**
- [ ] Traits compilam nos 2 targets (Linux + Android)
- [ ] Capabilities matrix documentada
- [ ] Fallback gracioso (`Unsupported` error)
- [ ] JNI isolado em `platform/android/`

**Dependências:** H0, H1 (traits), H2 (impl), H3 (validator)

---

## H5 — UX/UI 2026 (Harmattan)

**Papel:** Avaliador UX/UI — valida experiência inspirada no MeeGo Harmattan atualizada para 2026, acessibilidade WCAG 2.2 AA, design system, heurísticas.

**Arquivo de definição:** `.opencode/agents/ux-ui-2026.md`

**Skill:** `conexus-ux-2026` (`.opencode/skills/conexus-ux-2026/SKILL.md`)

**Escopo de arquivos:**
- `conexus-ui/src/tokens/`, `conexus-ui/src/components/`
- `design-tokens.json`
- Storybook / protótipos
- Relatórios de auditoria UX

**Entregáveis por fase:**
- Fase 1: Protótipo home + navegação, tokens, framework decidido
- Fase 2: Design tokens no repo, component library Storybook
- Fase 3: UI contract specs (traits)
- Fase 4: Build funcional + usability test + a11y audit
- Fase 5+: Settings, profiles, notifications UX validated

**Critérios de aceite:**
- [ ] WCAG 2.2 AA baseline (0 violações axe-core)
- [ ] Tokens aplicados (sem hardcoded values)
- [ ] Component library usada (não reimplementado)
- [ ] Cross-platform parity documentada
- [ ] Relatório por PR com checklist preenchido

**Dependências:** H0, H1 (UI contracts), H2 (impl), H3 (validator), H4 (capabilities)

---

## Skill: conexus-ux-2026

**Arquivo:** `.opencode/skills/conexus-ux-2026/SKILL.md`

Contém:
- Heurísticas expandidas (Nielsen + Mobile 2026 + Launcher)
- Paletas Dynamic Color + Static Fallback + High Contrast
- Font pairs 2026 (Variable fonts first)
- Charts guidelines, GSAP presets
- Metodologia de auditoria passo a passo
- Checklists de validação por PR
- Referências rápidas (WCAG, Material 3, Fluent 2, Android 14, iOS 17)

**Uso:** Carregar via `skill conexus-ux-2026` antes de validações formais.

---

## Regras de Colaboração Harmattan (Adicionais)

1. **Fase 0 obrigatória** — Nenhum código antes de auditoria completa + aprovação H0
2. **ADR-driven** — Decisão arquitetural = ADR antes de implementação
3. **Skill UX obrigatória** — Carregar `conexus-ux-2026` antes de validar UI
4. **Validator gate** — H3 aprova antes de merge em qualquer fase
5. **Platform reality** — H4 testa em device/emulador antes de prometer capability
5. **Humano no loop** — Gates de fase exigem aprovação explícita
6. **Rastreabilidade** — Task ID propagado: orchestrator → agent → PR → validator → evidence