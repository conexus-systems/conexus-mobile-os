# Known Issues — Lumia 730/735 (nokia-superman) Linux Port

**Atualizado:** 2026-10-22  
**Base:** postmarketOS edge / pmbootstrap / linux-mainline 6.x / linux-msm 4.19 downstream

---

## 🔴 CRÍTICOS — Bloqueiam Uso Como Telefone

| Componente | Status | Detalhes | Workaround |
|------------|--------|----------|------------|
| **Display / GPU** | ❌ Quebrado | Adreno 305 (OpenGL ES 3.0). Nenhum driver DRM/KMS no mainline. Downstream (linux-msm 4.19) tem `msm` driver mas precisa DTB correto + firmware. | Nenhum viável. Use serial/ADB apenas. |
| **Touchscreen** | ❌ Quebrado | Synaptics S3320 / FocalTech FT5x06 via I2C. Driver `synaptics_i2c` / `focaltech` não carregado (sem DTB bindings). | Nenhum. |
| **WiFi** | ❌ Quebrado | QCA6174 (ath10k). Precisa firmware `ath10k/firmware-6.bin` + board data + DTB. Blobs proprietários não redistribuíveis. | USB tethering via ADB? (precisa kernel config). |
| **Bluetooth** | ❌ Quebrado | Mesmo chip QCA6174 (BT via UART/HCI). Precisa firmware + hciattach. | Nenhum. |
| **Modem / RIL** | ❌ Quebrado | MDM9x25 (Qualcomm 9x25). Comunicação via QRTR/QMI/RPC. Drivers `qrtr`, `qmi`, `msm_pcie` não mainline. | Nenhum. |
| **Áudio** | ❌ Quebrado | Codec WCD9320 (I2C) + WCD9320 MBHC + MI2S. Drivers ASoC `wcd9320`, `msm8916-wcd-analog` não mainline. | Nenhum. |
| **Câmera** | ❌ Quebrado | Traseira OV5670 / Frontal IMX179. Drivers V4L2 `ov5670`, `imx179` + ISP `msm_isp` não mainline. | Nenhum. |

---

## 🟡 PARCIAIS — Funcionam Com Limitações

| Componente | Status | Detalhes |
|------------|--------|----------|
| **Kernel Boot** | ✅ Funciona | Downstream 4.19 bootado via lk2nd. Mainline 6.x WIP (RayyanAnsari) — boot instável. |
| **RAM / CPU** | ✅ OK | 4× Cortex-A7 detectados, ~900 MB livres. |
| **Storage (eMMC)** | ✅ OK | Partições montadas, rootfs em userdata. |
| **LEDs** | ✅ OK | `/sys/class/leds/` funcional (vermelho, verde, azul, LCD backlight). |
| **USB / ADB** | ✅ OK | `android_usb` gadget + `adb` funcionando. Shell acessível. |
| **Serial Console** | ✅ OK | `ttyMSM0` @ 115200 via cabo USB (precisa breakout board para bootlog completo). |
| **GPIO / Pinmux** | ⚠️ Parcial | `/sys/class/gpio/` sysfs OK. `libgpiod` / `gpiodetect` funciona se kernel config `GPIOLIB=y`. Debugfs pinctrol precisa `DEBUG_FS=y`. |
| **I2C Bus** | ⚠️ Parcial | Barramentos I2C expostos (`/sys/bus/i2c/devices/`), mas dispositivos não probeados sem DTB bindings. |
| **PMIC / Regulators** | ⚠️ Parcial | PM8916 detectado, regulators listados, mas controle limitado sem drivers consumers. |
| **Thermal** | ⚠️ Parcial | `msm_thermal` carrega, zonas térmicas expostas, mas throttling não testado. |
| **Battery / Fuel Gauge** | ⚠️ Parcial | `bq27421` / `bq27541` via I2C — driver carrega mas sem DTB binding não lê capacidade. |

---

## 🟢 FUNCIONAIS — Para Desenvolvimento Kernel

| Recurso | Status | Como Testar |
|---------|--------|-------------|
| **Shell ADB** | ✅ | `adb shell` → root prompt |
| **dmesg / kmsg** | ✅ | `adb shell dmesg`, `cat /proc/kmsg` |
| **/proc / /sys** | ✅ | Inspeção completa de dispositivos, drivers, regulators |
| **Kernel Modules** | ✅ | `insmod`/`rmmod` `.ko` cross-compilados (`make -C /lib/modules/$(uname -r)/build M=$PWD`) |
| **FTrace / Perf** | ✅ | `trace-cmd`, `perf` (se `PERF_EVENTS=y`) |
| **Kprobes / Kretprobes** | ✅ | Debug dinâmico de funções kernel |
| **KGDB / GDB** | ✅ | `gdb vmlinux` + `kgdboc=ttyMSM0,115200` (precisa serial físico) |
| **Cross-compile** | ✅ | `arm-linux-gnueabihf-gcc` / `aarch64-linux-gnu-gcc` (para módulos 32-bit) |
| **DTB Overlay** | ✅ | Testar device tree overlays via `mkdtboimg.py` + fastboot flash dtbo |

---

## 📋 Drivers Prioritários Para Portar (Ordem de Impacto)

1. **Display/GPU** — `drivers/gpu/drm/msm/` (downstream) → mainline `drm/msm/`
   - Requer: DTB bindings para `mdss`, `mdp`, `hdmi`, `edp`
   - Firmware: `a3xx_gmu.bin`, `a3xx_*.fw` (proprietário)

2. **Touchscreen** — `drivers/input/touchscreen/synaptics_i2c.c`, `focaltech.c`
   - Requer: DTB `touchscreen@XX` com `compatible = "synaptics,s3320"` + interrupt/GPIOs

3. **WiFi** — `drivers/net/wireless/ath/ath10k/`
   - Requer: Firmware `ath10k/QCA6174/hw3.0/` + board-2.bin + DTB `wcnss@XX`

4. **Modem** — `drivers/modem/qcom/qrtr.c`, `drivers/net/qrtr/`
   - Requer: `qrtr` + `smd` + `rpcrouter` + DTB `modem@XX` + firmware `modem_fw.elf`

5. **Audio** — `sound/soc/qcom/wcd9320.c`, `sound/soc/qcom/msm8916-wcd-analog.c`
   - Requer: DTB `sound@XX` com `wcd9320` + `mi2s` + `lpass` + firmware `wcd9320_mbhc.bin`

6. **Camera** — `drivers/media/platform/msm/camera/v2/` (downstream) → mainline `v4l2/msm/`
   - Requer: `msm_csid`, `msm_csiphy`, `msm_isp`, `msm_vfe` + DTB `camera@XX`

---

## 📚 Referências Para Portagem

| Recurso | Link |
|---------|------|
| **linux-msm (downstream 4.19)** | https://github.com/freedreno/linux-msm |
| **linux-mainline (WIP msn8916)** | https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/log/drivers/soc/qcom |
| **postmarketOS device tree** | https://gitlab.postmarketos.org/postmarketOS/pmaports/-/tree/master/device/device-nokia-superman |
| **RayyanAnsari mainline WIP** | https://gitlab.postmarketos.org/RayyanAnsari/linux/-/commits/nokia-superman-mainline |
| **freedreno (GPU)** | https://github.com/freedreno/freedreno |
| **ath10k firmware** | https://github.com/kvalle/ath10k-firmware |
| **Qualcomm QRTR/RPC** | https://www.kernel.org/doc/html/latest/driver-api/qrtr.html |
| **DTB bindings (MSM8916)** | https://git.kernel.org/pub/scm/linux/kernel/git/torvalds/linux.git/tree/Documentation/devicetree/bindings/arm/qcom |
| **XDA Lumia 730 Dev** | https://xdaforums.com/f/nokia-lumia-730.4529/ |

---

## 🛠 Dicas Para Desenvolvedores

```bash
# 1. Cross-compilar módulo kernel para testar no device
make -C /lib/modules/$(uname -r)/build M=$PWD ARCH=arm CROSS_COMPILE=arm-linux-gnueabihf- modules
adb push meu_driver.ko /data/local/tmp/
adb shell insmod /data/local/tmp/meu_driver.ko

# 2. Extrair DTB do boot.img para análise
adtbtool -x boot.img -o dtb_dump/
dtc -I dtb -O dts dtb_dump/dtb -o device_tree.dts

# 3. Testar DTB overlay sem reflashar kernel
# (precisa suporte no bootloader — lk2nd suporta)
fastboot flash dtbo meu_overlay.dtbo

# 4. Serial console físico (bootlog completo)
# Cabo USB breakout → GND + TX + RX → 115200 8N1
# screen /dev/ttyUSB0 115200

# 5. Kernel config do device (para replicar)
adb shell zcat /proc/config.gz > config_nokia-superman
```

---

## 📝 Contribuições

Se você portar algum driver:
1. Fork `pmaports` → `device-nokia-superman` / `linux-nokia-superman`
2. MR no postmarketOS GitLab
3. Teste no hardware real (obrigatório)
4. Documente no `known-issues.md` o que funcionou

**Contato maintainer:** RayyanAnsari (@RayyanAnsari no GitLab postmarketOS)