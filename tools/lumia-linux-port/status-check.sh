#!/usr/bin/env bash
# status-check.sh — Verifica hardware funcionando no Lumia 730/735 pós-flash
# Uso: ./status-check.sh
# Requer: device bootado com ADB acessível

set -euo pipefail

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
CYAN='\033[0;36m'
NC='\033[0m'

log_info()  { echo -e "${BLUE}[INFO]${NC} $*"; }
log_ok()    { echo -e "${GREEN}[✓]${NC} $*"; }
log_warn()  { echo -e "${YELLOW}[⚠]${NC} $*"; }
log_fail()  { echo -e "${RED}[✗]${NC} $*"; }
log_section() { echo -e "\n${CYAN}=== $* ===${NC}"; }

# Verifica ADB
if ! adb devices 2>/dev/null | grep -q "device$"; then
    log_error "Device não acessível via ADB"
    log_info "Verifique: device bootou? Depuração USB ativa? Cabo conectado?"
    log_info "Se em fastboot: fastboot reboot"
    exit 1
fi

log_ok "Device conectado via ADB"

run_adb() {
    adb shell "$@" 2>/dev/null || echo "FALHOU: $*"
}

# ============================================================
# CPU / ARQUITETURA
# ============================================================
log_section "CPU / ARQUITETURA"
run_adb cat /proc/cpuinfo | head -20
echo
run_adb uname -a

# ============================================================
# MEMÓRIA
# ============================================================
log_section "MEMÓRIA"
run_adb cat /proc/meminfo | head -15

# ============================================================
# STORAGE / PARTIÇÕES
# ============================================================
log_section "STORAGE / PARTIÇÕES"
run_adb lsblk
echo
run_adb df -h

# ============================================================
# DISPLAY / GPU (ESPERADO FALHAR)
# ============================================================
log_section "DISPLAY / GPU (ESPERADO: NÃO FUNCIONA)"
if run_adb ls /dev/dri/ | grep -q "."; then
    log_ok "/dev/dri/ existe:"
    run_adb ls -la /dev/dri/
else
    log_fail "/dev/dri/ VAZIO — Sem driver GPU/DRM (Adreno 305 sem suporte mainline)"
fi

if run_adb ls /sys/class/graphics/ 2>/dev/null | grep -q "."; then
    log_ok "/sys/class/graphics/ existe:"
    run_adb ls -la /sys/class/graphics/
else
    log_fail "/sys/class/graphics/ VAZIO — Sem framebuffer"
fi

# ============================================================
# TOUCH / INPUT (ESPERADO FALHAR)
# ============================================================
log_section "TOUCH / INPUT (ESPERADO: NÃO FUNCIONA)"
if run_adb ls /dev/input/ | grep -q "event"; then
    log_ok "/dev/input/ tem dispositivos:"
    run_adb ls -la /dev/input/
else
    log_fail "/dev/input/ VAZIO — Sem driver touch/I2C (synaptics/focaltech)"
fi

# ============================================================
# USB / SERIAL
# ============================================================
log_section "USB / SERIAL"
run_adb ls /dev/tty* 2>/dev/null | head -10
run_adb ls /sys/class/tty/ 2>/dev/null | head -10
run_adb dmesg | grep -i usb | tail -10

# ============================================================
# LEDs (FUNCIONA)
# ============================================================
log_section "LEDS (DEVE FUNCIONAR)"
if run_adb ls /sys/class/leds/ | grep -q "."; then
    log_ok "LEDs detectados:"
    for led in $(run_adb ls /sys/class/leds/); do
        brightness=$(run_adb cat /sys/class/leds/${led}/brightness 2>/dev/null)
        max_brightness=$(run_adb cat /sys/class/leds/${led}/max_brightness 2>/dev/null)
        trigger=$(run_adb cat /sys/class/leds/${led}/trigger 2>/dev/null)
        echo "  ${led}: brightness=${brightness}/${max_brightness} trigger=${trigger}"
    done
else
    log_fail "Nenhum LED em /sys/class/leds/"
fi

# ============================================================
# REDE (ESPERADO FALHAR - SEM WIFI/MODEM)
# ============================================================
log_section "REDE (ESPERADO: SEM WIFI/MODEM)"
run_adb ip link show
echo
run_adb iw dev 2>/dev/null || log_fail "iw não disponível / sem WiFi"
run_adb ls /sys/class/net/

# ============================================================
# MODEM / RIL (ESPERADO FALHAR)
# ============================================================
log_section "MODEM / RIL (ESPERADO: NÃO FUNCIONA)"
run_adb ls /dev/ttyUSB* 2>/dev/null || log_fail "Sem /dev/ttyUSB* (modem serial)"
run_adb ls /dev/ttyACM* 2>/dev/null || log_fail "Sem /dev/ttyACM* (modem USB)"
run_adb mmcli -L 2>/dev/null || log_fail "ModemManager não vê modem"

# ============================================================
# ÁUDIO (ESPERADO FALHAR)
# ============================================================
log_section "ÁUDIO (ESPERADO: NÃO FUNCIONA)"
run_adb cat /proc/asound/cards 2>/dev/null || log_fail "ALSA não detecta codecs (WCD9320)"
run_adb ls /dev/snd/ 2>/dev/null || log_fail "Sem /dev/snd/"

# ============================================================
# CÂMERA (ESPERADO FALHAR)
# ============================================================
log_section "CÂMERA (ESPERADO: NÃO FUNCIONA)"
run_adb ls /dev/video* 2>/dev/null || log_fail "Sem /dev/video* (V4L2)"
run_adb v4l2-ctl --list-devices 2>/dev/null || log_fail "v4l2-ctl não disponível / sem driver camera"

# ============================================================
# SENSORES (ESPERADO FALHAR)
# ============================================================
log_section "SENSORES (ESPERADO: NÃO FUNCIONA)"
run_adb ls /sys/bus/i2c/devices/ 2>/dev/null | head -20
run_adb ls /sys/class/sensors/ 2>/dev/null || log_fail "Sem /sys/class/sensors/"

# ============================================================
# GPIO / PINMUX (PARA DEV DRIVERS)
# ============================================================
log_section "GPIO / PINMUX (ÚTIL PARA DEV)"
run_adb ls /sys/class/gpio/ 2>/dev/null || log_warn "GPIO sysfs não montado (pode usar libgpiod)"
run_adb gpiodetect 2>/dev/null || log_warn "libgpiod não instalado / sem gpiochip"
run_adb cat /sys/kernel/debug/pinctrl/*/pins 2>/dev/null | head -30 || log_warn "debugfs/pinctrl não acessível"

# ============================================================
# KERNEL LOG (ÚLTIMAS MENSAGENS)
# ============================================================
log_section "KERNEL LOG (dmesg - últimas 30 linhas)"
run_adb dmesg | tail -30

# ============================================================
# RESUMO
# ============================================================
log_section "RESUMO DE FUNCIONALIDADE"

cat <<'EOF'
┌─────────────────────┬──────────┬────────────────────────────────────────┐
│ Componente          │ Status   │ Notas                                  │
├─────────────────────┼──────────┼────────────────────────────────────────┤
│ CPU (ARMv7 4-core)  │ ✅ OK    │ Cortex-A7 @ 1.2 GHz                    │
│ RAM (1 GB)          │ ✅ OK    │ ~900 MB disponível                     │
│ Storage (8 GB)      │ ✅ OK    │ eMMC, particionado                     │
│ Kernel Linux        │ ✅ OK    │ Bootando, shell funcional              │
│ LEDs                │ ✅ OK    │ /sys/class/leds/ funcional             │
│ USB Serial/ADB      │ ✅ OK    │ Comunicação host ↔ device              │
│ Display/GPU         │ ❌ NÃO   │ Adreno 305 sem driver DRM/KMS mainline │
│ Touchscreen         │ ❌ NÃO   │ Sem driver I2C (synaptics/focaltech)   │
│ WiFi                │ ❌ NÃO   │ QCA6174 sem firmware/blobs             │
│ Bluetooth           │ ❌ NÃO   │ QCA6174 sem firmware/blobs             │
│ Modem/RIL           │ ❌ NÃO   │ MDM9x25 sem driver QMI/RPC             │
│ Áudio               │ ❌ NÃO   │ WCD9320 sem driver ASoC/ALSA           │
│ Câmera              │ ❌ NÃO   │ OV5670/IMX179 sem driver V4L2          │
│ GPS/GNSS            │ ❌ NÃO   │ Sem driver                             │
│ NFC                 │ ❌ NÃO   │ Sem driver                             │
│ Sensores (accel,    │ ❌ NÃO   │ Sem drivers I2C                        │
│  gyro, prox, luz)   │          │                                        │
└─────────────────────┴──────────┴────────────────────────────────────────┘

PARA DESENVOLVIMENTO DE DRIVERS:
→ Foque em: GPU (DRM/KMS), Touch (I2C), WiFi (ath10k/ath11k), Modem (QRTR/QMI)
→ Referência: linux-msm (kernel Qualcomm downstream) + linux-mainline (WIP)
→ Contato: RayyanAnsari @ gitlab.postmarketos.org (mainline WIP para nokia-superman)
EOF