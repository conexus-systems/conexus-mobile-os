# ADR-005: Recovery Obrigatório Antes de Flash (Hard Gate) — **APROVADO**

**Status:** Aceito
**Data:** 2026-10-19
**Decisores:** Tech Lead (A0), Architect, Platform (A2), Security (A5), Release (A7)
**Assinatura:** A0 ✅

---

## Contexto

Instalar um sistema operacional customizado em dispositivo físico envolve riscos:
- Brick permanente (hardware inutilizado)
- Perda de dados do usuário
- Perda de garantia (bootloader unlock)
- Impossibilidade de voltar ao stock sem procedimento documentado
- Dispositivos de teste limitados (não temos device farm)

Política CONEXUS existente: "Recuperação testada antes de distribuir cada imagem" (SDD §10).

---

## Decisão

**Nenhuma imagem Conexus Mobile OS será flashada em dispositivo físico sem:**

1. **Procedimento de recovery documentado e testado** para aquele device exato
2. **Imagem stock original obtida e validada** (hash verificado, flash testado)
3. **Aprovação humana por escrito** (Tiago) para aquele device específico
4. **Device de backup disponível** (mesmo modelo) para caso de brick

**Aplicação:**
- A2 (Platform) é o único agente autorizado a executar flash
- Flash só ocorre após gate M2 (Boot + Recovery comprovados)
- Nightly/Alpha/Beta images: recovery procedure incluso no release package
- **Zero exceções** — nem para "teste rápido"

---

## Consequências

### Positivas
- Elimina risco de brick irreversível em devices de desenvolvimento
- Força documentação de recovery desde o início
- Cria cultura de segurança hardware-first
- Alinha com práticas de firmware engineering profissional

### Negativas
- Atrasa primeiros testes em hardware real (precisa recovery primeiro)
- Requer device de backup (custo extra)
- Processo burocrático para cada flash

### Riscos
- **Processo vira "teatro":** recovery documentado mas não testado → Mitigação: A2 deve demonstrar recovery funcionando no gate M2
- **Device único falha:** sem backup → Mitigação: Comprar 2 units do device alvo no M0 ✅ **DECIDIDO**

---

## Alternativas Consideradas

| Alternativa | Prós | Contras | Por que não |
|-------------|------|---------|-------------|
| Flash sem recovery, "a gente resolve se der errado" | Rápido para iterar | Brick = device perdido, dados perdidos, tempo perdido | Inaceitável profissionalmente |
| Recovery apenas documentado, não testado | Menos trabalho | Não garante que funciona quando precisa | Igual a não ter |
| Apenas emulador até M4 | Zero risco hardware | Não testa drivers reais, touch, modem, sensors, power | Hardware real essencial M2+ |

---

## Plano de Implementação

1. **M0 (Device Selection):** Comprar 2 units do device Android alvo ✅ **Pixel 8a (2 units)**
2. **M0:** Obter factory image stock, validar hash, testar flash stock → boot → flash stock (recovery test)
3. **M0:** Documentar procedimento em `docs/installation/ANDROID_INSTALL.md`
4. **M2 Gate:** A2 demonstra: flash Conexus → boot → interrupt update → recovery → boot stock
5. **Contínuo:** Todo novo device = mesmo processo. Todo release = recovery testado.

---

## Referências

- [SDD v0.1 §10, §9](./../../SDD.md)
- [Android Flash Tool](https://flash.android.com/)
- [CONEXUS deploy safety rules](../../_wt-agents-aucta/docs/factory/DEPLOY.md)