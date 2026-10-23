# ADR-011: Famílias de marca como matriz, não um APK por marca — **PROPOSTO**

**Status:** Proposto, parcialmente substituído pelo ADR-014 (APK por marca como contêiner, carregado por serviço de sistema). A matriz por marca e modelo continua valendo.
**Data:** 2026-10-21
**Decisores:** Tech Lead (A0), Security (A5), Platform (A2)
**Assinatura:** pendente

---

## Contexto

O pedido foi: "todos Samsung, todos OPPO, todos Xiaomi, todos POCO, um APK por marca". Ele junta duas ideias que não se sustentam:

1. **"Todos" de uma marca.** Samsung, Xiaomi, OPPO e POCO têm centenas de modelos com SoCs, kernels e políticas de bootloader diferentes. Não é possível declarar suporte geral sem matriz verificada por modelo (mesmo caminho do ADR-010).
2. **Um APK com drivers por marca.** Isso reproduz o problema do ADR-009: um APK que carrega driver com privilégio de kernel, sem assinatura do projeto, e mistura blobs proprietários de dezenas de aparelhos num único pacote. Também fere a regra de licença de vendor blobs (A5).

POCO é submarca da Xiaomi. Tratá-la como marca separada duplicaria a matriz.

---

## Decisão proposta

**Não criar APK por marca com drivers.** O modelo é:

1. **Matriz por marca, por modelo:** uma matriz por marca (Samsung, OPPO, Xiaomi com POCO incluída), com os mesmos critérios de entrada do ADR-010 (C1 desbloqueio oficial, C2 kernel GPL, C3 device tree, C4 compatibilidade com ADR-009, C5 recovery testado). Nenhum modelo é marcado como suportado sem verificação.
2. **Pacote de driver por componente e por device:** entra pelo caminho do ADR-009 (userspace preferido, assinado com a chave de driver). Os blobs vêm do aparelho do próprio usuário, fora do repo.
3. **Um app de gerenciamento só (`conexus-driver-manager`):** instala e atualiza pacotes de driver, valida assinatura e manifest. Não é um APK por marca, e não guarda blob proprietário.

---

## Consequências

### Positivas
- Um único caminho de instalação, validado uma vez, em todas as marcas.
- Nenhum blob proprietário é distribuído pelo projeto.
- A matriz mostra o que é verificado e o que é só candidato.

### Negativas
- Suporte por marca fica mais lento do que um pacote único faria, porque cada modelo precisa de verificação.
- O usuário precisa extrair os blobs do próprio aparelho para os componentes proprietários.

### Riscos
- **R-SCALE:** centenas de modelos tornam a matriz grande. Mitigação: começar por um modelo por marca, com ADR de device próprio.
- **R-LIC:** blobs de fabricante com licença restritiva. Mitigação: A5 revisa cada blob antes de uso, sem redistribuição.

---

## Alternativas consideradas

| Alternativa | Por que não |
|---|---|
| Um APK por marca com drivers embutidos | Privilégio de kernel sem assinatura do projeto; blobs redistribuídos; escala ruim |
| Suporte declarado a "todos" da marca | Sem base verificada; contradiz o ADR-010 |
| POCO como marca separada | Duplicaria a matriz da Xiaomi |

---

## Plano

1. **A2:** levantar, por marca, os modelos candidatos nas fontes oficiais (site do fabricante, LineageOS wiki) com data e fonte. Começar por um modelo por marca.
2. **A2:** verificar C1 e C2 de cada candidato.
3. **A0:** escolher no máximo um segundo device, com ADR de device próprio e nova rodada de M0.
4. **A4:** definir o formato do pacote de driver em `framework/sdk/shared/`, junto com o manifest do ADR-009.

---

## Referências

- ADR-009 (core único, drivers como pacotes)
- ADR-010 (família Moto G, matriz por modelo)
- `docs/architecture/DEVICE_SUPPORT.md`
