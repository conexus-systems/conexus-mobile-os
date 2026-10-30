# ORCHESTRATION_DECISIONS

- D-001 (2026-10-29): Nenhum merge de PR Dependabot. Todos BLOCKED por branch protection; a regra do SDD proíbe contornar proteção. Reavaliar quando checks estiverem verdes e houver aprovação.
- D-002: Cards no Trello só mudam de coluna após verificação do código real; este ciclo não moveu nenhum card.
- D-003: Ciclo 1 limitado a descoberta + estado. 80+ repos / 547 cards exigem execução em ondas por repo (trello-orchestrator), não em uma única passada.
- D-004: Limpeza de worktrees e deploy produção exigem confirmação explícita do Tiago.
