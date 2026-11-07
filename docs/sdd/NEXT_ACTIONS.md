# NEXT_ACTIONS — após esta execução

1. (done) PoC shell + testes + screenshots — revisar PR `feat/phase1-shell-poc`.
2. Integrar o binário Qt do shell ao ambiente QEMU (rootfs mínimo ou libs no initramfs) — fecha a Fase 1 "shell inicializado em QEMU".
3. CI: verificar o primeiro run do `shell-poc.yml` no push; ajustar dependências qt6 do runner.
4. Fase 2 (Shell MVP): TileProvider API + cache, drag-and-drop/resizing, notifications model, temas light/dark, busca.
5. Resolvedor de sudo (usuário) para debootstrap/OSTree — desbloqueia rootfs real.
6. SDD-HARMATTAN (track Rust multiplataforma) e SDD-MERIDIAN-MOBILE v2.1 (track Linux) continuam como trilhas de hardware.
