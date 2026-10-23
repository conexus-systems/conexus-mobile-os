# ADR-010: Família Moto G como alvos de dispositivo — **PROPOSTO**

**Status:** Proposto (não aprovado; não altera o primário M0)
**Data:** 2026-10-21
**Decisores:** Tech Lead (A0), Platform (A2), Security (A5)
**Assinatura:** pendente

---

## Contexto

O pedido foi suportar "todos os Moto G". A família tem dezenas de modelos, com SoCs, kernels, partições e políticas de bootloader muito diferentes. Não é um alvo único.

Restrições já decididas no projeto:

- **ADR-001 / M0:** primário Pixel 8a (`akita`), backup Nothing Phone (2). Um Moto G é um device novo e exige nova decisão de M0.
- **ADR-005:** recovery comprovado antes de qualquer flash.
- **ADR-009:** modelo de drivers (core único, pacotes assinados). Vale integralmente só para devices com **kernel GKI** (`vendor_dlkm`, KMI). Moto G antigos rodam kernels 3.x/4.x sem GKI; para eles o pacote de driver precisa de outro caminho.
- **AGENTS.md (regra 6):** nenhum agente faz flash em aparelho sem aprovação escrita.

Pesquisa feita nesta data (resultados fracos, verificar antes de usar):

- LineageOS tem páginas de instalação para modelos Moto G antigos (titan, falcon, peregrine, thea, athene, osprey) e para o Moto G 5G 2024 (fogo). As páginas que apareceram citam LineageOS 14.1, então podem estar desatualizadas. Fonte: wiki.lineageos.org.
- O LineageOS 17.1 incluiu Moto G (2015), Moto G4 Play e outros. Fonte: xda-developers.com.
- Não encontrei lista oficial atual de elegibilidade para desbloqueio de bootloader dos Moto G. Um relato de Moto G Power 2021 recusado não é confirmável. A fonte oficial é a página de desbloqueio da Motorola, consultada por modelo e região.
- Não encontrei informação confiável sobre publicação de fonte do kernel (GPL) por modelo.

---

## Decisão proposta

**Tratar "Moto G" como família com matriz por modelo, não como um alvo.** Cada modelo entra na matriz só com dados verificados nas fontes oficiais ou do LineageOS, e recebe um estado. Nenhum modelo é marcado como suportado antes de ter todos os critérios abaixo.

### Critérios de entrada por modelo

| # | Critério | Fonte aceita |
|---|---|---|
| C1 | Bootloader desbloqueável oficialmente, sem espera ou recusa para o modelo e a região | Página oficial de desbloqueio da Motorola, com o número do modelo |
| C2 | Fonte do kernel GPL publicada para o modelo exato | Repositório oficial da Motorola ou do LineageOS |
| C3 | Árvore de device e de vendor no LineageOS, na versão alvo | Repositório do LineageOS |
| C4 | Kernel compatível com o modelo de driver do ADR-009 (GKI ou mainline ≥ 5.10) | Versão do kernel do device |
| C5 | Partições A/B ou procedimento de recovery testado (ADR-005) | Teste em hardware, sem flash sem aprovação |

Modelos que não atendem C4 entram numa trilha separada, "legado sem GKI", com driver por pacote de userspace apenas, sem módulo de kernel.

### Matriz (estado inicial)

| Geração (exemplos verificados) | Modelos | C1 | C2 | C3 | C4 | Estado |
|---|---|---|---|---|---|---|
| Moto G 2014–2015 (titan, falcon, peregrine, thea, osprey) | a levantar | a verificar | a verificar | ✅ páginas no wiki (versões antigas) | ❌ kernel pré-GKI | **Legado — não suportado** |
| Moto G4 (athene) | a levantar | a verificar | a verificar | ✅ página no wiki | ❌ pré-GKI | **Legado — não suportado** |
| Moto G 5G 2024 (fogo) | a levantar | a verificar | a verificar | ✅ página no wiki | a verificar | **Candidato a verificar** |
| Demais gerações (2016–2026) | a levantar | a verificar | a verificar | a verificar | a verificar | **Não verificado** |

"A levantar" significa que a lista completa de modelos ainda não foi feita. Ela deve sair do site oficial da Motorola e do LineageOS, não de memória.

---

## Consequências

### Positivas
- Evita prometer suporte a "todos" sem base.
- Reaproveita o core único e o modelo de drivers para os devices GKI da família.
- Deixa explícito o que fica fora (legado pré-GKI).

### Negativas
- Os Moto G antigos, que são os mais fáceis de desbloquear, ficam de fora da trilha principal.
- A matriz exige pesquisa manual por modelo, com consulta a fontes oficiais.

### Riscos
- **R-BL:** a política de desbloqueio muda por região e por modelo. Mitigação: C1 é verificado no dia da decisão, por modelo, e registrado com data.
- **R-KERNEL:** sem fonte GPL, não há como compilar kernel nem módulo. Mitigação: C2 é critério de entrada obrigatório.
- **R-WIPE:** desbloquear apaga os dados. Mitigação: aviso e backup no `ANDROID_INSTALL.md`.

---

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Declarar "todos os Moto G" suportados | Não há base verificada; prometer compatibilidade universal está fora do escopo do SDD |
| Escolher um Moto G específico agora | Antes é preciso o M0 do device e a aprovação do ADR |
| Suportar só Moto G pré-GKI | Exige driver fora do ADR-009; contradiz o core único |

---

## Plano

1. **A2:** levantar a lista completa de modelos Moto G no site oficial da Motorola e no LineageOS, com data e fonte.
2. **A2:** para cada modelo, verificar C1 e C2 nas fontes oficiais e registrar na matriz.
3. **A0:** decidir se algum modelo entra como segundo device, com ADR de device próprio e nova rodada de M0.
4. **A5:** revisar a licença dos blobs de vendor de qualquer modelo antes de usar.

---

## Referências

- ADR-001, ADR-005, ADR-009
- `docs/architecture/DEVICE_SUPPORT.md`
- wiki.lineageos.org (páginas de devices Motorola)
- xda-developers.com (LineageOS 17.1 e Motorola)
