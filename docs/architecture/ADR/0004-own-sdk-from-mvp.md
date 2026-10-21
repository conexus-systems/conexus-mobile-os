# ADR-004: SDK Próprio desde MVP (Kotlin + TypeScript/React Native) — **APROVADO**

**Status:** Aceito
**Data:** 2026-10-19
**Decisores:** Tech Lead (A0), Architect, Framework (A4), Shell (A3)
**Assinatura:** A0 ✅

---

## Contexto

Apps no Conexus Mobile OS precisam de:
- Live Tiles creation e updates
- Navigation integration (deep links, back stack)
- Notifications via Action Center
- Permissions model alinhado ao SO
- Fluent Design System components
- Cross-platform capability (Android + future Linux/Lumia)

Opções:
1. Usar apenas Android SDK padrão → Apps são "apps Android", sem integração Conexus nativa
2. Flutter como SDK único → Single codebase, mas não nativo Android, performance/integração questões
3. React Native apenas → JavaScript bridge overhead, não ideal para system UI
4. **Dual SDK: Kotlin (Android-native) + TypeScript/React Native (cross-platform)**

---

## Decisão

**Desenvolver SDK próprio desde MVP com duas implementações:**

### Kotlin SDK (`framework/sdk/kotlin/`)
- Target: Apps Android nativos, system apps, deep platform integration
- APIs: TileProvider, TileUpdater, NavController, NotificationBuilder, PermissionManager, FluentTheme
- Distribuição: Maven artifact via GHCR (`ghcr.io/conexus-mobile-os/conexus-sdk-kotlin`)

### TypeScript/React Native SDK (`framework/sdk/typescript/`)
- Target: Cross-platform apps (React Native, Web, future Linux)
- APIs: Components (Tile, Button, Card, Nav), Hooks (useTile, useNavigation, usePermissions), Theme, Native Bridge
- Distribuição: npm package via GHCR (`@conexus-mobile-os/sdk`)

### Shared Contracts (`framework/shared/`)
- JSON Schema / TypeBox definitions para: Tile Manifest, App Manifest, Notification Schema
- Versionamento semver independente
- Validation em build time e runtime

### Reference App (`apps/reference-app/`)
- Demonstra: Tile creation + live update, Notification send/receive, Navigation, Fluent theming
- Implementado em **ambos** SDKs (Kotlin + TS) para validar parity

---

## Consequências

### Positivas
- Apps nativos Android têm integração total (performance, system APIs)
- Cross-platform apps possíveis sem reescrita completa
- Contratos versionados = evolução controlada
- Reference app valida ambos SDKs desde cedo

### Negativas
- Manter dois SDKs = esforço dobrado
- Parity entre Kotlin e TS APIs requer disciplina
- Native bridge para TS SDK adiciona complexidade
- Versionamento coordenado necessário

### Riscos
- **SDK fragmentation:** APIs divergem → Mitigação: Shared contracts como source of truth, CI validation
- **TS Native bridge performance:** → Mitigação: JSI (React Native 0.76+), benchmark M3
- **Developer adoption:** Dois SDKs confundem → Mitigação: Docs claros, migration guide, "start with Kotlin for system, TS for consumer"

---

## Alternativas Consideradas

| Alternativa | Prós | Contras | Por que não |
|-------------|------|---------|-------------|
| Só Kotlin | Simples, nativo | Sem cross-platform, exclui RN/Web devs | Limita ecossistema |
| Só TypeScript/RN | Cross-platform nativo | Bridge overhead, não ideal para system UI, performance | System apps precisam Kotlin |
| Flutter | Single codebase, bom performance | Não nativo Android, Dart ecosystem, system integration limitada | Não alinha com AOSP stack |
| Kotlin Multiplatform (KMP) | Compartilha logic, nativo ambos | Ainda imaturo para UI, RN ecosystem maior | Avaliar futuramente (M5+) |

---

## Plano de Implementação

1. **M1:** Definir contracts (schemas, interfaces) em `framework/shared/`
2. **M1:** Kotlin SDK skeleton com TileProvider, NavController interfaces
3. **M2:** TS SDK skeleton com components, hooks, JSI bridge setup
4. **M3 (Gate):** Reference app em **ambos** SDKs roda no device: tile live, notif, nav
5. **M4:** Publish artifacts, docs, migration guide, sample apps

---

## Referências

- [SDD v0.1 §4.4, §7 A4](./../../SDD.md)
- [React Native JSI](https://reactnative.dev/docs/jsi)
- [Kotlin Multiplatform](https://kotlinlang.org/lp/multiplatform/)