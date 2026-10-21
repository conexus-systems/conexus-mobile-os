# ADR-007: System Services como Processos Isolados (Binder / D-Bus) — APROVADO

**Status:** Aceito
**Data:** 2026-10-19
**Decisores:** Tech Lead (A0), Architect, Platform (A2), Framework (A4), Security (A5)
**Assinatura:** A0

---

## Contexto

O Conexus Mobile OS precisa de system services:
- Tile Engine (scheduling, updates, rendering)
- Notification Broker (receipt, grouping, actions)
- App Manager (catalog, launch, lifecycle)
- Search (local indexing, authorized content)
- Update (OTA, verification, rollback)
- Security (policies, permissions, audit)
- Bridge-Android (Android platform integration)

No Android (AOSP): Services rodam como processos separados, comunicam via Binder IPC (AIDL/HIDL). System server hospeda muitos services built-in.

No Linux (Lumia research): D-Bus e standard para IPC entre system services.

Opcoes:
1. Monolitico: Tudo no shell process (simples, mas single point of failure, security risk)
2. Android-style: Cada service = processo separado, Binder IPC (AIDL)
3. D-Bus style: Cada service = processo, D-Bus (mais portable para Linux)
4. Hybrid: Abstract IPC layer, Binder no Android, D-Bus no Linux

---

## Decisao

System services como processos isolados com camada de IPC abstrata.

### Arquitetura

```
+-----------------------------------------------------------------+
|                      CONEXUS SHELL (UI)                          |
+-----------------------------------------------------------------+
|                    IPC ABSTRACTION LAYER                         |
|  Interface: IConnection, IServiceManager, IProxy, IStub         |
+------------+------------+------------+--------------------------+
|  ANDROID   |  LINUX     |  TEST/MOCK |  FUTURE                  |
|  Binder    |  D-Bus     |  In-process|  gRPC, etc.              |
|  (AIDL)    |  (systemd) |  (unit     |                          |
|            |            |   tests)   |                          |
+------------+------------+------------+--------------------------+
         |            |            |
+--------v--+  +-------v----+ +-----v------+
|Tile Engine|  |Notif Broker| |App Manager|  ... (cada = processo)
+-----------+  +------------+ +-----------+
```

### Implementacao por Target

**Android (AOSP):**
- Services = android.app.Service subclasses (foreground services)
- IPC = AIDL interfaces em framework/sdk/kotlin/aidl/
- Registration = ServiceManager.addService() ou custom ConexusServiceManager
- SELinux policies por service (platforms/android/security/)

**Linux (Lumia):**
- Services = systemd user services ou system services
- IPC = D-Bus (system bus ou session bus)
- Interface definitions = XML D-Bus introspection ou GVariant
- Sandboxing = systemd units + capabilities + namespaces

**Shared (Framework):**
- Kotlin interfaces em framework/sdk/kotlin/services/
- TypeScript interfaces em framework/sdk/typescript/services/
- Codegen: AIDL para Kotlin, D-Bus XML para TS/Kotlin stubs

### Service Contracts (Exemplo)

```kotlin
// framework/sdk/kotlin/services/ITileEngine.aidl
package com.conexus.services.tile;

interface ITileEngine {
    void registerTileProvider(in ITileProvider provider);
    void unregisterTileProvider(in ITileProvider provider);
    void requestTileUpdate(String tileId, TileContent content);
    TileContent getTileContent(String tileId);
    List<TileInfo> getAllTiles();
}
```

---

## Consequencias

### Positivas
- Isolamento de falhas: Crash em Tile Engine nao derruba Shell
- Seguranca: Least privilege per service (SELinux/D-Bus policies)
- Testabilidade: Mock IPC layer para unit tests
- Portabilidade: Mesma interface, implementacao IPC por target
- Escalabilidade: Services independentes, deploy/upgrade isolado

### Negativas
- Complexidade: IPC layer, service discovery, lifecycle management
- Performance: IPC overhead vs in-process calls
- Debugging: Distributed tracing necessario
- Boot order: Dependency graph entre services

### Riscos
- Binder/D-Bus parity: APIs divergirem — Mitigacao: Shared interface definitions + codegen
- Android system server integration: Conexus services vs AOSP system services — Mitigacao: Rodar como user services, nao modificar system server
- Linux D-Bus permissions: — Mitigacao: Policy files, systemd hardening

---

## Alternativas Consideradas

| Alternativa | Pros | Contras | Por que nao |
|-------------|------|---------|-------------|
| Monolitico (tudo no shell) | Simples, zero IPC | Single point of failure, security, testability, scaling | Nao aceitavel para SO |
| So AIDL (Android-only) | Nativo Android | Nao porta para Linux/Lumia | Trava research track |
| So D-Bus | Portable | Nao nativo Android, overhead no AOSP | Android usa Binder nativamente |
| gRPC/HTTP para tudo | Universal, tooling | Overhead alto, nao integra com platform security (SELinux) | Nao adequado para system IPC |

---

## Plano de Implementacao

1. M1: Definir interface abstraction layer (framework/sdk/*/ipc/)
2. M1: AIDL definitions para todos services (framework/sdk/kotlin/aidl/)
3. M2: Android implementation: Services + AIDL stubs + SELinux policies
4. M3: Shell integration via IPC layer (mock para tests)
5. M5 (Lumia): D-Bus implementation se Linux boot validado

---

## Referencias

- Android AIDL (https://source.android.com/docs/core/aidl)
- Android Binder IPC (https://source.android.com/docs/core/architecture/binder)
- D-Bus Specification (https://dbus.freedesktop.org/doc/dbus-specification.html)
- systemd Service Management (https://www.freedesktop.org/software/systemd/man/systemd.service.html)
- SDD v0.1 4.3