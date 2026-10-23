#!/usr/bin/env bash
# flash.sh — Flash manual do kernel + rootfs no Lumia 730/735 via fastboot
# Uso: ./flash.sh [boot.img] [rootfs.img]
# Se não passar argumentos, usa as imagens mais recentes do pmbootstrap

set -euo pipefail

WORK_DIR="${HOME}/.local/var/pmbootstrap"
DEVICE_VENDOR="nokia"
DEVICE_CODENAME="superman"

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m'

log_info()  { echo -e "${BLUE}[INFO]${NC} $*"; }
log_ok()    { echo -e "${GREEN}[OK]${NC} $*"; }
log_warn()  { echo -e "${YELLOW}[WARN]${NC} $*"; }
log_error() { echo -e "${RED}[ERROR]${NC} $*"; }

# Encontra imagens mais recentes
find_latest_img() {
    local pattern="$1"
    find "${WORK_DIR}/cache" -name "${pattern}" -type f 2>/dev/null | sort -V | tail -1
}

BOOT_IMG="${1:-$(find_latest_img "boot.img-${DEVICE_VENDOR}-${DEVICE_CODENAME}*")}"
ROOTFS_IMG="${2:-$(find_latest_img "rootfs.img-${DEVICE_VENDOR}-${DEVICE_CODENAME}*")}"

if [[ -z $BOOT_IMG || -z $ROOTFS_IMG ]]; then
    log_error "Imagens não encontradas em ${WORK_DIR}/cache"
    log_info "Rode ./lumia-port.sh build primeiro ou passe caminhos:"
    log_info "  ./flash.sh /path/boot.img /path/rootfs.img"
    exit 1
fi

log_info "Imagens selecionadas:"
log_info "  boot:  ${BOOT_IMG}"
log_info "  rootfs: ${ROOTFS_IMG}"

# Verifica device em fastboot
log_info "Verificando device em fastboot..."
if ! fastboot devices 2>/dev/null | grep -q "fastboot$"; then
    log_error "Nenhum device em fastboot mode"
    log_info "Coloque Lumia em fastboot: Power + Vol Down (ligado)"
    exit 1
fi

UNLOCKED=$(fastboot getvar unlocked 2>&1 | grep -i "unlocked: yes" || true)
if [[ -z $UNLOCKED ]]; then
    log_error "Bootloader NÃO unlocked! Faça unlock no Windows (WPInternals)"
    exit 1
fi

log_ok "Device pronto: fastboot + unlocked"

# Confirmação
echo
read -rp "Flash AGORA? Apaga o device. (y/N): " confirm
if [[ ${confirm,,} != "y" ]]; then
    log_info "Cancelado."
    exit 0
fi

# Flash boot
log_info "Flashando boot partition..."
fastboot flash boot "${BOOT_IMG}"

# Flash rootfs (userdata no Lumia)
log_info "Flashando rootfs partition (userdata)..."
fastboot flash userdata "${ROOTFS_IMG}"

# Reboot
log_info "Rebooting..."
fastboot reboot

log_ok "Flash concluído! Aguarde boot (30-60s)."
log_info "Verifique status: adb shell cat /proc/cpuinfo"