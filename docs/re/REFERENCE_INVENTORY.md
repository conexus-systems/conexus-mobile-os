# Reference Inventory — Windows 10 Mobile Build 15254.603

**Versão:** v0.2  
**Data:** 19 de outubro de 2026  
**Status:** M0 — Dados de pesquisa inicial (clean room)

---

## Identificação da Referência

| Campo | Valor |
|-------|-------|
| **OS** | Windows 10 Mobile |
| **Version** | 1709 (Fall Creators Update) |
| **Build** | 15254.603 (final documented update, Jan 2020) |
| **Device** | Lumia 950 XL (RM-1116) — primary reference |
| **SoC** | Qualcomm Snapdragon 810 (MSM8994) |
| **RAM** | 3 GB LPDDR4 |
| **Storage** | 32 GB eMMC 5.0 |
| **Display** | 5.7" 1440x2560 AMOLED, 564 PPI, 16:9 |
| **Idioma** | Português (Brasil) / English (US) |
| **Resolução** | 1440x2560 (WQHD) |
| **DPI** | 564 ppi |
| **Escala** | 100%, 125%, 150%, 200% (text scaling) |
| **Firmware Source** | FFU/WIM via Windows Device Recovery Tool (legitimate) |
| **Data Captura** | 2026-10-19 (research date) |
| **Condições** | Clean install, no Microsoft account, offline mode, no SIM |

---

## Fontes de Dados (Clean Room — Observation Only)

| Fonte | Tipo | URL | Data Acesso |
|-------|------|-----|-------------|
| BetaWiki | Documentation | https://betawiki.net/wiki/Windows_10_Mobile_build_15254 | 2026-10-19 |
| Microsoft Learn (Archive) | Documentation | https://learn.microsoft.com/en-us/windows/uwp/ | 2026-10-19 |
| Windows Central (Archive) | Journalism | https://www.windowscentral.com/ | 2026-10-19 |
| LumiaWOA GitHub | Open Source | https://github.com/WOA-Project/LumiaWOA | 2026-10-19 |
| Lumia950XLPkg | Open Source (UEFI) | https://github.com/WOA-Project/Lumia950XLPkg | 2026-10-19 |
| Fluent Design System (MIT) | Open Source | https://github.com/microsoft/fluentui | 2026-10-19 |

> **Nota:** Nenhum binário proprietário foi descompilado, decompilado ou inspecionado além de formatos abertos. Assets visuais documentados via screenshots públicos e documentação.

---

## Inventário de Recursos Acessíveis (Público)

### Sistema de Arquivos (Estrutura Conhecida via Documentação)

| Caminho | Tipo | Descrição | Fonte |
|---------|------|-----------|-------|
| `Windows/` | Dir | System files, binaries, drivers | Public knowledge |
| `Program Files/WindowsApps/` | Dir | App packages (.appx/.msix) | Public knowledge |
| `Data/Users/DefApps/` | Dir | Default apps data, templates | Public knowledge |
| `EFIESP/` | Dir | UEFI boot partition (ESP) | LumiaWOA docs |
| `MainOS/` | WIM | Main OS image (Windows Imaging Format) | Public knowledge |
| `Data/SharedData/` | Dir | Shared app data, media | Public knowledge |
| `Windows/System32/` | Dir | System DLLs, kernel, drivers | Public knowledge |

### Recursos Visuais (Assets Públicos / Documentados)

| Recurso | Descrição | Formato | Notas Clean Room |
|---------|-----------|---------|------------------|
| **Start Background** | Solid color + optional parallax image | JPG/PNG | Recriar: solid accent color + subtle pattern |
| **Tile Templates** | 4 sizes: Small (1x1), Medium (2x2), Wide (4x2), Large (4x4) | XAML templates | Documentar layout grid, recriar do zero |
| **Accent Colors** | 21 system colors + custom picker | Hex/ARGB | Usar Fluent 2 palette + Conexus brand override |
| **System Icons** | Segoe MDL2 Assets font + PNG fallbacks | Font + PNG | Substituir: Fluent System Icons (MIT) |
| **Fonts** | Segoe UI (proprietary) | TTF | **Não usar** — Substituir: Inter (OFL) / Noto Sans (OFL) |
| **Animations** | EntranceThemeTransition, PaneThemeTransition, RepositionThemeTransition | XAML Storyboard | Recriar: Fluent Motion tokens (100/200/300ms) |
| **Sounds** | System sounds (tap, toast, lock, unlock) | WAV | Recriar: custom sounds ou CC0 |

### Configurações e Manifestos (Estrutura Pública)

| Arquivo | Localização Típica | Conteúdo | Abordagem Clean Room |
|---------|-------------------|----------|---------------------|
| `AppxManifest.xml` (Shell) | `WindowsApps/Microsoft.Windows.Start_*/` | Start capabilities, tile templates, extensions | Documentar schema, criar namespace próprio |
| `AppxManifest.xml` (Settings) | `WindowsApps/Microsoft.Windows.Settings_*/` | Settings pages, categories, commands | Mapear categorias → criar estrutura própria |
| `AppxManifest.xml` (Action Center) | `WindowsApps/Microsoft.Windows.ActionCenter_*/` | Notification extensions, quick actions | Definir contracts próprios |
| `Registry` (HKLM/HKCU) | `SYSTEM/Software/Microsoft/Windows/CurrentVersion/` | System settings, policies | Não acessar — reimplementar config store próprio |
| `Provisioning XML` | `Windows/Provisioning/` | OOBE, policies, MDM | Documentar conceitos, não copiar XML |

---

## Apps de Sistema Inventariados (Package Family Names Públicos)

| App | Package Family Name (Público) | Função | Relevância Conexus | Status Clean Room |
|-----|-------------------------------|--------|-------------------|-------------------|
| **Start** | `Microsoft.Windows.Start_8wekyb3d8bbwe` | Tela Iniciar, Tiles, App list | **Crítica** | Observar comportamento |
| **Action Center** | `Microsoft.Windows.ActionCenter_cw5n1h2txyewy` | Notificações, Quick Actions | **Crítica** | Observar comportamento |
| **Settings** | `Microsoft.Windows.Settings_cw5n1h2txyewy` | Configurações do sistema | **Crítica** | Observar comportamento |
| **Phone** | `Microsoft.CommsPhone_8wekyb3d8bbwe` | Telefonia, chamadas | Referência | Não reimplementar (hardware-dependent) |
| **Messaging** | `Microsoft.Messaging_8wekyb3d8bbwe` | SMS/MMS/RCS | Referência | Não reimplementar (hardware-dependent) |
| **Camera** | `Microsoft.Camera_8wekyb3d8bbwe` | Câmera, foto, vídeo | Referência | Não reimplementar (hardware-dependent) |
| **Keyboard** | `Microsoft.Keyboard_8wekyb3d8bbwe` | Teclado virtual, shape writing | **Alta** | Recriar layout + logic própria |
| **Lock Screen** | `Microsoft.LockScreen_cw5n1h2txyewy` | Tela de bloqueio, glance | **Alta** | Recriar com Fluent |
| **People** | `Microsoft.People_8wekyb3d8bbwe` | Contatos, integração social | Média | SDK contract apenas |
| **Mail/Calendar** | `Microsoft.WindowsCommunicationsApps_8wekyb3d8bbwe` | Email, calendário | Baixa | Não reimplementar |
| **Edge** | `Microsoft.MicrosoftEdge_8wekyb3d8bbwe` | Navegador | Baixa | Não reimplementar |
| **Store** | `Microsoft.WindowsStore_8wekyb3d8bbwe` | Loja de apps | Baixa | Futuro: Conexus Store |
| **File Explorer** | `Microsoft.Windows.FileExplorer_cw5n1h2txyewy` | Gerenciador de arquivos | Baixa | Futuro |
| **Alarms** | `Microsoft.WindowsAlarms_8wekyb3d8bbwe` | Alarmes, timer, world clock | Média | SDK contract |
| **Calculator** | `Microsoft.WindowsCalculator_8wekyb3d8bbwe` | Calculadora | Baixa | Futuro |
| **Maps** | `Microsoft.WindowsMaps_8wekyb3d8bbwe` | Mapas, navegação | Baixa | Futuro |
| **Photos** | `Microsoft.Windows.Photos_8wekyb3d8bbwe` | Galeria, edição | Baixa | Futuro |
| **OneDrive** | `Microsoft.SkyDrive_8wekyb3d8bbwe` | Sync nuvem | Baixa | Não reimplementar |

---

## Recursos de UI/UX Documentados (Observação Comportamental)

### Start Screen
- **Layout**: Grid 4 colunas (portrait) / 6 colunas (landscape)
- **Tile Sizes**: Small (1x1), Medium (2x2), Wide (4x2), Large (4x4)
- **Groups**: Named groups, drag-to-create, drag-to-move
- **All Apps**: Swipe right or tap "All apps" → alphabetical list with jump list (A-Z)
- **Search**: Integrated search box, filters apps + settings + web
- **Animation**: Entrance stagger 200ms, tile press scale 0.95, flip animation 600ms

### Live Tiles
- **Update Mechanisms**: Push notifications (WNS), Periodic (background task), Scheduled, Local API
- **Content Templates**: TileSquare150x150, TileWide310x150, TileSquare310x310 (XML templates)
- **Flip Animation**: 3D Y-axis rotation, 600ms cycle, front/back content
- **Badges**: Number (1-99, "99+"), Glyph (none, activity, alert, etc.)

### Action Center
- **Access**: Swipe from top edge (threshold ~20%)
- **Sections**: Quick Actions (top, 4-col grid) + Notifications (scrollable)
- **Quick Actions**: Toggle tiles (WiFi, BT, Rotation, Location, etc.), long press → Settings
- **Notifications**: Grouped by app, expandable, inline actions (Reply, Dismiss, Open)
- **Priority**: High (peek toast), Normal, Low (silent)

### Navigation
- **Back Button**: Hardware (Lumia) or Software (on-screen), long press = Task Switcher
- **Task Switcher**: Card stack horizontal, swipe to dismiss, tap to resume
- **Gestures**: Edge swipe left = back, right = action center, up = notifications (configurable)
- **Pivot/Panorama**: In-app pattern, horizontal swipe between sections, sticky header

### Keyboard
- **Layouts**: QWERTY (per language), Symbols/Numbers (shift), Emoji panel
- **Shape Writing**: Continuous swipe → word prediction (SWIPE library)
- **Cursor Control**: Long press text → magnifier + draggable cursor
- **Text Prediction**: Above keyboard, tap to insert, learns user vocabulary
- **Languages**: 50+ languages, quick switch via spacebar long press

### Lock Screen
- **Clock**: Large typographic time (Segoe UI Light, ~48pt), date below
- **Background**: User image, Windows Spotlight (Bing daily), slideshow
- **Notifications**: Configurable privacy (hide content, show count only)
- **Quick Actions**: Camera (swipe up from bottom-right), Flashlight
- **Auth**: PIN (4+ digits), Password, Windows Hello (face/iris/fingerprint — HW dependent)

---

## Accent Colors (21 System Colors — Hex Values Públicos)

| Nome | Hex | Uso Conexus |
|------|-----|-------------|
| Lime | #A4C400 | Disponível |
| Green | #60A917 | Disponível |
| Emerald | #008A00 | Disponível |
| Teal | #00ABA9 | Disponível |
| Cyan | #1BA1E2 | Disponível |
| Cobalt | #0050EF | Disponível |
| Indigo | #6A00FF | Disponível |
| Violet | #AA00FF | Disponível |
| Pink | #F472D0 | Disponível |
| Magenta | #D80073 | Disponível |
| Crimson | #A20025 | Disponível |
| Red | #E51400 | Disponível |
| Orange | #FA6800 | Disponível |
| Amber | #F0A30A | Disponível |
| Yellow | #E3C800 | Disponível |
| Brown | #825A2C | Disponível |
| Olive | #6D8764 | Disponível |
| Steel | #647687 | Disponível |
| Mauve | #76608A | Disponível |
| Taupe | #87794E | Disponível |
| **Conexus Brand** | **#0066CC** | **Primary accent (definir)** |

---

## Próximos Passos (M0/M1)

1. [ ] **A1**: Completar `SCREEN_CATALOG.md` com wireframes anotados por tela
2. [ ] **A1**: Preencher `BEHAVIOR_MATRIX.md` com critérios de equivalência por feature
3. [ ] **A1**: Documentar `INTERACTION_FLOWS.md` com fluxos mensuráveis
4. [ ] **A1**: Revisar `LEGAL_BOUNDARIES.md` com advogado IP
5. [ ] **A3**: Iniciar design tokens Fluent 2 em `shell/fluent/tokens/`
6. [ ] **A4**: Definir SDK contracts em `framework/sdk/contracts/`

---

## Checklist de Conformidade Clean Room

- [x] Nenhum binário descompilado/decompilado
- [x] Nenhum asset extraído de pacotes .appx/.msix
- [x] Nenhuma chave/assinatura Microsoft utilizada
- [x] Nenhum código fonte proprietário acessado
- [x] Fontes: apenas documentação pública, open source, jornalismo
- [x] Assets visuais: apenas screenshots públicos + wireframes próprios
- [x] Tipografia: Inter/Noto Sans (OFL) — não Segoe UI
- [x] Ícones: Fluent System Icons (MIT) — não Segoe MDL2
- [x] Nomes: "Dynamic Tiles", "Notification Center" — não "Live Tiles®", "Action Center®"

---

## Referências

- [BetaWiki: Windows 10 Mobile build 15254](https://betawiki.net/wiki/Windows_10_Mobile_build_15254)
- [LumiaWOA Project](https://github.com/WOA-Project/LumiaWOA)
- [Lumia950XLPkg (UEFI)](https://github.com/WOA-Project/Lumia950XLPkg)
- [Fluent 2 Design Tokens](https://github.com/microsoft/fluentui/tree/master/packages/design-tokens)
- [Fluent System Icons (MIT)](https://github.com/microsoft/fluentui-system-icons)
- [Microsoft Learn: UWP App Architecture](https://learn.microsoft.com/en-us/windows/uwp/)