# ADR-007: Arquitetura Modular Antes de Microserviços

**Status:** Aceito  
**Data:** 2026-10-22  
**Decisores:** H0 (Orchestrator), H1 (Architect)

---

## Contexto

A arquitetura do Conexus Harmattan inclui múltiplos serviços (Tile Engine, Notification Broker, App Manager, Profile Manager, Driver Manager). A decisão é se esses serviços rodam como processos separados (microserviços) ou como módulos dentro do mesmo processo (arquitetura modular).

## Decisão

**Iniciar como aplicação modular (single process, multiple crates).** Usar processos separados somente quando houver uma justificativa concreta de segurança, confiabilidade ou isolamento.

## Consequências

### Positivas
- **Simplicidade:** Um binário (`conexus-launcher`), deploy trivial
- **Performance:** Zero IPC overhead, function calls diretas
- **Debugging:** Stack traces completos, breakpoints fáceis
- **Testes:** Integration tests = unit tests (mesmo processo)
- **Desenvolvimento:** `cargo run` = tudo roda, sem orchestration

### Negativas
- **Fault isolation:** Crash em um módulo derruba tudo (mitigado com `panic::catch_unwind` em boundaries)
- **Security boundary:** Um exploit no Tile Engine afeta Notifications (mitigado com `unsafe` isolado)
- **Scaling:** Não escala horizontalmente (não necessário para launcher)

## Quando Separar Processos (Critérios)

| Critério | Exemplo | Ação |
|----------|---------|------|
| **Privilégios diferentes** | System service precisa root/capabilities | Processo separado (`conexus-daemon`) |
| **Fault domain crítico** | Driver manager carrega blobs proprietários | Isolar em processo sandboxed |
| **Lifecycle independente** | Tile engine deve sobreviver a launcher restart | System service (D-Bus activatable) |
| **Resource isolation** | Memory leak em um módulo não deve afetar outros | Processo separado + cgroups |
| **Security boundary** | Código não-confiável (plugins terceiros) | WASM sandbox ou processo separado |

## Implementação Atual (Modular)

```
conexus-launcher (binary)
├── conexus-core (tiles, notifications, apps, drivers)  — lógica pura
├── conexus-profiles, conexus-apps, conexus-permissions — services
├── conexus-config, conexus-events, conexus-diagnostics — infra
├── conexus-platform-linux / -android — adapters (traits)
└── conexus-ui — UI framework (Slint/Iced/egui)
```

**Comunicação interna:** Function calls + `tokio::sync` channels + `tracing` events.

**Processos separados (Futuro, Fase 8+):**

| Serviço | Processo | Bus | Justificativa |
|---------|----------|-----|---------------|
| `conexus-daemon` | Systemd service | D-Bus (system bus) | System services, boot activation |
| `conexus-tile-engine` | Daemon | D-Bus (session) | Survive launcher restart, widgets |
| `conexus-notifications` | Daemon | D-Bus (session) | System-wide notification broker |

## Alternativas Consideradas

| Alternativa | Avaliação |
|-------------|-----------|
| **Microserviços desde o início** | Overhead massivo: D-Bus/Binder, service discovery, deployment, debugging |
| **Monolito único (sem crates)** | Acoplamento forte, testes difíceis, recompilação total |
| **Erlang/Elixir style actors** | Não Rust, runtime próprio, overkill |

## Validação

- [ ] `conexus-launcher` compila e roda como single binary
- [ ] Crates independentes: `cargo test -p conexus-core`, `-p conexus-profiles`, etc
- [ ] Integration tests usam mesma crate graph (não IPC)
- [ ] `panic::catch_unwind` em boundaries críticos (plugin loading, JNI)
- [ ] Documentação: quando separar processo (critérios acima)

---

**Assinatura:** H0, H1  
**Próxima revisão:** Fase 8 (SO próprio) — reavaliar separação de serviços