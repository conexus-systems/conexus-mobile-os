# Compatibility Matrix — Windows 10 Mobile Features → Conexus Mobile OS

**Versão:** v0.1  
**Data:** 19 de outubro de 2026  
**Status:** Template — preencher M1/M3

---

## Objetivo

Mapear cada feature do Windows 10 Mobile para:
1. **Viabilidade** no Conexus (Android/Linux)
2. **Esforço estimado** (S/M/L/XL)
3. **Dependências** (hardware, platform, legal)
4. **Decisão** (Include MVP / Include Alpha / Defer / Drop)

---

## Matriz por Categoria

### Start & Live Tiles

| Feature Win10M | Viabilidade Android | Viabilidade Linux | Esforço | Dependências | Decisão | Notas |
|----------------|---------------------|-------------------|---------|--------------|---------|-------|
| Grid 4-col (portrait) / 6-col (landscape) | ✅ Native (RecyclerView/Compose) | ✅ Custom | S | — | **MVP** | Core experience |
| Tile sizes: S/M/W/L | ✅ | ✅ | S | — | **MVP** | |
| Tile groups + labels | ✅ | ✅ | S | Persistence | **MVP** | |
| Live Tile flip animation | ✅ Compose/Canvas | ✅ Custom | M | — | **MVP** | 3D flip = GPU |
| Live Tile content update (push/periodic) | ✅ WorkManager/FCM | ✅ systemd timers + push | M | Bridge-Android | **MVP** | |
| Tile resize (drag handles) | ✅ | ✅ | M | — | **MVP** | |
| All Apps list (alphabetical + jump list) | ✅ | ✅ | S | AppManager | **MVP** | |
| Integrated search (apps + settings + web) | ✅ | ✅ | M | Search service | **Alpha** | Web = browser intent |
| Folders (tiles inside tile) | ✅ | ✅ | M | — | **Alpha** | Win10M não tinha; adicionar |
| Tile transparency/acrylic | ⚠️ Compose experimental | ⚠️ Custom shader | L | GPU, battery | **Beta** | Medir impacto |

---

### Navigation & Multitasking

| Feature Win10M | Viabilidade Android | Viabilidade Linux | Esforço | Dependências | Decisão | Notas |
|----------------|---------------------|-------------------|---------|--------------|---------|-------|
| Back button (hardware/software) | ✅ Native | ✅ Custom | S | — | **MVP** | |
| Edge swipe gestures (back, action center, notifications) | ✅ GestureDetector | ✅ Custom | M | — | **Alpha** | Conflito com apps |
| Task switcher (card stack) | ✅ Recents API | ✅ Custom | M | AppManager | **MVP** | |
| Pivot/Panorama pattern (in-app) | ✅ ViewPager2/Compose | ✅ Custom | S | SDK | **SDK** | Para apps Conexus |
| Split screen / multi-window | ✅ Android 7+ | ⚠️ Wayland/Weston | L | Platform | **Beta** | Lumia: não |
| Timeline / Activity history | ❌ (Cortana deprecated) | ❌ | XL | Cloud sync | **Drop** | Substituir por local recents |

---

### Action Center & Notifications

| Feature Win10M | Viabilidade Android | Viabilidade Linux | Esforço | Dependências | Decisão | Notas |
|----------------|---------------------|-------------------|---------|--------------|---------|-------|
| Notification shade (swipe down) | ✅ Native (NotificationListener) | ✅ Custom | S | — | **MVP** | |
| Grouped notifications (by app/time) | ✅ | ✅ | S | NotificationBroker | **MVP** | |
| Expandable notifications + actions | ✅ | ✅ | S | — | **MVP** | |
| Quick Actions grid (toggles) | ✅ Tiles/Compose | ✅ Custom | S | System services | **MVP** | |
| Long press quick action → settings | ✅ Intent | ✅ Deep link | S | — | **MVP** | |
| Notification channels/categories | ✅ Android O+ | ✅ Custom | S | — | **MVP** | |
| Priority/Heads-up (peek) | ✅ | ✅ | S | — | **Alpha** | |
| Notification history | ✅ Android 11+ | ✅ Custom | S | — | **Alpha** | |
| Reply inline (Direct Reply) | ✅ RemoteInput | ✅ Custom | M | — | **Alpha** | |

---

### Settings

| Feature Win10M | Viabilidade Android | Viabilidade Linux | Esforço | Dependências | Decisão | Notas |
|----------------|---------------------|-------------------|---------|--------------|---------|-------|
| Categories (System, Devices, Network, Personalization, Accounts, Time, Privacy, Update) | ✅ | ✅ | M | Settings service | **MVP** | Reorganizar para mobile |
| Search within settings | ✅ | ✅ | S | — | **MVP** | |
| Per-app permissions | ✅ Native | ✅ Custom | S | PermissionManager | **MVP** | |
| Theme (Light/Dark/Auto) | ✅ | ✅ | S | Fluent tokens | **MVP** | |
| Accent color picker | ✅ | ✅ | S | Fluent tokens | **Alpha** | |
| Display (brightness, night light, scaling) | ✅ Native | ⚠️ Hardware | M | Platform | **Alpha** | |
| Storage sense / Storage | ✅ | ✅ | M | — | **Alpha** | |
| Battery saver / Battery usage | ✅ Native | ⚠️ Hardware | M | Platform | **Alpha** | |
| Privacy dashboard | ✅ | ✅ | L | — | **Beta** | |

---

### Input & Keyboard

| Feature Win10M | Viabilidade Android | Viabilidade Linux | Esforço | Dependências | Decisão | Notas |
|----------------|---------------------|-------------------|---------|--------------|---------|-------|
| QWERTY layout (pt-BR, en-US, etc.) | ✅ Native IME | ✅ Custom IME | M | — | **MVP** | |
| Shape Writing (swipe typing) | ✅ Native (Gboard-like) | ⚠️ Complex | XL | ML model | **Alpha** | Usar lib existente |
| Cursor control (magnifier) | ✅ Native | ✅ Custom | M | — | **MVP** | |
| Voice typing | ✅ Native | ⚠️ Cloud/On-device | L | STT service | **Beta** | |
| Handwriting | ❌ Deprecated | ❌ | XL | — | **Drop** | |
| Clipboard history | ✅ Android 13+ | ✅ Custom | S | — | **Alpha** | |
| One-handed mode | ✅ | ✅ | S | — | **Alpha** | |

---

### Lock Screen

| Feature Win10M | Viabilidade Android | Viabilidade Linux | Esforço | Dependências | Decisão | Notas |
|----------------|---------------------|-------------------|---------|--------------|---------|-------|
| Clock + date (typographic) | ✅ | ✅ | S | — | **MVP** | |
| Notifications on lock (privacy control) | ✅ Native | ✅ Custom | S | — | **MVP** | |
| Quick actions on lock | ✅ | ✅ | S | — | **Alpha** | |
| Camera quick launch | ✅ Intent | ✅ Custom | S | — | **MVP** | |
| PIN / Password / Pattern | ✅ Native | ✅ Custom | S | — | **MVP** | |
| Biometric (fingerprint/face) | ✅ BiometricPrompt | ⚠️ Hardware | M | Platform | **Alpha** | Lumia: não |
| Glance screen (always-on) | ✅ AOD (OLED) | ⚠️ Hardware | M | Platform | **Beta** | |

---

### Connectivity & Hardware

| Feature Win10M | Viabilidade Android | Viabilidade Linux | Esforço | Dependências | Decisão | Notas |
|----------------|---------------------|-------------------|---------|--------------|---------|-------|
| Phone (calls, SMS, Voicemail) | ✅ Native (TelecomManager) | ❌ Lumia: modem proprietary | — | Platform | **MVP (Android)** / **Drop (Lumia)** | |
| Mobile data (LTE/5G) | ✅ Native | ❌ Lumia: proprietary | — | Platform | **MVP (Android)** / **Drop (Lumia)** | |
| Wi-Fi / Wi-Fi Direct / Hotspot | ✅ Native | ⚠️ Partial | M | Platform | **MVP (Android)** / **Research (Lumia)** | |
| Bluetooth (A2DP, HFP, LE) | ✅ Native | ⚠️ Partial | M | Platform | **MVP (Android)** / **Research (Lumia)** | |
| NFC / Tap to pay | ✅ Native | ❌ Lumia: proprietary | — | Platform | **Alpha (Android)** / **Drop (Lumia)** | |
| Camera (photo, video, barcode) | ✅ CameraX / Camera2 | ❌ Lumia: proprietary | M | Platform | **MVP (Android)** / **Drop (Lumia)** | |
| Sensors (accel, gyro, prox, light, compass) | ✅ Native | ⚠️ Partial | S | Platform | **MVP (Android)** / **Research (Lumia)** | |
| GPS / GNSS | ✅ Native | ❌ Lumia: proprietary | — | Platform | **MVP (Android)** / **Drop (Lumia)** | |
| USB (OTG, charging, data) | ✅ Native | ⚠️ Partial | S | Platform | **MVP (Android)** / **Research (Lumia)** | |

---

### Platform & System

| Feature Win10M | Viabilidade Android | Viabilidade Linux | Esforço | Dependências | Decisão | Notas |
|----------------|---------------------|-------------------|---------|--------------|---------|-------|
| App platform (UWP → Conexus SDK) | ✅ Android + SDK | ✅ SDK | XL | Framework | **MVP (SDK)** | |
| App sandbox / permissions | ✅ Native | ✅ systemd + namespaces | M | Security | **MVP** | |
| OTA updates (A/B) | ✅ update_engine | ✅ Custom/RAUC | M | ADR-008 | **Alpha** | |
| Verified boot (AVB/dm-verity) | ✅ Native | ✅ Custom | M | ADR-005 | **MVP** | |
| Encryption (FBE) | ✅ Native | ✅ fscrypt/LUKS | M | Platform | **MVP** | |
| SELinux / AppArmor | ✅ Native | ✅ AppArmor | M | Security | **MVP** | |
| Backup/Restore | ✅ Native (Auto Backup) | ✅ Custom | M | — | **Alpha** | |
| Device admin / MDM | ✅ DevicePolicyManager | ✅ Custom | L | — | **Beta** | Enterprise |
| Kiosk mode | ✅ Lock Task Mode | ✅ Custom | M | — | **Beta** | |

---

### Deprecated / Dropped (Win10M → Não no Conexus)

| Feature | Motivo | Substituto |
|---------|--------|------------|
| Cortana | Descontinuada Microsoft | Assistant próprio (futuro) / Google Assistant |
| Windows Hello (iris/face Lumia) | Hardware proprietário | BiometricPrompt padrão |
| Continuum (desktop mode via dock) | Hardware específico | Desktop Mode via USB-C/DP Alt Mode (genérico) |
| SD card as internal storage | Android adopta storage | Adoptable storage nativo |
| Kids Corner | Funcionalidade nicho | Multiple users / Restricted profiles |
| Data Sense | Substituído por Android Data Saver | Data Saver nativo |
| Wi-Fi Sense | Privacidade/descontinuado | Não replicar |
| Bluetooth file transfer (OBEX) | Obsoleto | Nearby Share / Quick Share |
| FM Radio | Hardware raro | Streaming apps |
| Office Mobile (built-in) | Apps separados | Office PWA / Apps |
| OneDrive placeholder files | OneDrive Files On-Demand | OneDrive app |
| Windows Store | Não GMS | Conexus Store (futuro) / F-Droid / APK |

---

## Legenda

- ✅ = Totalmente viável com APIs nativas
- ⚠️ = Parcialmente viável / requer trabalho custom / hardware dependency
- ❌ = Não viável (hardware proprietário, API não existe, deprecated)
- **MVP** = Incluir no M3 Gate (MVP funcional)
- **Alpha** = Incluir no M4 Gate (Alpha release)
- **Beta** = Incluir no Beta/Stable
- **SDK** = Expor via SDK para apps
- **Drop** = Não implementar
- **Research** = Investigar no M5 (Lumia)

---

## Próximos Passos

1. [ ] A1 + A2 + A3 + A4 preencher viabilidade/esforço baseado em research técnico
2. [ ] A0 revisa e aprova decisões
3. [ ] Atualizar `BEHAVIOR_MATRIX.md` com decisões finais
4. [ ] Rastrear para test cases e milestones