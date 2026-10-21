# Conexus Mobile OS — CORE

Princípios, decisões de base, riscos e Definition of Done. Este arquivo é referenciado por `README.md`, `AGENTS.md` e `M0_ACTION_CHECKLIST.md`.

## Objetivo

Recriar a experiência de uso do Windows 10 Mobile com identidade Conexus (Live Tiles, tipografia expressiva, navegação rápida, Action Center) sobre uma base moderna e aberta, como sistema operacional completo e não como launcher. Não reutilizar código, firmware, binários, assets ou chaves proprietários da Microsoft ou da Nokia.

## Princípios

1. **Clean room:** pesquisa (observação de comportamento, `docs/re/`) separada da implementação (código original). Ver `docs/re/LEGAL_BOUNDARIES.md`.
2. **Local-first:** nenhum recurso básico depende de login, rede, anúncio, doação ou API de IA.
3. **Permissões especiais só com consentimento explícito.** Nenhuma concessão automática.
4. **Conteúdo do usuário não sai do aparelho por padrão.** Notificações, e-mail, uso e texto digitado não vão a serviços remotos sem ação explícita.
5. **Não prometer desempenho, bateria ou compatibilidade sem medição.** Emulador não vale como aparelho.
6. **Android e Lumia são trilhas técnicas separadas** (ADR-002, ADR-003). Interfaces compartilhadas acima; kernel, drivers e boot específicos por alvo.
7. **Licenças, avisos e hashes de assets preservados.** Dependências só com licença aprovada (`docs/legal/THIRD_PARTY_NOTICES.md`).
8. **Toda mudança tem critério de aceite e registro do que foi realmente testado.** Não marcar como feito sem evidência.
9. **Nenhuma operação destrutiva em aparelho sem aprovação humana por escrito** (ADR-005, `AGENTS.md` regra 6).
10. **Nenhum segredo no repositório.** Chaves de produção só com o responsável do projeto.

## Estado atual

Ver `README.md` ("Status Atual") e `M0_ACTION_CHECKLIST.md`. Fase: M0, viabilidade e inventário.

## ADRs baseline

| ADR | Decisão |
|-----|---------|
| 0001 | Base do produto Android (ver os dois arquivos `0001-*`; consolidação pendente de decisão do A0) |
| 0002 | Lumia como laboratório de pesquisa, não produto |
| 0003 | Monorepo com builds isolados por plataforma |
| 0004 | SDK próprio desde o MVP |
| 0005 | Recovery comprovado antes de qualquer flash |
| 0006 | Fluent/Metro como linguagem visual |
| 0007 | Serviços de sistema isolados (Binder/D-Bus) |
| 0008 | OTA com partições A/B ou fallback por recovery |

ADRs 0009 em diante estão propostos em branches `feat/*` e entram aqui quando aprovados.

## Risk register

| ID | Risco | Mitigação | Estado |
|----|-------|-----------|--------|
| R01 | Device sem bootloader desbloqueável | Pixel 8a com unlock oficial | mitigado na escolha; a confirmar em hardware |
| R02 | Drivers proprietários indisponíveis | Kernel GKI 6.1 mainline no Pixel 8a | mitigado na escolha; a confirmar |
| R03 | Quebra do upstream LineageOS | Tag `lineage-23.2` fixada, vendor isolado, CI nightly | aberto |
| R04 | Lumia sem telefonia/emergência confiável | Documentar como limitação; não prometer | aberto (ADR-002) |
| R06 | A definir — citado em `ROADMAP.md` e `M0_ACTION_CHECKLIST.md` sem definição; confirmar com A0 (provável: revisão legal/IP pendente) | — | a definir |
| R07 | Engenharia desviada para o Lumia | M5 só após o gate M2 | aberto |

## Definition of Done

- Mudança compilada com o toolchain declarado no repositório.
- Testes relevantes executados e resultados anexados. Resultados antigos não valem como prova de mudanças novas.
- Teste manual em aparelho quando o requisito envolve hardware ou UI, com registro na tabela de evidência de `docs/architecture/DEVICE_SUPPORT.md`.
- Documentação, riscos e matriz de compatibilidade atualizados.
- Sem segredos, chaves privadas ou artefatos proprietários não licenciados.
