# ADR-002: Core Compartilhado em Rust

**Status:** Aceito  
**Data:** 2026-10-22  
**Decisores:** H0 (Orchestrator), H1 (Architect)

---

## Contexto

O Conexus Harmattan deve rodar em duas plataformas distintas: Linux (desktop/Wayland) e Android. A lógica de negócio, serviços de sistema e contratos devem ser compartilhados para evitar duplicação, garantir consistência e permitir testes cruzados.

## Decisão

**Implementar a lógica de domínio, serviços de sistema e contratos compartilháveis em Rust**, organizados em crates independentes no workspace Cargo.

## Consequências

### Positivas
- **Single source of truth:** Regras de tile, notificação, app, perfil em um lugar
- **Type safety:** Contratos como traits Rust — erros de integração viram erros de compilação
- **Testabilidade:** Core testável sem device, sem platform, sem UI
- **Cross-platform:** Mesma crate compila para `x86_64-linux` e `aarch64-android`
- **Performance:** Zero-cost abstractions, sem GC, controle de memória
- **Ecosystem:** `serde`, `tokio`, `tracing`, `thiserror` maduros

### Negativas
- **Curva de aprendizado:** Team precisa Rust proficiency
- **JNI/FFI overhead:** Android adapter requer JNI (isolado em `platform/android`)
- **Compile times:** Workspace grande pode ser lento (mitigado com `cargo check` incremental)

## Estrutura de Crates (Core Compartilhado)

```
conexus-error/      # Tipos de erro unificados
conexus-config/     # Config persistente + migrations
conexus-events/     # Event bus interno
conexus-diagnostics/# Logs, métricas, diagnostics
conexus-platform/   # TRAITS de plataforma (contratos)
conexus-sdk/        # Schemas validation (tile, app, notification)
conexus-core/       # Services logic (tiles, notifications, apps, drivers)
conexus-profiles/   # Profile management
conexus-apps/       # App catalog + launch (wraps platform)
conexus-permissions/# Policy engine
```

## Platform Traits (Contratos)

```rust
// conexus-platform crate — traits públicos
pub trait AppDiscovery: Send + Sync { ... }
pub trait NotificationBackend: Send + Sync { ... }
pub trait PermissionManager: Send + Sync { ... }
pub trait ConfigStore: Send + Sync { ... }
pub trait PlatformDiagnostics: Send + Sync { ... }
```

**Regra:** Core **nunca** importa `platform-linux` ou `platform-android`. Apenas traits de `conexus-platform`.

## Platform Adapters (Implementações)

| Platform | Crate | Mecanismo |
|----------|-------|-----------|
| Linux | `conexus-platform-linux` | D-Bus, desktop files, notify-rust, sysinfo |
| Android | `conexus-platform-android` | JNI → PackageManager, NotificationManager, SharedPreferences |

## Alternativas Consideradas

| Alternativa | Avaliação |
|-------------|-----------|
| **Kotlin Multiplatform (KMP)** | Bom para Android, fraco no Linux desktop, UI não compartilhada |
| **TypeScript/React Native** | Não roda nativamente no Linux desktop, performance inferior |
| **C++ shared library** | Unsafe, manual memory, sem type safety cross-language |
| **Go shared library** | CGO overhead, não roda bem no Android, sem generics (até 1.18) |

## Validação

- ✅ `cargo check --workspace --target x86_64-unknown-linux-gnu` passa
- ✅ `cargo check --workspace --target aarch64-linux-android` passa
- ✅ Core tests passam sem device (`cargo test --workspace --lib`)
- ✅ Platform adapters implementam traits sem breaking changes

---

**Assinatura:** H0, H1  
**Próxima revisão:** Se nova plataforma alvo adicionada (iOS, Windows, WebAssembly)