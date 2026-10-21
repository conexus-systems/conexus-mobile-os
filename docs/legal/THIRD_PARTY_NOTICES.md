# Third-Party Notices — Conexus Mobile OS

**Versão:** v0.1
**Data:** 19 de outubro de 2026
**Status:** Template — atualizar a cada release

---

## Visão Geral

Este documento lista todos os componentes de terceiros incorporados, linkados ou distribuídos com o Conexus Mobile OS, com suas respectivas licenças e avisos de copyright.

**Política de Licenças (ADR-001, SECURITY.md):**
- ✅ **Permitidas:** Apache-2.0, MIT, BSD-3-Clause, BSD-2-Clause, ISC, Unicode-3.0, OFL-1.1, CC0-1.0, Zlib, libpng
- ⚠️ **Condicional:** LGPL-2.1 (apenas dynamic linking, sem modificação), MPL-2.0 (file-level copyleft)
- ❌ **Proibidas no runtime:** GPL-3.0, AGPL-3.0, LGPL-3.0, SSPL, Commons Clause, BSL, EULA proprietária

---

## Componentes por Categoria

### 1. Base System (AOSP / Android)

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| Android Open Source Project | 14/15 | Apache-2.0 | `platforms/android/` | Upstream; patches em `platforms/android/patches/` |
| Linux Kernel | 5.10/6.x | GPL-2.0-only | `platforms/android/kernel/` | **Exceção:** Kernel é GPL-2.0, permitido como base de SO |
| ART (Android Runtime) | | Apache-2.0 | AOSP | |
| Bionic (libc) | | BSD-3-Clause | AOSP | |
| Toybox | | BSD-3-Clause | AOSP | |
| Conscrypt | | Apache-2.0 | AOSP | TLS provider |
| OkHttp | | Apache-2.0 | AOSP | HTTP client |
| Protobuf | | BSD-3-Clause | AOSP | |
| FlatBuffers | | Apache-2.0 | AOSP | |
| TinyXML2 | | Zlib | AOSP | |
| libzip | | BSD-3-Clause | AOSP | |
| OpenSSL / BoringSSL | | Apache-2.0 / OpenSSL | AOSP | |
| zlib | | Zlib | AOSP | |
| libpng | | libpng | AOSP | |
| freetype | | FTL (BSD-like) | AOSP | |
| harfbuzz | | MIT | AOSP | |
| Skia | | BSD-3-Clause | AOSP | Graphics |
| Angle | | BSD-3-Clause | AOSP | GL→Vulkan/D3D |
| Vulkan Loader | | Apache-2.0 | AOSP | |
| SwiftShader | | Apache-2.0 | AOSP | Software Vulkan |

### 2. Fluent Design System (Microsoft)

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| Fluent UI React Native | Latest | MIT | `shell/fluent/rn/` | `@fluentui/react-native` |
| Fluent Design Tokens | Latest | MIT | `shell/fluent/tokens/` | `@fluentui/tokens` |
| Fluent System Icons | Latest | MIT | `shell/fluent/icons/` | `@fluentui/system-icons` |
| Fluent UI Web Components | Latest | MIT | `shell/fluent/web/` | `@fluentui/web-components` |

### 3. Kotlin / Android Libraries

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| Kotlin Stdlib | 1.9+ | Apache-2.0 | `shell/`, `framework/sdk/kotlin/` | |
| Kotlinx Coroutines | 1.7+ | Apache-2.0 | `shell/`, `services/` | |
| Kotlinx Serialization | 1.5+ | Apache-2.0 | `framework/sdk/kotlin/` | |
| Jetpack Compose | 1.5+ | Apache-2.0 | `shell/start/`, `shell/fluent/compose/` | `androidx.compose.*` |
| Jetpack Navigation | 2.7+ | Apache-2.0 | `shell/navigation/` | |
| Room | 2.6+ | Apache-2.0 | `services/` | SQLite wrapper |
| DataStore | 1.0+ | Apache-2.0 | `services/` | Preferences |
| WorkManager | 2.9+ | Apache-2.0 | `services/tile-engine/` | Background tasks |
| Hilt / Dagger | 2.48+ | Apache-2.0 | `shell/`, `services/` | DI |
| Coil | 2.5+ | Apache-2.0 | `shell/` | Image loading |
| Accompanist | 0.30+ | Apache-2.0 | `shell/` | Compose extras |

### 4. TypeScript / React Native Libraries

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| React | 18.2+ | MIT | `framework/sdk/typescript/`, `apps/reference-app/` | |
| React Native | 0.74+ | MIT | `framework/sdk/typescript/`, `apps/reference-app/` | |
| TypeScript | 5.3+ | Apache-2.0 | `framework/sdk/typescript/` | |
| React Navigation | 6.1+ | MIT | `framework/sdk/typescript/` | |
| Zustand / Jotai | Latest | MIT | `framework/sdk/typescript/` | State mgmt |
| React Hook Form | Latest | MIT | `framework/sdk/typescript/` | Forms |
| Zod | Latest | MIT | `framework/shared/` | Schema validation |
| TypeBox | Latest | MIT | `framework/shared/` | JSON Schema |
| Axios | Latest | MIT | `framework/sdk/typescript/` | HTTP client |

### 5. System Services / Infrastructure

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| systemd | 255+ | LGPL-2.1 | `platforms/linux-lumia/` | **Dynamic linking only** |
| D-Bus | 1.15+ | GPL-2.0 / AFL-2.1 | `platforms/linux-lumia/` | System bus |
| GLib | 2.78+ | LGPL-2.1 | `platforms/linux-lumia/` | **Dynamic linking only** |
| libinput | 1.24+ | MIT | `platforms/linux-lumia/` | Input handling |
| Wayland | 1.22+ | MIT | `platforms/linux-lumia/` | Display protocol |
| Weston | 12.0+ | MIT | `platforms/linux-lumia/` | Wayland compositor |
| Mesa | 23.3+ | MIT | `platforms/linux-lumia/` | Graphics drivers |
| DRM/KMS | Kernel | GPL-2.0 | Kernel | Display |

### 6. Security / Crypto

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| BoringSSL | Latest | OpenSSL + BSD | `platforms/android/security/` | |
| libsodium | 1.0.18+ | ISC | `services/security/` | Crypto primitives |
| age (encryption) | 1.1+ | BSD-3-Clause | `tools/release/signing/` | File encryption |
| minisign | 0.11+ | ISC | `tools/release/signing/` | Signatures |
| cosign | 2.2+ | Apache-2.0 | `tools/release/signing/` | Container signing |
| syft | Latest | Apache-2.0 | `tools/build/dependency-check/` | SBOM |
| grype | Latest | Apache-2.0 | `tools/build/dependency-check/` | Vuln scanning |
| trivy | Latest | Apache-2.0 | `tools/build/dependency-check/` | Vuln scanning |

### 7. Build / CI / Tools

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| Soong / Blueprint | AOSP | Apache-2.0 | `platforms/android/` | Build system |
| Ninja | 1.11+ | Apache-2.0 | Build | |
| Bazel | 6.4+ | Apache-2.0 | `tools/build/` | Alternative build |
| Git | 2.43+ | GPL-2.0 | VCS | Build tool only |
| Python | 3.11+ | PSF | Scripts | Build tool only |
| Node.js | 20+ | MIT | `framework/sdk/typescript/` | Build tool only |
| pnpm | 8+ | MIT | Package manager | |

### 8. Testing

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| JUnit | 5.10+ | EPL-2.0 | `tests/unit/` | Unit tests |
| MockK | 1.13+ | Apache-2.0 | `tests/unit/` | Kotlin mocking |
| Espresso | 3.5+ | Apache-2.0 | `tests/device/` | UI tests Android |
| UIAutomator | 2.2+ | Apache-2.0 | `tests/device/` | System UI tests |
| Playwright | 1.40+ | Apache-2.0 | `tests/integration/` | E2E web/desktop |
| Detox | 20+ | MIT | `tests/device/` | E2E React Native |
| Vitest | 1.0+ | MIT | `tests/unit/` | TS unit tests |
| k6 | 0.47+ | AGPL-3.0 | `tests/performance/` | **Build tool only** — load testing |

### 9. Fonts

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| Inter | 4.0+ | OFL-1.1 | `shell/fluent/tokens/fonts/` | Primary UI font |
| Noto Sans | Latest | OFL-1.1 | `shell/fluent/tokens/fonts/` | Fallback / CJK |
| Noto Sans Mono | Latest | OFL-1.1 | `shell/fluent/tokens/fonts/` | Monospace |
| Fluent System Icons Font | Latest | MIT | `shell/fluent/icons/` | Icon font |

### 10. Lumia Research (platforms/linux-lumia/)

| Componente | Versão | Licença | Localização | Notas |
|------------|--------|---------|-------------|-------|
| Lumia950XLPkg (UEFI) | Latest | BSD-2-Clause | `platforms/linux-lumia/uefi/` | EDK2-based |
| LumiaWOA Drivers | Latest | GPL-2.0 / MIT | `platforms/linux-lumia/drivers/` | Kernel drivers |
| EDK2 | Latest | BSD-2-Clause | `platforms/linux-lumia/uefi/` | UEFI reference |
| Little Kernel (LK) | Latest | MIT | `platforms/linux-lumia/boot/` | Bootloader |

---

## Formato de Registro (Para Adições Futuras)

Ao adicionar nova dependência, preencher:

```markdown
### [Categoria] — [Nome do Componente]

| Campo | Valor |
|-------|-------|
| **Componente** | Nome exato |
| **Versão** | Versão fixada (sem `^` ou `~`) |
| **Licença** | SPDX identifier |
| **Localização** | Path no repo ou `external` |
| **Tipo** | `runtime` / `build-tool` / `test-only` / `dev-only` |
| **Link** | URL do repo / package |
| **Copyright** | `Copyright (c) [ano] [holder]` |
| **Licença Texto** | Caminho para LICENSE no source ou reprodução aqui |
| **Modificações** | `None` ou descrição |
| **Justificativa** | Por que esta dependência? Alternativas avaliadas? |
| **Aprovado Por** | A5 (Security) + A0 (Tech Lead) |
| **Data** | YYYY-MM-DD |
```

---

## Verificação Automatizada (CI)

```yaml
# .github/workflows/license-check.yml
jobs:
  license-check:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - name: Generate SBOM
        uses: anchore/sbom-action@v0
        with:
          format: spdx-json
          output-file: sbom.spdx.json
      - name: Check Licenses
        run: |
          # Custom script: verify all SPDX licenses in allowlist
          python tools/build/dependency-check/license_check.py sbom.spdx.json
      - name: Vulnerability Scan
        uses: aquasecurity/trivy-action@master
        with:
          scan-type: 'fs'
          format: 'sarif'
          output: 'trivy.sarif'
      - name: Upload SARIF
        uses: github/codeql-action/upload-sarif@v2
        with:
          sarif_file: 'trivy.sarif'
```

---

## Atualização por Release

| Release | Data | SBOM Hash | Vulns (Critical/High) | Novas Dependências | Removidas |
|---------|------|-----------|----------------------|-------------------|-----------|
| v0.1.0-alpha.1 | TBD | TBD | 0/0 | [Lista] | — |

---

## Contato

Dúvidas sobre licenças: legal@conexus.systems
Relatórios de vulnerabilidade: security@conexus.systems