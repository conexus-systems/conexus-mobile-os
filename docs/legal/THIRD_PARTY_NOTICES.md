# Third Party Notices — Conexus Harmattan

**Projeto:** Conexus Harmattan  
**Data:** 22 de outubro de 2026  
**Versão:** 0.1

---

## Referências Históricas (Não Dependências Runtime)

Os seguintes projetos foram estudados como referências comportamentais, visuais e arquiteturais. **Nenhum código foi copiado** para o runtime final. Todas as implementações são originais em Rust.

### MeeGo Harmattan / Nokia N9 (Histórico)

| Projeto | Organização | Licença | Uso |
|---------|-------------|---------|-----|
| Kernel Plus Harmattan | `harmattan/kernel-plus-harmattan` | GPL-2.0 | Referência kernel patches OMAP3 |
| Profilematic | `harmattan/profilematic` | BSD-3-Clause | Referência comportamental: regras de perfil contextuais |
| SyncUI | `harmattan/syncui` | LGPL-2.1 | Referência UI: sincronização de contas |
| Accounts UI | `harmattan/accounts-ui` | LGPL-2.1 | Referência UI: gerenciamento de contas |

### Projetos Comunitários (ajalkane)

| Projeto | Autor | Licença | Uso |
|---------|-------|---------|-----|
| Profilematic (fork) | Jussi A. | BSD-3-Clause | Referência comportamental: regras de perfil |
| ToggleAvailability | Jussi A. | BSD-3-Clause | Referência UI: quick toggles (wifi, bt, rotation) |
| MeeGo Keyboard | Jussi A. | BSD-3-Clause | Referência UI: teclado virtual, shape writing |

### Nemo Mobile (Ativo)

| Projeto | Organização | Licença | Uso |
|---------|-----------|---------|-----|
| Qt Components | `nemomobile/qt-components` | **LGPL-2.1** | **Referência API/Design de componentes** — Button, Switch, PageStack, TabBar, GridView, Theme |
| Glacier Home | `nemomobile/glacier-home` | LGPL-2.1 | **Referência UX principal** — Home grid, widgets, quick settings, gestos, navegação |
| Lipstick | `nemomobile/lipstick` | LGPL-2.1 | Referência arquitetural: separação compositor/shell |
| QtQuickControls Nemo | `nemomobile/qtquickcontrols-nemo` | LGPL-2.1 | Referência componentes: controles com tema Nemo |

### Outros

| Projeto | Autor | Licença | Uso |
|---------|-------|---------|-----|
| MoeDict MeeGo | rschiang | GPL-3.0 | Referência app structure: dicionário offline |

---

## Dependências Runtime (Código Real no Binário)

### Rust Crates (Workspace)

| Crate | Versão | Licença | Fonte |
|-------|--------|---------|-------|
| `serde` | 1.x | MIT OR Apache-2.0 | crates.io |
| `serde_json` | 1.x | MIT OR Apache-2.0 | crates.io |
| `serde_derive` | 1.x | MIT OR Apache-2.0 | crates.io |
| `tokio` | 1.x | MIT | crates.io |
| `tracing` | 0.1.x | MIT | crates.io |
| `thiserror` | 1.x | MIT OR Apache-2.0 | crates.io |
| `async-trait` | 0.1.x | MIT OR Apache-2.0 | crates.io |
| `zbus` | 4.x | MIT | crates.io |
| `notify-rust` | 4.x | MIT | crates.io |
| `desktop-entry` | 0.3.x | MIT OR Apache-2.0 | crates.io |
| `xdg` | 2.x | MIT OR Apache-2.0 | crates.io |
| `dirs` | 5.x | MIT OR Apache-2.0 | crates.io |
| `sysinfo` | 0.30.x | MIT | crates.io |
| `jni` | 0.21.x | MIT OR Apache-2.0 | crates.io |
| `ndk-glue` | 0.9.x | MIT OR Apache-2.0 | crates.io |
| `ndk-sys` | 0.8.x | MIT OR Apache-2.0 | crates.io |

### UI Framework (Pendente ADR-008)

| Framework | Licença | Status |
|-----------|---------|--------|
| **Slint** | MIT (open source) / Commercial | Candidato (PoC Fase 1) |
| **Iced** | MIT | Candidato (PoC Fase 1) |
| **egui** | MIT | Candidato (PoC Fase 1) |

### Go (Instalador)

| Package | Licença |
|---------|---------|
| stdlib | BSD-3-Clause |
| github.com/google/uuid | BSD-3-Clause |
| golang.org/x/sys | BSD-3-Clause |

### Kotlin/Gradle (Driver Core - Android)

| Package | Licença |
|---------|---------|
| kotlinx.coroutines | Apache-2.0 |
| kotlinx.serialization | Apache-2.0 |
| AndroidX libraries | Apache-2.0 |
| Android SDK | Apache-2.0 |

### Node.js (Site/Documentação)

| Package | Licença |
|---------|---------|
| turbo | MIT |
| typescript | Apache-2.0 |
| @types/node | MIT |
| @types/react | MIT |

---

## Licença do Projeto

**Conexus Harmattan** é licenciado sob **Apache-2.0**.

```
Copyright 2026 Conexus Systems

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
```

---

## Atribuições Obrigatórias (LGPL-2.1)

Para referências LGPL-2.1 (`nemomobile/*`, `harmattan/syncui`, `harmattan/accounts-ui`):

1. **Não há linking dinâmico** no binário final — apenas referências de design
2. **Código fonte disponível** em: https://github.com/nemomobile, https://github.com/harmattan
3. **Modificações:** Nenhuma — não há código copiado
4. **Licença completa:** Disponível em https://www.gnu.org/licenses/lgpl-2.1.html

---

## Marcas Registradas

- "MeeGo", "Harmattan", "N9" — marcas da Nokia / Microsoft
- "Nemo Mobile" — marca da Nemo Mobile Community
- "Qt" — marca da The Qt Company
- "Android" — marca da Google LLC
- "Linux" — marca registrada de Linus Torvalds
- **Conexus**, **Conexus OS**, **Conexus Launcher** — marcas da Conexus Systems

**Uso:** Referências históricas usam nomes originais apenas para contexto factual. Produto final usa marca **Conexus** exclusivamente.

---

## Contato

Para questões sobre licenciamento: legal@conexus-systems.com

---

**Gerado automaticamente em:** 2026-10-22  
**Versão:** 0.1  
**Próxima atualização:** Após ADR-008 (framework UI selecionado)