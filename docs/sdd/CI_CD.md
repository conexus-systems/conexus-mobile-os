# CI_CD — Fase 1

Workflow: `.github/workflows/shell-poc.yml`
- **build+test**: ubuntu-latest, apt qt6-base-dev/qt6-declarative-dev/qt6-base-dev-tools, CMake build, `ctest` offscreen, upload de screenshots como artifact.
- **qemu-boot**: busybox estático + initramfs + `qemu-system-x86` (TCG), grep `CONEXUS-BOOT-OK` no serial — falha se não bootar.
Arquiteturas: x86_64 agora; aarch64 na Fase 6 (toolchain cruzada); armv7 (Lumia) só com toolchain comprovada.
Nenhuma imagem é publicada estável sem os critérios do RELEASE_CHECKLIST (Fase 7).
