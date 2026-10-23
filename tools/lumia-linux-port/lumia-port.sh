#!/usr/bin/env bash
# lumia-port.sh — Helper para portagem Linux (postmarketOS) no Lumia 730/735 (nokia-superman)
# Uso: ./lumia-port.sh {init|build|flash|status|clean|help}
# Requer: Linux real (não WSL2), pmbootstrap, fastboot, device com bootloader UNLOCKED

set -euo pipefail

# Configurações
DEVICE_VENDOR="nokia"
DEVICE_CODENAME="superman"
DEVICE_ARCH="armhf"        # ARMv7 32-bit
KERNEL_FLAVOR="downstream" # mainline não bootada ainda
UI="none"                  # Sem display driver
WORK_DIR="${HOME}/.local/var/pmbootstrap"
PMBS_REPO="https://gitlab.postmarketos.org/postmarketOS/pmbootstrap.git"
PMBS_DIR="${WORK_DIR}/pmbootstrap"

# Cores
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info()  { echo -e "${BLUE}[INFO]${NC} $*"; }
log_ok()    { echo -e "${GREEN}[OK]${NC} $*"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

show_banner() {
    cat <<'EOF'
╔══════════════════════════════════════════════════════════════════╗
║           Lumia 730/735 Linux Port Helper (nokia-superman)        ║
║                      postmarketOS / pmbootstrap                   ║
║                                                                   ║
║  ⚠️  RESULTADO: Linux headless (sem display/touch/WiFi/modem)     ║
║  ✅  Kernel bootando + shell serial/USB                           ║
╚══════════════════════════════════════════════════════════════════╝
EOF
}

check_prereqs() {
    log_info "Verificando pré-requisitos..."
    
    local missing=()
    
    command -v git >/dev/null 2>&1 || missing+=("git")
    command -v python3 >/dev/null 2>&1 || missing+=("python3")
    command -v fastboot >/dev/null 2>&1 || missing+=("fastboot (android-tools)")
    command -v adb >/dev/null 2>&1 || missing+=("adb (android-tools)")
    command -v qemu-arm-static >/dev/null 2>&1 || missing+=("qemu-user-static")
    command -v binfmt-support >/dev/null 2>&1 || missing+=("binfmt-support")
    command -v sudo >/dev/null 2>&1 || missing+=("sudo")
    
    if [[ ${#missing[@]} -gt 0 ]]; then
        log_error "Faltando dependências: ${missing[*]}"
        log_info "Ubuntu/Debian: sudo apt install git python3 android-tools-adb android-tools-fastboot qemu-user-static binfmt-support"
        log_info "Fedora: sudo dnf install git python3 android-tools qemu-user-static binfmt-support"
        log_info "Arch: sudo pacman -S git python android-tools qemu-user-static binfmt-support"
        return 1
    fi
    
    # Verifica se está em Linux real (não WSL2 para flash)
    if grep -qi microsoft /proc/version 2>/dev/null; then
        log_warn "WSL2 detectado — build funciona, mas FLASH NÃO FUNCIONA (USB passthrough problemático)"
        log_warn "Use Linux real (dual boot, VM com USB passthrough, ou live USB) para ./lumia-port.sh flash"
    fi
    
    log_ok "Pré-requisitos OK"
    return 0
}

check_device_fastboot() {
    log_info "Verificando device em fastboot mode..."
    local devices
    devices=$(fastboot devices 2>/dev/null | grep -v '^$' | wc -l)
    
    if [[ $devices -eq 0 ]]; then
        log_error "Nenhum device em fastboot mode"
        log_info "Coloque o Lumia em fastboot: Power + Vol Down (ligado) → fastboot mode"
        log_info "Verifique: fastboot devices"
        return 1
    fi
    
    local unlocked
    unlocked=$(fastboot getvar unlocked 2>&1 | grep -i "unlocked: yes" || true)
    if [[ -z $unlocked ]]; then
        log_error "Bootloader NÃO está unlocked!"
        log_info "Faça unlock no Windows com WPInternals primeiro (veja wpinternals-guide.md)"
        return 1
    fi
    
    log_ok "Device em fastboot mode + bootloader unlocked"
    return 0
}

cmd_init() {
    show_banner
    check_prereqs || return 1
    
    log_info "Configurando pmbootstrap para ${DEVICE_VENDOR}/${DEVICE_CODENAME}..."
    
    mkdir -p "${WORK_DIR}"
    
    if [[ ! -d "${PMBS_DIR}/.git" ]]; then
        log_info "Clonando pmbootstrap..."
        git clone "${PMBS_REPO}" "${PMBS_DIR}"
    else
        log_info "Atualizando pmbootstrap..."
        git -C "${PMBS_DIR}" pull
    fi
    
    # Configura pmbootstrap non-interactive
    log_info "Executando pmbootstrap init (non-interactive)..."
    "${PMBS_DIR}/pmbootstrap.py" init \
        --work-dir "${WORK_DIR}" \
        --device "${DEVICE_VENDOR}-${DEVICE_CODENAME}" \
        --kernel "linux-${DEVICE_VENDOR}-${DEVICE_CODENAME}" \
        --ui "${UI}" \
        --username user \
        --password "${PMBS_PASS:-lumia123}" \
        --full-disk-encryption no \
        --no-fde \
        --mirror "https://mirror.postmarketos.org" \
        --allow-untrusted \
        --details-to-stdout 2>&1 | tee "${WORK_DIR}/init.log"
    
    # Verifica se config salvou
    if [[ -f "${WORK_DIR}/config_${DEVICE_VENDOR}-${DEVICE_CODENAME}.sh" ]]; then
        log_ok "Configuração salva em ${WORK_DIR}/config_${DEVICE_VENDOR}-${DEVICE_CODENAME}.sh"
    else
        log_warn "Config pode não ter salvo corretamente — verifique ${WORK_DIR}/init.log"
    fi
    
    # Cria symlink conveniente
    ln -sf "${PMBS_DIR}/pmbootstrap.py" "${WORK_DIR}/pmbootstrap"
    
    log_ok "Init concluído. Próximo: ./lumia-port.sh build"
}

cmd_build() {
    show_banner
    check_prereqs || return 1
    
    if [[ ! -f "${WORK_DIR}/config_${DEVICE_VENDOR}-${DEVICE_CODENAME}.sh" ]]; then
        log_error "Config não encontrada. Rode ./lumia-port.sh init primeiro"
        return 1
    fi
    
    log_info "Iniciando build (pode demorar 30-60 min na primeira vez)..."
    log_info "Logs em ${WORK_DIR}/build.log"
    
    # Build kernel + rootfs
    "${PMBS_DIR}/pmbootstrap.py" \
        --work-dir "${WORK_DIR}" \
        build \
        --details-to-stdout 2>&1 | tee "${WORK_DIR}/build.log"
    
    log_ok "Build concluído!"
    log_info "Imagens em:"
    find "${WORK_DIR}/cache" -name "*.img" -o -name "*.tar.gz" 2>/dev/null | head -20
    
    log_info "Próximo: ./lumia-port.sh flash (device em fastboot mode)"
}

cmd_flash() {
    show_banner
    check_prereqs || return 1
    check_device_fastboot || return 1
    
    log_info "Procurando imagens para flash..."
    
    local boot_img rootfs_img
    boot_img=$(find "${WORK_DIR}/cache" -name "boot.img-${DEVICE_VENDOR}-${DEVICE_CODENAME}*" -type f 2>/dev/null | head -1)
    rootfs_img=$(find "${WORK_DIR}/cache" -name "rootfs.img-${DEVICE_VENDOR}-${DEVICE_CODENAME}*" -type f 2>/dev/null | head -1)
    
    if [[ -z $boot_img || -z $rootfs_img ]]; then
        log_error "Imagens não encontradas. Rode ./lumia-port.sh build primeiro"
        log_info "Procurado em: ${WORK_DIR}/cache"
        return 1
    fi
    
    log_info "Encontrado:"
    log_info "  boot:  ${boot_img}"
    log_info "  rootfs: ${rootfs_img}"
    
    # Confirmação
    echo
    read -rp "Flash NOW? Isso apaga o device. (y/N): " confirm
    if [[ ${confirm,,} != "y" ]]; then
        log_info "Cancelado."
        return 0
    fi
    
    log_info "Flashando boot..."
    fastboot flash boot "${boot_img}"
    
    log_info "Flashando rootfs..."
    fastboot flash userdata "${rootfs_img}"  # Lumia usa userdata para rootfs
    
    log_info "Rebooting..."
    fastboot reboot
    
    log_ok "Flash concluído! Device reiniciando..."
    log_info "Aguarde 30-60s e rode ./lumia-port.sh status"
}

cmd_status() {
    show_banner
    
    log_info "Verificando device (ADB ou fastboot)..."
    
    # Tenta ADB primeiro
    if adb devices 2>/dev/null | grep -q "device$"; then
        log_ok "Device acessível via ADB"
        log_info "Coletando status do hardware..."
        
        adb shell cat /proc/cpuinfo 2>/dev/null | head -20
        echo
        adb shell cat /proc/meminfo 2>/dev/null | head -10
        echo
        adb shell ls /sys/class/leds/ 2>/dev/null
        echo
        adb shell ls /dev/dri/ 2>/dev/null || log_warn "Sem /dev/dri (GPU/display não funcional)"
        echo
        adb shell ls /dev/input/ 2>/dev/null || log_warn "Sem /dev/input (touch não funcional)"
        echo
        adb shell ip link show 2>/dev/null | grep -E "^[0-9]+:" | head -10
        echo
        adb shell dmesg 2>/dev/null | tail -20
        
    elif fastboot devices 2>/dev/null | grep -q "fastboot$"; then
        log_warn "Device em fastboot mode — não bootou no Linux ainda"
        log_info "Tente: fastboot getvar unlocked && fastboot reboot"
        
    else
        log_error "Device não detectado (nem ADB, nem fastboot)"
        log_info "Verifique cabo USB, modo fastboot, drivers"
        return 1
    fi
}

cmd_clean() {
    log_info "Limpando build dir (mantém config)..."
    rm -rf "${WORK_DIR}/cache" "${WORK_DIR}/tmp" "${WORK_DIR}/build.log"
    log_ok "Limpo. Config preservada em ${WORK_DIR}/config_${DEVICE_VENDOR}-${DEVICE_CODENAME}.sh"
}

cmd_help() {
    show_banner
    cat <<EOF
Uso: ./lumia-port.sh {init|build|flash|status|clean|help}

Comandos:
  init     Configura pmbootstrap para nokia-superman (uma vez)
  build    Compila kernel + rootfs (30-60 min primeira vez)
  flash    Flasha boot + rootfs via fastboot (precisa device unlocked)
  status   Verifica hardware funcionando no device (via ADB)
  clean    Remove cache/build (mantém config)
  help     Mostra esta ajuda

Pré-requisitos (Linux real):
  - git, python3, android-tools (adb/fastboot), qemu-user-static, binfmt-support
  - Device com bootloader UNLOCKED (feita no Windows via WPInternals)

Fluxo típico:
  1. Windows: WPInternals unlock bootloader (veja wpinternals-guide.md)
  2. Linux:   ./lumia-port.sh init
  3. Linux:   ./lumia-port.sh build
  4. Linux:   ./lumia-port.sh flash  (device em fastboot)
  5. Linux:   ./lumia-port.sh status (após boot)

⚠️  Resultado: Linux headless (sem display/touch/WiFi/modem/áudio/câmera)
    Para desenvolvimento de kernel/drivers APENAS.
EOF
}

main() {
    local cmd="${1:-help}"
    case "${cmd}" in
        init)    cmd_init ;;
        build)   cmd_build ;;
        flash)   cmd_flash ;;
        status)  cmd_status ;;
        clean)   cmd_clean ;;
        help|*)  cmd_help ;;
    esac
}

main "$@"