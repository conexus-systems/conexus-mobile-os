# ADR-006: Repositórios Históricos como Referência

**Status:** Aceito  
**Data:** 2026-10-22  
**Decisores:** H0 (Orchestrator), H1 (Architect), H5 (UX/UI)

---

## Contexto

Existem repositórios públicos com código do MeeGo Harmattan (N9), Nemo Mobile, e projetos comunitários relacionados. O projeto Conexus Harmattan deve decidir como usar esses recursos sem violar licenças ou criar dependências indesejadas.

## Decisão

**Auditar licenças e compatibilidade antes de reutilizar qualquer código.** Repositórios históricos servem como referências de comportamento, UI e integração — **não como dependências automáticas do runtime final**.

## Consequências

### Positivas
- **Legal compliance:** Zero risco de violação de GPL/LGPL/patentes
- **Clean room:** Implementação própria garante ownership total
- **Modern stack:** Rust + Slint/Iced/egui em vez de Qt/QML legado
- **Ownership:** Conexus controla roadmap, não depende de upstreams abandonados

### Negativas
- **Esforço de reimplementação:** Componentes UI, profile logic, tile engine reescritos
- **Tempo:** Referências aceleram design, mas código próprio leva tempo

## Classificação de Reutilização

| Categoria | Ação | Exemplos |
|-----------|------|----------|
| **Referência Comportamental** | ✅ Estudar, documentar, reimplementar | `harmattan/profilematic` (regras de perfil), `glacier-home` (gestos, grid) |
| **Referência Visual/UX** | ✅ Inspiração, design tokens | `nemomobile/qt-components` (component API), `glacier-home` (layout) |
| **Referência Arquitetural** | ✅ Padrões, separação de concerns | `nemomobile/lipstick` (compositor/shell separation) |
| **Código Direto (Copy/Paste)** | ❌ **Proibido** | Qualquer `.qml`, `.cpp`, `.py` de repos GPL/LGPL |
| **Dependência Runtime** | ❌ **Proibido** | Linkar `qt-components`, `lipstick` no binário final |

## Análise de Licenças (Repositórios-Chave)

| Repositório | Licença | Pode Copiar? | Pode Linkar? | Uso Permitido |
|-------------|---------|--------------|--------------|---------------|
| `harmattan/kernel-plus-harmattan` | GPL-2.0 | ❌ | ❌ | Referência kernel patches apenas |
| `harmattan/profilematic` | BSD-3 | ✅ (com atribuição) | ✅ | Referência comportamental |
| `harmattan/syncui` | LGPL-2.1 | ⚠️ (dynamic link) | ⚠️ | Apenas referência |
| `nemomobile/qt-components` | **LGPL-2.1** | ⚠️ | ⚠️ (dynamic only) | **Referência API/Design — não copiar** |
| `nemomobile/glacier-home` | LGPL-2.1 | ⚠️ | ⚠️ | **Referência UX principal** |
| `nemomobile/lipstick` | LGPL-2.1 | ⚠️ | ⚠️ | Referência arquitetural |
| `ajalkane/*` | BSD-3 | ✅ | ✅ | Código livre, mas legacy Qt/QML |

## Estratégia por Componente

| Componente Harmattan | Fonte Referência | Implementação Conexus |
|----------------------|------------------|----------------------|
| **Profile Engine** | `harmattan/profilematic` + `ajalkane/profilematic` | `conexus-profiles` crate (Rust, rules engine) |
| **Tile Engine / Live Tiles** | `glacier-home` widgets | `conexus-core::tiles` (Rust, flip 3D, badge) |
| **Home Screen / App Grid** | `glacier-home` + `qt-components/GridView` | `conexus-ui` (Rust, Slint/Iced/egui) |
| **Action Center / Quick Settings** | `glacier-home` quick panel | `conexus-core::notifications` + UI |
| **Navigation / Pivots** | `qt-components/TabBar` + `PageStack` | `conexus-ui` NavigationController |
| **Switches / Toggles** | `qt-components/Switch` + `ajalkane/toggleavailability` | `conexus-ui` components |
| **Text Input / Keyboard** | `ajalkane/meego-keyboard` | `conexus-ui` + platform adapter |

## Clean Room Process

1. **Estudar** repositório referência → documentar comportamento em `docs/behavior-matrix.md`
2. **Especificar** requisitos em ADR/issue → critérios de aceitação mensuráveis
3. **Implementar** do zero em Rust → testes automatizados
4. **Validar** contra behavior matrix → conformidade comportamental
5. **Atribuir** referências em `THIRD_PARTY_NOTICES.md` e docs

## Validação

- [ ] `docs/repository-matrix.md` completa com licenças
- [ ] `docs/behavior-matrix.md` documenta comportamentos referenciados
- [ ] `THIRD_PARTY_NOTICES.md` inclui todas referências
- [ ] Zero código GPL/LGPL copiado no repositório
- [ ] Build reproduzível sem dependências históricas

---

**Assinatura:** H0, H1, H5  
**Próxima revisão:** Se novo repositório histórico for considerado