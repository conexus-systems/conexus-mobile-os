# LineageOS 23.2 — verificação de Rust e do launcher padrão

**Data:** 2026-10-22 · **Método:** leitura dos repositórios `LineageOS/*` no ramo `lineage-23.2` e do AOSP no tag base, pela API do GitHub e pelo `android.googlesource.com`. Nenhuma árvore foi sincronizada e nada foi compilado.

## 1. Qual Android é o LineageOS 23.2

O manifesto `LineageOS/android`, ramo `lineage-23.2`, `default.xml`, define o remoto AOSP com `revision="refs/tags/android-16.0.0_r4"`. **LineageOS 23.2 é Android 16**, não Android 15.

**Correção feita:** SDD, README, ADR-001 (LineageOS), `DEVICE_SUPPORT` e o site afirmavam "Android 15 QPR / `android-15.0.0_r1`". Foram corrigidos para Android 16 / `android-16.0.0_r4`. O ADR-001 AOSP (substituído) fica como histórico.

**Observação:** em 2026-10-22 o ramo padrão dos repositórios LineageOS já é `lineage-24.0`. O projeto continua fixado em `23.2` (existe para `android_build_soong`, `android_frameworks_base` e `android_device_google_akita`). Migrar para 24.0 é decisão futura do A0; não foi avaliada.

## 2. Rust na plataforma — confirmado

| Verificação | Resultado |
|---|---|
| `android_build_soong`, ramo `lineage-23.2`, pasta `rust/` | Existe, com `binary.go`, `library.go`, `bindgen.go`, `clippy.go`, `compiler.go`, `coverage.go`, `fuzz.go`, `prebuilt.go` e testes |
| Tipos de módulo registrados em `rust/binary.go` | `rust_binary` e `rust_binary_host` (e os demais em `library.go`) |
| Backend Rust do AIDL, tag `android-16.0.0_r4`, `system/tools/aidl/build/aidl_interface.go` | Existe (`langRust = "rust"`, `ShouldGenerateRustBackend`, alvo `<nome>-rust`) |

**Conclusão:** os serviços e HALs em Rust do ADR-012 e do ADR-016 são suportados pelo build da plataforma na versão fixada. Isto confirma o suporte do build, **não** que um serviço nosso compila nem roda no aparelho: nenhum foi construído ainda.

## 3. Launcher padrão — como o Android decide

- O papel `android.app.role.HOME` (`PermissionController/res/xml/roles.xml`, ramo `lineage-23.2`) **não tem `default-holders` por configuração**.
- Quem escolhe é `HomeRoleBehavior.getFallbackHolderAsUser` (`PermissionController/role-controller/java/com/android/role/controller/behavior/HomeRoleBehavior.java`): consulta as atividades `MAIN` + `HOME`, ignora o launcher de emergência do Settings e escolhe a de **maior `priority`** no intent-filter. **Se duas empatam, não há padrão** (volta `null`) e o usuário é perguntado.
- O LineageOS 23.2 instala o launcher dele por `vendor/lineage/config/common_mobile.mk`: `Launcher3QuickStep` (ou `Launcher3QuickStepGo` em produtos Go) e `Launcher3Overlay`.

**Como tornar o Conexus Launcher o padrão, sem patch no framework:**
1. No makefile de produto do Conexus, incluir o `ConexusLauncher` em `PRODUCT_PACKAGES`.
2. Garantir que ele seja o único HOME com a maior prioridade: ou dar ao intent-filter HOME dele `priority` maior que a do Launcher3, ou tirar o Launcher3 da lista com `PRODUCT_PACKAGES := $(filter-out Launcher3QuickStep,$(PRODUCT_PACKAGES))` depois da herança.
3. **Cuidado:** o `Launcher3QuickStep` também é o provedor de Recentes e da navegação por gestos (Quickstep). Remover o pacote pode quebrar os Recentes e a navegação por gestos. A opção mais segura é **manter o Launcher3QuickStep instalado como provedor de Recentes e só mudar quem é o HOME**. Isso **não foi verificado** e precisa de teste em aparelho.

## 4. O que continua sem confirmação

- Que um serviço Rust do Conexus compile na árvore do `lineage-23.2` (o suporte existe; falta construir um).
- O comportamento de Recentes e gestos com Conexus como HOME e Launcher3QuickStep como provedor.
- O kernel do Pixel 8a (`android-gs-akita-6.1-android16` no `DEVICE_SUPPORT`): não foi consultado agora.
- Qualquer coisa que dependa de aparelho.

## 5. Reproduzir

```bash
gh api "repos/LineageOS/android/contents/default.xml?ref=lineage-23.2" -H "Accept: application/vnd.github.raw" | grep -n 'revision="refs/tags'
gh api "repos/LineageOS/android_build_soong/contents/rust/binary.go?ref=lineage-23.2" -H "Accept: application/vnd.github.raw" | grep RegisterModuleType
curl -s "https://android.googlesource.com/platform/system/tools/aidl/+/refs/tags/android-16.0.0_r4/build/aidl_interface.go?format=TEXT" | base64 -d | grep -in rust
gh api "repos/LineageOS/android_vendor_lineage/contents/config/common_mobile.mk?ref=lineage-23.2" -H "Accept: application/vnd.github.raw" | grep -n Launcher3
```
