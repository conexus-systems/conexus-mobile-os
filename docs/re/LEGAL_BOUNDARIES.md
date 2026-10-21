# Legal Boundaries — Reverse Engineering Windows 10 Mobile

**Versão:** v0.1  
**Data:** 19 de outubro de 2026  
**Status:** Requer revisão legal antes de M0 Gate

---

## Princípios Orientadores

> **Objetivo:** Construir especificação comportamental do Windows 10 Mobile para reimplementação independente.
> **Não objetivo:** Copiar código, binários, assets proprietários, ou contornar proteções da Microsoft.

### O que É Permitido (Clean Room / Observação Comportamental)

| Atividade | Base Legal | Procedimento |
|-----------|------------|--------------|
| **Observar comportamento** de sistema legítimo rodando em hardware próprio | Uso legítimo de software licenciado (EULA permite uso) | Documentar inputs → outputs, transições, timings, estados |
| **Inspecionar recursos acessíveis** (strings, layouts, manifestos em formato aberto) | Interoperabilidade / Engenharia reversa para compatibilidade (DMCA 1201(f), EU Directive 2009/24) | Apenas formatos não-criptografados, não-ofuscados intencionalmente |
| **Consultar documentação pública** (Microsoft Learn, BetaWiki, community docs) | Domínio público / Licença da documentação | Citar fontes, respeitar termos de uso |
| **Analisar código aberto** (LumiaWOA, drivers Linux, UEFI implementations) | Licenças open source (MIT, GPL, etc.) | Respeitar licenças originais |
| **Criar implementação original** baseada em especificação comportamental | Direito autoral não protege ideias/funcionalidade | Código, assets, nomes, ícones 100% originais |
| **Usar nomes descritivos genéricos** (ex: "Live Tile", "Action Center") | Nomes funcionais não protegidos | Evitar branding Microsoft (Windows, Microsoft, Lumia) |

### O que NÃO É Permitido

| Atividade | Por Que | Risco |
|-----------|---------|-------|
| **Descompilar/decompilar binários** proprietários (.exe, .dll, .appx) | Viola EULA, DMCA 1201(a), copyright | Processo, injunção, danos |
| **Extrair assets** (ícones, imagens, sons, fontes) de pacotes .appx/.wim | Copyright nos assets | Injunção, danos estatutários |
| **Copiar código** fonte ou lógica de implementação | Copyright no código | Injunção, danos |
| **Contornar proteções** (criptografia, assinatura, DRM, secure boot) | DMCA 1201(a)(1), anti-circumvention | Crime federal (EUA), sanções civis |
| **Reutilizar chaves/assinaturas** Microsoft | Fraude, trademark, segurança | Crime, revogação de certificados |
| **Distribuir firmware/imagens** Microsoft | Copyright, distribuição não autorizada | Injunção, danos |
| **Usar marcas registradas** (Windows, Microsoft, Lumia, Live Tiles®) | Trademark law | Confusão de marca, dilution |

---

## Metodologia Clean Room (Two-Team)

Para máxima proteção legal, recomenda-se separação física/lógica:

### Team A — Specification Writers (A1 - Research)
- **Acesso:** Sistema Windows 10 Mobile rodando em hardware
- **Atividade:** Observar, documentar comportamentos, criar matrizes
- **Saída:** `BEHAVIOR_MATRIX.md`, `SCREEN_CATALOG.md`, `INTERACTION_FLOWS.md` — **apenas especificação funcional**
- **Restrição:** **NÃO** escreve código de implementação

### Team B — Implementers (A3, A4, A2)
- **Acesso:** **APENAS** especificações do Team A + documentação pública
- **Atividade:** Implementar do zero baseando-se nas specs
- **Restrição:** **NÃO** acessa sistema Windows 10 Mobile, binários, ou assets
- **Evidência:** Logs de desenvolvimento, commits, code review mostram originação independente

> **Nota:** Em contexto de agente IA único, a separação é lógica: prompts de research não geram código; prompts de implementação não acessam system reference.

---

## Análise de Recursos Específicos

| Recurso Windows 10 Mobile | Status Legal | Ação Conexus |
|---------------------------|--------------|--------------|
| **Nome "Live Tiles"** | Trademark Microsoft (provavelmente) | Usar "Dynamic Tiles" ou "Live Tiles" como termo descritivo genérico; não como brand |
| **Nome "Action Center"** | Trademark Microsoft | Usar "Notification Center" ou "Quick Actions Panel" |
| **Nome "Continuum"** | Trademark Microsoft | Usar "Desktop Mode" ou "External Display Mode" |
| **Segoe UI Font** | Copyright Microsoft, não open source | **Não usar**. Substituir por **Inter** (OFL) ou **Noto Sans** (OFL) |
| **System Icons** | Copyright Microsoft | **Não usar**. Usar **Fluent System Icons** (MIT) ou ícones próprios |
| **System Sounds** | Copyright Microsoft | **Não usar**. Criar próprios ou usar CC0 |
| **Acrylic/Mica Shader Code** | Trade secret / Copyright | Reimplementar do zero baseado em paper público (Microsoft Research) |
| **Keyboard Layout (pt-BR)** | Layout funcional não protegido | Recriar layout idêntico funcionalmente; assets próprios |
| **Gesture Patterns** (edge swipes) | Funcionalidade não protegida | Reimplementar; patentes expiram 2030+ |
| **Tile XML Schema** | Formato funcional | Recriar schema equivalente; não copiar namespace Microsoft |
| **AppxManifest Structure** | Formato padrão (Open Packaging Conventions) | Usar padrão OPC; namespace próprio |

---

## Fontes de Referência Autorizadas

| Fonte | Tipo | Uso Permitido |
|-------|------|---------------|
| **Microsoft Learn / Docs** | Documentação pública | Referência de APIs públicas, conceitos, guidelines |
| **BetaWiki / Windows Central / Thurrott** | Jornalismo/Community | Comportamento documentado, screenshots públicos |
| **LumiaWOA GitHub** | Open source (MIT/Apache) | Código UEFI, drivers Linux, boot process — **sob licença** |
| **WOA Project Drivers** | Open source | Drivers Linux para hardware Lumia — **sob licença** |
| **Fluent Design System (Microsoft)** | Open source (MIT) | Design tokens, components, icons — **sob licença MIT** |
| **Windows UI Library (WinUI)** | Open source (MIT) | Componentes, patterns — **sob licença MIT** |

---

## Checklist de Conformidade (Por Entregável)

### Para cada documento em `docs/re/`:
- [ ] Nenhum código proprietário copiado
- [ ] Nenhum asset (imagem, ícone, som, fonte) extraído de binários Microsoft
- [ ] Nomes Microsoft usados apenas como referência descritiva ("comportamento similar a Windows 10 Mobile")
- [ ] Fontes citadas com URLs e data de acesso
- [ ] Assets anexados são: criados por nós, domínio público, ou licença compatível (MIT, Apache-2.0, CC0, OFL)

### Para cada código em `shell/`, `framework/`, `services/`:
- [ ] Código 100% original (escrito por agentes/desenvolvedores Conexus)
- [ ] Nenhuma string literal copiada de binários Microsoft
- [ ] Nomes de classes/métodos próprios (ex: `ConexusTileEngine`, não `WindowsTileEngine`)
- [ ] Namespaces próprios (`com.conexus.*`, `@conexus-mobile-os/*`)
- [ ] Licença no header (Apache-2.0 ou MIT)

### Para assets em `shell/fluent/`, `framework/sdk/*/theme/`:
- [ ] Ícones: Fluent System Icons (MIT) ou próprios
- [ ] Fontes: Inter (OFL) / Noto Sans (OFL) / próprias
- [ ] Cores: Paleta própria inspirada, não copiada
- [ ] Shaders: Implementação própria baseada em papers públicos

---

## Processo de Revisão Legal (M0 Gate)

1. **A1** completa `REFERENCE_INVENTORY.md`, `BEHAVIOR_MATRIX.md` (rascunho)
2. **A0** submete para revisão legal (advogado especializado em IP/software)
3. **Legal** emite parecer: "Clean room methodology adequada" ou "Riscos identificados: [lista]"
4. **A0** documenta parecer em `docs/legal/LEGAL_OPINION_YYYYMMDD.md` (confidencial)
5. **M0 Gate** só passa com parecer favorável ou mitigações aceitas

---

## Registro de Decisões (Atualizar Conforme Projeto)

| Data | Decisão | Fundamento | Aprovado Por |
|------|---------|------------|--------------|
| 2026-10-19 | Adotar metodologia clean room two-team (lógica) | Minimizar risco copyright/trade secret | A0 (pendente legal) |
| 2026-10-19 | Não usar Segoe UI; usar Inter/Noto Sans | Font copyright Microsoft | A0 |
| 2026-10-19 | Não usar system icons Microsoft; usar Fluent Icons (MIT) | Icon copyright Microsoft | A0 |
| 2026-10-19 | Termos descritivos: "Dynamic Tiles", "Notification Center" | Evitar trademark Microsoft | A0 (pendente legal) |
| 2026-10-19 | Namespace próprio: `com.conexus.*`, `@conexus-mobile-os/*` | Identidade própria, evitar passing off | A0 |

---

## Contato Legal

- **Advogado IP/Software:** [Nome/Contato — a definir]
- **Jurisdição Principal:** Brasil (LGPD, Lei 9.609/98, Lei 9.279/96)
- **Jurisdição Secundária:** EUA (DMCA, Copyright Act, Lanham Act) — se distribuição global

---

**Próxima Ação:** Agendar revisão legal antes de M0 Gate (semana 2).