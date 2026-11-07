# KNOWN_LIMITATIONS — execução 2026-11-07

1. **Sem PDF**: matriz `PDF_REQUIREMENTS_MATRIX.md` registrada como vazia.
2. **sudo indisponível no host**: sem debootstrap/systemd-nspawn → o "distro rootfs" da Fase 1
   ficou no nível initramfs (busybox). Rootfs Debian/OSTree completo exige sudo (ou CI runner).
3. **KVM aninhado travou** (timeout sem output) → boot QEMU validado em **TCG** (~4 s). Investigar
   permissões/nested-KVM depois.
4. **Shell PoC roda no host/QEMU windowed**, não dentro do QEMU — integrar Qt6 ao initramfs é o
   próximo passo (bibliotecas no initramfs ou rootfs mínimo).
5. **Tiles "demo"** representam estados demonstrativos (marcados); nenhum dado externo inventado.
6. **APP_FOREGROUND/OVERVIEW** sem runtime de apps ainda (Fase 2/5).
7. CI definido mas **não executado** (primeiro run acontece no push do branch).
