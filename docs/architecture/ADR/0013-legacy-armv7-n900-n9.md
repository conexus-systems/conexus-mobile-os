# ADR-013: Alvos legados ARMv7 — Nokia N900 e N9 — **PROPOSTO**

**Status:** Proposto (abre uma segunda família de base; não altera o primário M0)
**Data:** 2026-10-21
**Decisores:** Tech Lead (A0), Platform (A2), Security (A5)
**Assinatura:** pendente

---

## Contexto

O pedido foi instalar o sistema no Nokia N900 e no N9, com uma meta ambiciosa. Os dois são aparelhos Linux ARMv7 (Cortex-A8, TI OMAP3), sem Android moderno e fora do modelo de kernel do ADR-009.

Restrições reais:

- **N900:** 256 MB de RAM. O SDD fixa 150 MB para o processo do shell; com 256 MB, o sistema inteiro mal cabe. Não roda Android moderno nem GKI.
- **N9:** memória e kernel mais novos que o N900, mas a base de software é MeeGo/Nemo, não Android. Status do bootloader e publicação de fonte do kernel precisam de verificação.
- **ADR-001 e ADR-009** pressupõem AOSP/GKI. Para estes dois aparelhos, nenhum dos dois vale.
- **ADR-005 (recovery antes de flash):** o N900 permite testar sem tocar a memória interna. O NOLO carrega o U-Boot a partir do cartão SD, então um teste de boot do Conexus pode ser feito sem alterar o eMMC.

Pesquisa feita nesta data (resultados parciais; verificar antes de usar):

- A documentação do U-Boot para o N900 (RX-51) descreve o fluxo: NOLO carrega o U-Boot, que procura um kernel ou `boot.scr` no cartão SD ou no eMMC. A versão de referência é a v2021.10. Fonte: elixir.u-boot.org (README.nokia_rx51).
- O suporte a UBIFS está desabilitado nessa documentação, e a USB tty também.
- A base de software do N9 é Nemo Mobile sobre Mer, continuação do MeeGo CE. O status atual do bootloader do N9 não foi confirmado nas fontes que encontrei. Fonte: wiki.debian.org/Mobile/Nokia_N9.

---

## Decisão proposta

**Tratar N900 e N9 como uma segunda família de base, "Linux ARMv7 legado", com escopo reduzido.** Não é o Android com shell próprio. O escopo proposto:

| Fase | Entrega | Critério |
|---|---|---|
| 1 | **N900: boot de Linux a partir do SD**, sem tocar o eMMC | Linux inicia pelo cartão; NOLO intacto; remover o SD restaura o aparelho |
| 2 | **Core Rust cross-compilado para `armv7-unknown-linux-gnueabihf`** rodando no N900 | `conexus-core` (SDK e serviços) executa e passa os testes no aparelho |
| 3 | **Shell mínimo** (tiles e lista de apps) em UI leve, dentro de 256 MB | Medição de memória documentada; sem prometer o shell completo |
| 4 | **N9:** mesma sequência, após verificar bootloader e fonte do kernel | Critérios C1 e C2 do ADR-010, aplicados ao N9 |

A experiência completa do SDD (Live Tiles com animação, Action Center, Lock Screen) **não** é prometida para o N900. A meta é provar o core e um shell mínimo.

---

## Consequências

### Positivas
- Teste de boot sem risco: o eMMC original permanece intacto, o que é o cenário mais seguro para a regra do ADR-005.
- O core em Rust (ADR-012) já é pequeno o suficiente para caber em 256 MB.
- Serve de prova de portabilidade: se o core roda em ARMv7 de 256 MB, roda em Lumia e em Android.

### Negativas
- Cria uma segunda base de software, com manutenção própria. Isso aumenta a carga de A2 e A4.
- Não há Google Play, nem compatibilidade Android, nem recursos modernos (câmera, sensores, rede atual).

### Riscos
- **R-MEM:** 256 MB não sustentam o shell do SDD. Mitigação: meta de memória medida no aparelho e escopo reduzido.
- **R-BL (N9):** bootloader fechado ou sem método documentado. Mitigação: N9 fica na fase 4, só depois de C1 verificado.
- **R-KERNEL:** kernel do N900 é antigo e pouco mantido. Mitigação: usar o kernel da comunidade já documentado, sem patch manual oculto (critério A2).
- **R-DESTR:** qualquer escrita no eMMC é destrutiva. Mitigação: nenhum agente grava no eMMC sem aprovação escrita (AGENTS.md, regra 6).

---

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Android no N900 | 256 MB e sem kernel GKI; inviável |
| Shell completo do SDD no N900 | Excede a memória; prometer isso seria falso |
| Ignorar os aparelhos | Perde a prova de portabilidade do core, que é o ganho principal |

---

## Plano

1. **A2:** levantar, sem tocar o eMMC, o procedimento de boot do N900 a partir do SD, com a versão do U-Boot e do kernel da comunidade. Registrar fonte e data.
2. **A4:** adicionar o alvo `armv7-unknown-linux-gnueabihf` ao workspace e validar `cargo build` para esse alvo (cross-compilation sem hardware).
3. **A2:** medir a memória do core no aparelho quando houver N900 disponível.
4. **A5:** revisar a cadeia de boot do SD (sem assinatura, por ser o próprio NOLO) e os riscos de execução de código não assinado no aparelho.
5. **A0:** aprovar a segunda família de base e o escopo reduzido.

---

## Referências

- ADR-005 (recovery antes de flash), ADR-010 (critérios de device), ADR-012 (Rust)
- U-Boot, `doc/README.nokia_rx51` (elixir.u-boot.org, v2021.10)
- Debian Wiki, Mobile/Nokia_N9 (wiki.debian.org)
