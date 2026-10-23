# ADR-002: Lumia 950/950 XL como Laboratório Experimental — **APROVADO**

**Status:** Aceito
**Data:** 2026-10-19
**Decisores:** Tech Lead (A0), Architect, Platform (A2)
**Assinatura:** A0 ✅

---

## Contexto

O Windows 10 Mobile rodava nativamente em hardware Lumia (Snapdragon 808/810). O projeto LumiaWOA demonstrou:
- UEFI/EDK2 boot em Lumia 950/950 XL
- Windows ARM64 rodando nativamente
- Linux ARM64 boot possível (com limitações)
- Documentação de hardware, partitions, boot process

No entanto:
- Microsoft encerrou suporte em 2020
- Drivers proprietários (modem, câmera, GPU, audio) não abertos
- Telefonia e chamadas de emergência não confiáveis
- Hardware antigo (2015): 3GB RAM, 32GB storage, Snapdragon 808/810
- Bootloader unlock status varia por modelo/variante

---

## Decisão

**Manter Lumia 950/950 XL e Lumia 730/735 como alvos de pesquisa (laboratório), NÃO como produto.**

- **Lumia 950 / 950 XL (RM-1104 / RM-1105 / RM-1116):** alvo de pesquisa de boot e drivers. Há UEFI comunitário (Lumia950XLPkg).
- **Lumia 730 / 735 (RM-914 / RM-915):** alvo de pesquisa documental. Conforme `docs/lumia-730/TECHNICAL_ASSESSMENT.md`, não há bootloader desbloqueável nem port UEFI, então o Conexus Mobile OS não roda nele. A pesquisa cobre o que é possível (ex.: restauração e uso do sistema original, comparação de hardware) e registra por que a imagem não é viável.

- Objetivo: Entender boot ARM64, drivers, limitações de hardware, validar conceitos de baixo nível
- Escopo: UEFI boot → Linux kernel → Conexus Shell (se viável)
- Não prometer: Uso diário, telefonia funcional, câmera, emergência
- Timeline: Trilha paralela (M5), não bloqueia M0-M4 Android

---

## Consequências

### Positivas
- Acesso a hardware de referência do Windows 10 Mobile original
- Validação de conceitos: UEFI, ACPI, device trees ARM64
- Aprendizado aplicável a futuros devices ARM64
- Comunidade ativa (LumiaWOA, WOA Project) para troubleshooting

### Negativas
- Hardware obsoleto para uso real em 2026+
- Drivers proprietários = dead ends para peripherals críticos
- Tempo de engenharia desviado do produto principal
- Risco de "sunk cost" em plataforma sem futuro

### Riscos
- **R04:** Lumia sem telefonia/emergência → Mitigação: Documentar como limitação conhecida; não prometer
- **R07:** Engenharia bottleneck → Mitigação: M5 só inicia após M2 gate; 1 dev + IA agents

---

## Alternativas Consideradas

| Alternativa | Prós | Contras | Por que não |
|-------------|------|---------|-------------|
| Lumia como produto principal | Autêntico Windows Mobile hardware | Hardware antigo, drivers fechados, sem suporte, emergência não confiável | Risco alto para usuários reais |
| Outro device Windows Phone (930, 640, etc.) | Mais barato | Menos documentado, menos community support, hardware pior | 950/XL são os únicos com UEFI community |
| Nenhum Lumia — só Android | Foco total no produto | Perde referência de hardware original, validação ARM64 | Research value justifica effort baixo |

---

## Plano de Implementação

1. **M0:** Identificar modelo exato (950 vs 950 XL), variante (RM-1104/1105/1116), bootloader status
2. **M0:** Revisar LumiaWOA repos, Lumia950XLPkg UEFI, documentar status atual
3. **M5 (Semana 17+):** Linux ARM64 boot validation, driver matrix per component
4. **M5:** Feasibility report: Go/No-go para imagem Conexus dedicada

---

## Referências

- [LumiaWOA Project](https://github.com/WOA-Project/LumiaWOA)
- [Lumia950XLPkg (UEFI)](https://github.com/WOA-Project/Lumia950XLPkg)
- [WOA Project GitHub](https://github.com/WOA-Project)
- [SDD v0.1 §3, §4.1, §10](./../../SDD.md)