# ORCHESTRATION_DECISIONS

- D-001 (2026-10-29): Nenhum merge de PR Dependabot. Todos BLOCKED por branch protection; a regra do SDD proíbe contornar proteção. Reavaliar quando checks estiverem verdes e houver aprovação.
- D-002: Cards no Trello só mudam de coluna após verificação do código real; este ciclo não moveu nenhum card.
- D-003: Ciclo 1 limitado a descoberta + estado. 80+ repos / 547 cards exigem execução em ondas por repo (trello-orchestrator), não em uma única passada.
- D-004: Limpeza de worktrees e deploy produção exigem confirmação explícita do Tiago.
- D-005 (2026-10-30): Cards 1470–1475 (conexus-app, épico 1433) NÃO executados. conexus-app é template Expo/RN; as premissas (Rails/Sidekiq/Terraform AWS) são falsas e o épico foi migrado para bff-auth em 25/10. Comentado nos 6 cards; aguardam decisão do Tiago (reapontar ou fechar).
- D-006: Launcher — PRs #73 (restaura PRIVACY/DATA_SAFETY apagados por c0da8e8) e #74 (workflows manual-only); card 1603 em Em testes (7 testes ok); bff-api#104 em rascunho (card 1276).
