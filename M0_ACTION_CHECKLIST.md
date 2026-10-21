# M0 Action Checklist — Semana 1 (2026-10-19 a 2026-10-26)

**Objetivo:** Completar Gate M0 — Device Android aprovado com boot/recovery comprovado

---

## ✅ Já Concluído (Planejamento)

- [x] SDD v0.1 baseline criado
- [x] CORE.md, AGENTS.md, ROADMAP.md, SECURITY.md criados
- [x] 8 ADRs baseline propostos (001-008)
- [x] DEVICE_SUPPORT.md preenchido com 4 candidatos + decisão
- [x] Estrutura de diretórios `docs/`, `shell/`, `framework/`, `services/`, `platforms/`, `apps/`, `tests/`, `tools/`
- [x] Research templates: REFERENCE_INVENTORY, BEHAVIOR_MATRIX, SCREEN_CATALOG, INTERACTION_FLOWS, COMPATIBILITY_MATRIX, LEGAL_BOUNDARIES
- [x] Repositório GitHub linkado: https://github.com/conexus-systems/conexus-mobile-os

---

## 🎯 Ações Esta Semana (Prioridade Máxima)

### A0 (Tech Lead) — Decisões & Compras
| Ação | Responsável | Prazo | Status |
|------|-------------|-------|--------|
| Aprovar/rejeitar ADRs 001-008 | A0 | 2026-10-22 | ⬜ |
| **Comprar 2x Google Pixel 8a** (128GB ou 256GB) | A0 | 2026-10-23 | ⬜ |
| Registrar IMEI/serial numbers dos devices | A0 | 2026-10-23 | ⬜ |
| Agendar revisão legal `LEGAL_BOUNDARIES.md` | A0 | 2026-10-24 | ⬜ |
| Assinar ADRs aprovados | A0 | 2026-10-25 | ⬜ |

### A2 (Platform) — Hardware Validation
| Ação | Responsável | Prazo | Status |
|------|-------------|-------|--------|
| Baixar Factory Image Pixel 8a (`akita`) | A2 | 2026-10-23 | ⬜ |
| Validar SHA-256 do factory image | A2 | 2026-10-23 | ⬜ |
| **Flash Factory Image → Boot → Confirmar Stock** | A2 | 2026-10-24 | ⬜ |
| **Bootloader Unlock** (`fastboot flashing unlock`) | A2 | 2026-10-24 | ⬜ |
| **LineageOS Recovery Flash + Boot Test** | A2 | 2026-10-25 | ⬜ |
| Documentar procedimento em `ANDROID_INSTALL.md` | A2 | 2026-10-26 | ⬜ |
| Lumia 950 XL UEFI status research | A2 | 2026-10-26 | ⬜ |

### A1 (Research) — Win10M Reference
| Ação | Responsável | Prazo | Status |
|------|-------------|-------|--------|
| Obter firmware Win10M 15254.603 legítimo (Lumia 950 XL) | A1 | 2026-10-24 | ⬜ |
| Iniciar `REFERENCE_INVENTORY.md` (build, device, assets) | A1 | 2026-10-25 | ⬜ |
| Iniciar `SCREEN_CATALOG.md` (capturas Start, Tiles, ActionCenter) | A1 | 2026-10-26 | ⬜ |
| Revisar `LEGAL_BOUNDARIES.md` com advogado | A1 | 2026-10-25 | ⬜ |

### A5 (Security) — Threat Model Inicial
| Ação | Responsável | Prazo | Status |
|------|-------------|-------|--------|
| Threat model baseline (STRIDE) | A5 | 2026-10-25 | ⬜ |
| Signing key policy (dev/ci/alpha/prod) | A5 | 2026-10-25 | ⬜ |
| License allowlist/blocklist review | A5 | 2026-10-25 | ⬜ |

---

## 📦 Entregáveis de Saída M0 (Gate)

| Entregável | Localização | Critério |
|------------|-------------|----------|
| **2x Pixel 8a em mãos** | Físico | Serial numbers registrados |
| **Stock restore comprovado** | `docs/installation/ANDROID_INSTALL.md` | Flash factory → boot OK |
| **Bootloader unlock testado** | `docs/installation/ANDROID_INSTALL.md` | Unlock + wipe confirmado |
| **LineageOS recovery funcional** | `docs/installation/ANDROID_INSTALL.md` | Recovery boot + factory reset |
| **ADRs 001-008 assinados** | `docs/architecture/ADR/` | PDF/MD assinados |
| **Legal opinion** | `docs/legal/LEGAL_OPINION_YYYYMMDD.md` | Parecer advogado |
| **Risk register atualizado** | `CORE.md` | R01, R02, R06 mitigados |

---

## 💰 Orçamento Estimado M0

| Item | Qtd | Unit (USD) | Total (USD) |
|------|-----|------------|-------------|
| Google Pixel 8a 128GB | 2 | ~$499 | ~$998 |
| Cabos USB-C, adaptadores | 1 kit | ~$50 | ~$50 |
| **Total M0** | | | **~$1.050** |

---

## 🔴 Blockers / Riscos

| Risco | Probabilidade | Impacto | Mitigação |
|-------|---------------|---------|-----------|
| Pixel 8a indisponível no Brasil | Média | Alto | Importar via Amazon US / eBay / revendedor internacional |
| Advogado IP indisponível esta semana | Alta | Médio | Usar template `LEGAL_BOUNDARIES.md` como base, revisar async |
| Factory image download falha | Baixa | Alto | Mirrors: `flash.android.com`, `developers.google.com/android/images` |
| Bootloader unlock falha (device defect) | Baixa | Crítico | **Por isso 2 units** — testar no backup se primário falhar |

---

## 📋 Próxima Semana (M0 → M1 Transition)

Se Gate M0 passar em 2026-11-01:

1. **A0** — Kickoff M1: SDD v1.0 approval meeting
2. **A3** — Design System Tokens (Fluent 2) → `shell/fluent/tokens/`
3. **A4** — SDK Contracts Definition → `framework/sdk/contracts/`
4. **A1** — Behavior Matrix completa (P0/P1 features)
5. **A2** — AOSP Build Environment Setup (manifest repo, toolchains)
6. **A5** — CI Pipeline Skeleton (license check, vuln scan, SBOM)

---

**Owner:** A0 (Tech Lead)  
**Review:** Daily standup async (update checklist)  
**Gate Review:** 2026-11-01 (ou antes se tudo pronto)