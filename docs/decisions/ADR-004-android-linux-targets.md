# ADR-004: Android e Linux como Alvos

**Status:** Aceito  
**Data:** 2026-10-22  
**Decisores:** H0 (Orchestrator), H1 (Architect), H4 (Platform Adapter)

---

## Contexto

O Conexus Harmattan deve funcionar em dois ambientes fundamentalmente diferentes:
- **Linux:** Desktop/Wayland/X11, systemd, D-Bus, XDG, portals
- **Android:** Framework Java/Kotlin, Binder IPC, sandbox, permissions, Play Store

A arquitetura deve suportar ambos sem duplicação de lógica de negócio.

## Decisão

**Manter um núcleo compartilhado em Rust (`conexus-*` crates) e adaptadores específicos por plataforma** (`conexus-platform-linux`, `conexus-platform-android`).

## Consequências

### Positivas
- **Lógica única:** Core services, schemas, config, profiles = 1 código
- **Paridade funcional:** Features classificadas por capacidade de plataforma
- **Testes cruzados:** Core testado no Linux, valida Android
- **Deploy flexível:** Launcher Android (APK) + Linux App (Flatpak/AppImage/native)

### Negativas
- **Divergência de APIs:** Linux = D-Bus/Files/Portals; Android = Binder/Intents/ContentProviders
- **Capacidades assimétricas:** Ex: runtime permissions só Android; system services só Linux
- **Testing complexity:** Precisa device farm Linux + Android

## Classificação de Funcionalidades por Plataforma

| Funcionalidade | Linux | Android | Notas |
|----------------|-------|---------|-------|
| **App Discovery** | ✅ `.desktop` files, portals | ✅ `PackageManager` | Contrato unificado |
| **App Launch** | ✅ `xdg-open`, `gio` | ✅ Intents | Contrato unificado |
| **Notifications** | ✅ Freedesktop/DBus | ✅ `NotificationManager` | Actions: ambos suportam |
| **Runtime Permissions** | ⚠️ Portals/Polkit | ✅ Nativo API 23+ | Linux limitado |
| **Config Store** | ✅ XDG `~/.config` | ✅ `DataStore`/`SharedPreferences` | Contrato unificado |
| **Tile Engine** | ✅ Custom (launcher-integrado) | ✅ `AppWidgetManager` + custom | Diverge em implementação |
| **System Services** | ✅ D-Bus (systemd) | ❌ Não aplicável | Linux-only |
| **Dynamic Color** | ⚠️ Static fallback | ✅ Material You (API 31+) | ADR-010 |
| **Launcher Registration** | `.desktop` + DBus activation | `CATEGORY_HOME` intent | Diferente |

## Platform Capabilities (Runtime Detection)

```rust
// Cada adapter expõe capabilities reais
pub struct AppDiscoveryCapabilities {
    pub supports_launch_intent: bool,
    pub supports_app_shortcuts: bool,
    pub supports_app_categories: bool,
}

pub struct NotificationCapabilities {
    pub supports_actions: bool,
    pub supports_inline_reply: bool,
    pub supports_images: bool,
    pub supports_progress: bool,
    pub supports_channels: bool,
}
```

**Regra:** Core **nunca** assume capability — consulta `capabilities()` no adapter.

## Implementação

| Platform | Crate | Mecanismo IPC |
|----------|-------|---------------|
| Linux | `conexus-platform-linux` | D-Bus (zbus), desktop files, notify-rust, portals |
| Android | `conexus-platform-android` | JNI → PackageManager, NotificationManager, SharedPreferences |

## Alternativas Consideradas

| Alternativa | Avaliação |
|-------------|-----------|
| **Kotlin Multiplatform (KMP)** | UI não compartilhada, Linux support fraco |
| **Flutter/Dart** | Não Rust, GC, não integra com core Rust |
| **React Native / TypeScript** | Não nativo Linux desktop, performance |
| **Dois codebases separados** | Duplicação de lógica, inconsistências |

## Validação

- [ ] Core crate compila para `x86_64-unknown-linux-gnu` e `aarch64-linux-android`
- [ ] Platform traits implementados em ambos adapters
- [ ] Capabilities matrix documentada e testada
- [ ] CI roda `cargo check` nos dois targets
- [ ] Launcher funcional básico nos dois ambientes

---

**Assinatura:** H0, H1, H4  
**Próxima revisão:** Se nova plataforma adicionada (iOS, Windows, WebAssembly via WASM)