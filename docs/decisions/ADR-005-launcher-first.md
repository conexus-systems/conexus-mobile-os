# ADR-005: Launcher Primeiro; Sistema Operacional Depois

**Status:** Aceito  
**Data:** 2026-10-22  
**Decisores:** H0 (Orchestrator), H1 (Architect)

---

## Contexto

O escopo final do Conexus Harmattan inclui uma imagem de sistema própria (boot, kernel, init, services, launcher). No entanto, construir um SO completo requer device trees, kernel config, vendor blobs, bootloader unlock, recovery, validação em hardware — tudo isso antes de validar a UX e a arquitetura do launcher.

## Decisão

**Iniciar com uma aplicação executável (Launcher) que roda sobre SO existente**, sem exigir imagem de sistema personalizada. A construção de imagem de sistema será meta posterior (Fase 8), com requisitos independentes.

## Consequências

### Positivas
- **Validação precoce:** UX, arquitetura, core testados em semanas, não meses
- **Iteração rápida:** Mudança de UI/launcher = recompilar app, não reflashar device
- **Device pool maior:** Qualquer device Android 7+ / Linux desktop vira target de teste
- **Feedback real:** Usuários testam launcher na Play Store / Flathub antes de commit SO
- **Risco isolado:** Falha no launcher não briga device; falha no SO = brick

### Negativas
- **Limitações de launcher Android:** Não controla system services, não substitui notch/status bar, sandbox restrito
- **Funcionalidades postergadas:** Tile engine system-wide, notification broker global, profile system service — só no SO próprio
- **Duplicação temporária:** Launcher app + futuro system service podem coexistir

## Escopo do Launcher (Fase 4–7)

| Incluído no Launcher App | Fora do Launcher (SO Próprio) |
|--------------------------|-------------------------------|
| Home grid (tiles S/M/W/L) | Kernel config, bootloader |
| App drawer + search | Vendor blobs, device tree |
| Action Center (notifications) | Init, recovery, partition layout |
| Settings app (Conexus config) | SELinux policies, verified boot |
| Profiles (regras locais) | System services (D-Bus system bus) |
| Theme/accent/dynamic color | OTA update engine (A/B) |
| Gestos de navegação | Hardware bring-up (display, touch, modem) |

## Transição para SO Próprio (Fase 8)

**Critérios de entrada para Fase 8:**
1. Launcher validado em ≥3 devices Android + ≥2 Linux distros
2. Core services estáveis, APIs congeladas (semver)
3. Device candidatos selecionados com bootloader unlock + kernel source
4. Team dedicado a platform/boot (separado de UI/core team)

**Entregáveis Fase 8:**
- `platforms/android/` device trees + BoardConfig
- Kernel config + build scripts
- Vendor blobs extraction pipeline
- Recovery image + flash tools
- OTA metadata + signing pipeline
- SBOM + security audit

## Alternativas Consideradas

| Alternativa | Avaliação |
|-------------|-----------|
| **SO completo desde o início** | Alto risco, validação tardia, device pool pequeno |
| **Launcher + System Service (Android)** | System service precisa root/signature — não viável para distribuição |
| **Apenas SO (sem launcher app)** | Sem validação UX precoce, feedback loop lento |

## Validação

- [ ] Launcher APK instalável em Android 7+ (minSdk 24)
- [ ] Launcher executável em Linux (Wayland/X11, Flatpak/AppImage)
- [ ] Core services funcionais sem system privileges
- [ ] Documentação clara: "Isto é um launcher; SO próprio = Fase 8"

---

**Assinatura:** H0, H1  
**Próxima revisão:** Fase 8 gate — decisão de prosseguir com SO próprio