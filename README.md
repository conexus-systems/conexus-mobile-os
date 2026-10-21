# Conexus Mobile OS

> **Um sistema operacional móvel independente, inspirado no Windows 10 Mobile, reimaginado para 2026.**

[![License: Apache-2.0](https://img.shields.io/badge/License-Apache%202.0-blue.svg)](LICENSE)
[![Status: M0 Planning](https://img.shields.io/badge/Status-M0%20Planning-orange.svg)](ROADMAP.md)
[![Base: LineageOS 23.2](https://img.shields.io/badge/Base-LineageOS%2023.2-green.svg)](https://wiki.lineageos.org/)
[![Launcher: Conexus Launcher](https://img.shields.io/badge/Launcher-Conexus%20Launcher-purple.svg)](#)

---

## Visão Geral

O **Conexus Mobile OS** nasce da ideia de preservar o que tornava o Windows 10 Mobile especial — Live Tiles, navegação fluida, Action Center, teclado excepcional, integração entre apps — mas construído sobre uma arquitetura moderna, aberta e sustentável.

### O que NÃO é
- ❌ Um launcher / skin para Android
- ❌ Um fork do Windows 10 Mobile (código proprietário Microsoft)
- ❌ Um projeto para rodar Windows no celular

### O que É
- ✅ Um **sistema operacional completo** (kernel, drivers, services, shell, apps)
- ✅ **Engenharia reversa lícita** — observamos comportamento, reimplementamos do zero
- ✅ **Base: LineageOS 23.2 (Android 15 QPR)** — upstream público, builds reproduzíveis
- ✅ **Launcher padrão: Conexus Launcher** — Live Tiles, pivôs, CommandBar, Metro UI
- ✅ **Open Source (Apache-2.0)** — releases públicos, OTA próprio, site público
- ✅ **Dois targets, um ecossistema:**
  - **Android (LineageOS 23.2)** — Produto principal, uso real, compatibilidade Android nativa
  - **Lumia 950/950 XL** — Laboratório de pesquisa, validação de baixo nível
- ✅ **SDK próprio desde o MVP** — Kotlin (Android) + TypeScript/React Native (cross-platform)
- ✅ **Metro/W10M Design System** — Selawik, Accent configurável, raio 0, Mica/Acrylic

---

## Status Atual

**Fase:** M0 — Viabilidade e Inventário (Semanas 1-2)

### Próximos Gates
| Gate | Target | Critério Principal |
|------|--------|-------------------|
| **M0** | 2026-11-01 | 2x Pixel 8a comprados; stock restore + bootloader unlock + LineageOS recovery testados; ADRs assinados; legal review OK; **GitHub org + repos + Pages site criados** |
| **M1** | 2026-11-15 | SDD v1.0 assinado; Design tokens Metro/W10M; Shell prototype; SDK contracts |
| **M2** | 2026-12-13 | LineageOS 23.2 + ConexusLauncher build reproduzível; Boot + Recovery em hardware (10/10) |
| **M3** | 2027-01-10 | Tile Engine + Notifications + SDK + Reference app no device |
| **M4** | 2027-02-07 | **Alpha público** — GitHub Releases + OTA metadata + site + chaves produção |
| **M5** | 2027-03-07 | Relatório Lumia: Go/No-go para imagem dedicada |

---

## Documentação Principal

| Documento | Descrição |
|-----------|-----------|
| [SDD.md](SDD.md) | Software Design Document — arquitetura completa, requisitos, base técnica LineageOS |
| [CORE.md](CORE.md) | Princípios, ADRs baseline, riscos, Definition of Done |
| [ROADMAP.md](ROADMAP.md) | Milestones M0-M5, cronograma, recursos, métricas |
| [AGENTS.md](AGENTS.md) | 8 subagentes (A0-A7), escopos, critérios, dependências |
| [SECURITY.md](SECURITY.md) | Threat model, 7 camadas, supply chain, incident response |
| [DEVICE_SUPPORT.md](docs/architecture/DEVICE_SUPPORT.md) | Matriz de seleção (Pixel 8a primário, Nothing Phone 2 backup) |
| [ADRs](docs/architecture/ADR/) | 8 ADRs baseline (LineageOS base, Launcher default, Open Source, etc.) |

### Pesquisa (Engenharia Reversa Lícita)
| Documento | Descrição |
|-----------|-----------|
| [REFERENCE_INVENTORY.md](docs/re/REFERENCE_INVENTORY.md) | Inventário Win10M 15254.603 (20 apps, 21 accents, assets) |
| [BEHAVIOR_MATRIX.md](docs/re/BEHAVIOR_MATRIX.md) | Matriz comportamental com critérios de equivalência |
| [SCREEN_CATALOG.md](docs/re/SCREEN_CATALOG.md) | Catálogo visual + Metro token mapping |
| [INTERACTION_FLOWS.md](docs/re/INTERACTION_FLOWS.md) | 11 fluxos (Start, Nav, ActionCenter, Settings, Keyboard, Lock) |
| [COMPATIBILITY_MATRIX.md](docs/re/COMPATIBILITY_MATRIX.md) | Viabilidade por feature (MVP/Alpha/Beta/Drop) |
| [LEGAL_BOUNDARIES.md](docs/re/LEGAL_BOUNDARIES.md) | Clean room methodology, o que é/não é permitido |
| [THIRD_PARTY_NOTICES.md](docs/legal/THIRD_PARTY_NOTICES.md) | 60+ dependências com licenças |

---

## Arquitetura em Resumo

```
┌─────────────────────────────────────────────────────────────┐
│                    CONEXUS EXPERIENCE                         │
│  Conexus Launcher (Live Tiles, Pivôs, CommandBar, Metro)     │
│  Start · Busca · Notificações · Multitarefa · Metro UI      │
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

---

## Subagentes (Equipe de IA)

| Agente | Papel | Status |
|--------|-------|--------|
| **A0** | Tech Lead / Orquestrador | Planejando M0 |
| **A1** | Research & Reverse Engineering | Aguardando M0 |
| **A2** | Platform / Boot / Drivers | Aguardando device purchase |
| **A3** | Shell / Metro UI | Aguardando behavior matrix |
| **A4** | Framework / SDK / Apps | Aguardando SDK contracts |
| **A5** | Segurança / Supply Chain | Planejando threat model |
| **A6** | QA / Performance | Aguardando test infrastructure |
| **A7** | Release / Documentation | Preparando site + release templates |

---

## Como Contribuir

> **Este projeto está em fase de planejamento (M0).** Contribuições de código serão bem-vindas após M1 Gate.

1. Leia [CORE.md](CORE.md) para entender princípios e Definition of Done
2. Revise [ADRs](docs/architecture/ADR/) para decisões arquiteturais
3. Verifique [AGENTS.md](AGENTS.md) para entender estrutura de trabalho
4. Issues e discussões: [GitHub Issues](https://github.com/conexus-mobile-os/conexus-mobile-os/issues)

### Princípios de Contribuição
- **Clean room:** Separação entre research (observação) e implementation (código original)
- **Security first:** Nenhum segredo no repo; chaves de produção só com Tiago
- **Hardware safety:** Zero flash destrutivo sem aprovação humana por escrito
- **Apache-2.0:** Licença do projeto; dependências compatíveis apenas

---

## Hardware Alvo (M0 Gate — **DECIDIDO**)

### Android (Produto) — **Pixel 8a (akita) Selecionado como Primário**

| Device | Papel | Justificativa |
|--------|-------|---------------|
| **Google Pixel 8a (akita)** | **Primário** | Unlock trivial (`fastboot flashing unlock`), kernel 6.1 GKI mainline, LineageOS 23.2 oficial (weekly builds), A/B + dynamic partitions, device tree/vendor upstream LineageOS, hardware novo (mai/2024) |
| **Nothing Phone (2) (pong)** | **Backup** | Unlock trivial, kernel source liberado pelo OEM (3 dias pós-launch), A/B + dynamic, preço ~$400, mas device tree/vendor community-only |
| OnePlus 12 (waffle) | Terceira opção | Hardware top, mas $700+, LineageOS unofficial, carrier variant risks |
| Xiaomi 14 (houji) | ❌ Rejeitado | Unlock wait 7-168h + limite 1 device/ano, LineageOS closed/beta |

**Critérios atendidos pelo Pixel 8a:** ✅ Bootloader unlock oficial ✅ Kernel source mainline 6.1 ✅ Recovery oficial LineageOS ✅ A/B + Dynamic partitions ✅ Community forte (LineageOS upstream) ✅ Device tree/vendor mantidos ✅ Hardware 2024

### Lumia (Pesquisa — Não-Produto)
- **Modelos de pesquisa:**
  - **Lumia 950 / 950 XL** (RM-1104 / RM-1105 / RM-1116) — preferencial 950 XL por tela maior; boot e drivers
  - **Lumia 730 / 735** (RM-914 / RM-915) — pesquisa documental; sem bootloader desbloqueável nem UEFI, não roda o Conexus Mobile OS ([assessment](docs/lumia-730/TECHNICAL_ASSESSMENT.md))
- **Referências (950):** [LumiaWOA](https://github.com/WOA-Project/LumiaWOA), [Lumia950XLPkg](https://github.com/WOA-Project/Lumia950XLPkg)
- **Expectativa:** Boot Linux ARM64, drivers parciais, **sem telefonia/emergência confiável**
- **Status:** Trilha paralela M5 — Go/No-go após M4 Alpha

---

## Release & Distribuição Open Source

### Organização GitHub: `conexus-mobile-os`
- `conexus-mobile-os` — monorepo principal
- `platforms-android-manifest` — manifest repo LineageOS + Conexus customizations
- `conexus-launcher` — launcher standalone
- `conexus-sdk` — Kotlin + TS/RN SDK packages

### Site Público: `conexus-mobile-os.github.io`
- **Landing:** Features, downloads, device matrix
- **Downloads:** Releases estáveis + canais (nightly/weekly/monthly/stable)
- **OTA Metadata:** `https://conexus-mobile-os.github.io/ota/ota_metadata.json`
- **Docs:** SDD, API reference, build guide, porting guide
- **Blog:** Release notes, changelogs, technical deep-dives

### Release Cadence
| Canal | Frequência | Versioning | Rollout |
|-------|------------|------------|---------|
| **Nightly** | Diária | `YYYY.MM.DD-nightly` | Devs only |
| **Weekly** | Semanal | `YYYY.WW-weekly` | Testers |
| **Monthly** | Mensal | `vX.Y.0-monthly` | 5% → 100% |
| **Stable** | Trimestral | `vX.Y.Z` | 1% → 100% |

### Artifacts por Release
| Artifact | Descrição |
|----------|-----------|
| `boot.img` / `dtbo.img` / `vbmeta.img` / `vendor_boot.img` / `super.img` / `recovery.img` | Partition images |
| `lineage_akita-target_files-<version>.zip` | Target files para OTA |
| `conexus-akita-<version>.zip` | Full OTA package (sideload) |
| `SHA256SUMS` + `.asc` | Hashes + assinaturas minisign/cosign |
| `SBOM.spdx.json` | Software Bill of Materials |
| `ota_metadata.json` | Metadata para cliente `conexus-update` |

---

## Conexus Launcher — Default Experience

| Feature | Spec |
|---------|------|
| **Start Screen** | Grid 4/6 col, Live Tiles (S/M/W/L), flip 3D 600ms, grupos/pastas drag-drop |
| **App Drawer** | Pivô `todos · favoritos · recentes`, busca, jump list A-Z |
| **Action Center** | Swipe top-edge, Quick Actions 4-col (CommandBar), notificações agrupadas |
| **Navigation** | Gestos borda (left=back, right=action center), task switcher card stack |
| **Settings** | Metro UI: header grande, pivô categorias, busca, Personalização (tema/accent/animações) |
| **Lock Screen** | Relógio tipográfico, notificações privacidade, camera quick-launch, PIN/biometric |
| **Keyboard** | Shape Writing, cursor magnifier, multi-idioma, acessibilidade |

### Metro/W10M Design System (Selawik + Accent)
| Token | Escuro | Claro |
|-------|--------|-------|
| `bg` | `#000000` | `#FFFFFF` |
| `surface` | `#1F1F1F` | `#F2F2F2` |
| `accent` | Configurável (21 cores + custom) | Configurável |
| `font` | **Selawik** (OFL) | **Selawik** |
| `radius` | 0 (botão, campo, tile) | 0 |
| `motion` | 120ms/250ms/350ms | 120ms/250ms/350ms |

---

## Licença

**Apache License 2.0** — veja [LICENSE](LICENSE)

Componentes de terceiros: veja [THIRD_PARTY_NOTICES.md](docs/legal/THIRD_PARTY_NOTICES.md)

---

## Aviso Legal

> **Este projeto realiza engenharia reversa lícita do Windows 10 Mobile** (build 15254.603) **apenas para documentação comportamental**. Não copiamos código, binários, assets, chaves ou contornamos proteções da Microsoft. Todos os componentes são reimplementados independentemente. Marcas registradas (Windows, Microsoft, Lumia, Live Tiles) pertencem aos seus respectivos donos.

---

## Contato & Links

- **Organização GitHub:** https://github.com/conexus-mobile-os
- **Site Público:** https://conexus-mobile-os.github.io
- **Downloads/Releases:** https://github.com/conexus-mobile-os/conexus-mobile-os/releases
- **Documentação:** https://conexus-mobile-os.github.io/docs
- **OTA Metadata:** https://conexus-mobile-os.github.io/ota/ota_metadata.json
- **Legal/Security:** security@conexus.systems
- **Matrix:** https://matrix.to/#/#conexus-mobile-os:matrix.org
- **Discussions:** https://github.com/conexus-mobile-os/conexus-mobile-os/discussions

---

*Última atualização: 21 de outubro de 2026 — SDD v0.2 (LineageOS 23.2 base, Conexus Launcher default, Open Source)*# conexus-mobile-os
