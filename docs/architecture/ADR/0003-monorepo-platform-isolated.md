# ADR-003: Monorepo com Builds de Plataforma Isolados — **APROVADO**

**Status:** Aceito
**Data:** 2026-10-19
**Decisores:** Tech Lead (A0), Architect, Platform (A2), Framework (A4)
**Assinatura:** A0 ✅

---

## Contexto

O Conexus Mobile OS tem dois targets principais com stacks tecnológicos radicalmente diferentes:
- **Android (AOSP):** Kotlin/Java, Soong/Blueprint, ART, HAL, vendor blobs
- **Lumia (Linux ARM64):** C/Rust, mainline kernel, systemd, D-Bus, meson/cmake

Mais: shared contracts (SDK, design system, service APIs), tools, docs, tests.

Opções de estrutura:
1. Monorepo único com tudo
2. Monorepo com subdiretórios isolados por plataforma
3. Repositórios separados por plataforma + shared packages
4. Mono-repo para shared, repos separados para platform builds

---

## Decisão

**Monorepo único (`conexus-mobile-os/`) com separação lógica estrita por plataforma.**

Estrutura:
```
conexus-mobile-os/
├── shell/                 # Shared UI components (Kotlin + TS)
├── framework/             # Shared SDK, contracts, widgets
├── services/              # Shared system services (logic only)
├── platforms/
│   ├── android/           # AOSP device trees, vendor, BoardConfig, kernel
│   └── linux-lumia/       # UEFI, kernel config, device trees, meson
├── apps/                  # Reference apps (cross-platform via SDK)
├── tests/                 # Shared test infrastructure
├── tools/                 # Build, validation, release tools
└── docs/                  # All documentation
```

**Regra de ouro:** Código de plataforma NÃO mistura com shared. `platforms/android/` contém apenas o necessário para build AOSP (device tree, vendor, kernel config). `platforms/linux-lumia/` idem. Shared code vive em `shell/`, `framework/`, `services/`.

AOSP build exige source tree específica. Solução: `platforms/android/` é um **manifest repo** (`.repo/manifest.xml`) que aponta para AOSP upstream + local device trees. Build roda fora do monorepo ou em worktree dedicada.

---

## Consequências

### Positivas
- Single source of truth para contratos, design system, docs
- Atomic commits cross-platform (ex: SDK change + Android impl + Lumia impl)
- CI/CD unificado, versionamento único
- Fácil code review, shared tooling, shared dependencies

### Negativas
- AOSP source tree (~100GB) não vive no repo — apenas manifest + local customizations
- Git history pode ficar ruidoso com platform-specific changes
- Clone size grande se incluir vendor blobs (usar Git LFS ou external storage)

### Riscos
- **Build complexity:** AOSP build != standard build → Mitigação: Documentar claramente em `tools/build/`, CI isolates
- **Dependency confusion:** Shared deps vs platform deps → Mitigação: `package.json`/`build.gradle` separados por platform

---

## Alternativas Consideradas

| Alternativa | Prós | Contras | Por que não |
|-------------|------|---------|-------------|
| Repos separados (android, lumia, shared) | Isolamento total, clones menores | Sync de contratos manual, versionamento fragmentado, CI complexo | Overhead de coordenação alto |
| Monorepo + AOSP source vendored | Tudo junto | 100GB+ no repo, impossível de manter | Inviável |
| Submodules para AOSP | Versionamento explícito | Submodules são dolorosos, AOSP não usa git submodules nativamente | Experiência ruim |

---

## Plano de Implementação

1. **M0:** Criar estrutura de diretórios, `.gitignore` para AOSP out/, vendor blobs (LFS)
2. **M0:** Setup manifest repo em `platforms/android/manifest/`
3. **M1:** Shared packages: design tokens (npm), SDK contracts (Maven + npm)
4. **M2:** CI pipeline que faz: shared tests → android build → lumia build (se M5)
5. **Contínuo:** Enforce via CODEOWNERS: platform dirs owned by A2, shared by A0/A3/A4

---

## Referências

- [SDD v0.1 §11](./../../SDD.md)
- [CONEXUS monorepo patterns](../DEVICE_SUPPORT.md)