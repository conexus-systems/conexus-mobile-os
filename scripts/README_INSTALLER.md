# Conexus Mobile OS - Automatic Installer

Automated installer for Conexus Mobile OS that detects your device, downloads the correct build, and flashes it automatically.

## Features

- 🔍 **Auto-detection** - Detects connected device via ADB/fastboot
- 📱 **Multi-device support** - Pixel 8a (primary), Nothing Phone 2 (backup)
- 📦 **Auto-download** - Fetches correct build from GitHub Releases (`conexus-systems/conexus-mobile-os`)
- ✅ **Verification** - SHA-256 obrigatório (lido do `SHA256SUMS` da release); sem hash válido o instalador recusa
- 🔒 **Safety checks** - Battery, storage, bootloader status
- 🔄 **OTA ready** - Sets up automatic updates
- 🛡️ **Safety first** - Dry-run mode, confirmations, backup warnings

## Quick Start

```bash
# Quick install (interactive, stable channel)
./scripts/install.sh

# Specific channel
./scripts/install.sh --channel monthly

# Dry run (simulate without changes)
./scripts/install.sh --dry-run

# Skip backup prompt
./scripts/install.sh --skip-backup
```

## Requirements

### Host Machine
- Linux, macOS, or Windows (WSL2)
- Python 3.8+
- Android Platform Tools (`adb`, `fastboot`)
- `sudo` access for udev rules

### Device Requirements
- **Google Pixel 8a (akita)** - Primary target
  - Bootloader unlockable
  - 8GB+ RAM, 128GB+ storage
- **Nothing Phone 2 (pong)** - Backup target
  - Bootloader unlockable
  - 8GB+ RAM

### Device Setup
1. Enable **Developer Options** → **USB Debugging**
2. Enable **OEM Unlocking** in Developer Options
3. Connect via USB-C

## Installation Process

1. **Detection** - Identifies device via ADB/fastboot
2. **Prerequisites** - Battery ≥30%, storage ≥3GB, bootloader status
3. **Download** - Fetches correct build from GitHub Releases
4. **Verification** - SHA256 checksum validation
5. **Flash** - Fastboot flash all partitions (A/B)
6. **Verify** - Boots and validates installation

## Supported Devices

| Device | Codename | Status | Notes |
|--------|----------|--------|-------|
| Pixel 8a | `akita` | ✅ Primary | Official LineageOS 23.2 support |
| Nothing Phone 2 | `pong` | ✅ Backup | Kernel source available |
| OnePlus 12 | `waffle` | ⚠️ Tertiary | Unofficial LineageOS |
| Xiaomi 14 | `houji` | ❌ Rejected | Anti-dev unlock policy |

## Release Channels

| Channel | Frequency | Use Case |
|---------|-----------|----------|
| `nightly` | Daily | Developers only |
| `weekly` | Weekly | Testers |
| `monthly` | Monthly | Power users |
| `stable` | Quarterly | **Recommended** |

## Udev Rules (Linux)

For device access without `sudo`:

```bash
sudo cp scripts/51-conexus-mobile.rules /etc/udev/rules.d/
sudo udevadm control --reload-rules
sudo udevadm trigger
```

Add user to plugdev group:
```bash
sudo usermod -aG plugdev $USER
# Log out and back in
```

## Manual Installation (Alternative)

If automatic installer fails:

```bash
# 1. Download release
wget https://github.com/conexus-mobile-os/conexus-mobile-os/releases/download/v1.0.0-stable-akita/conexus-akita-v1.0.0-stable.zip

# 2. Extract
unzip conexus-akita-v1.0.0-stable.zip

# 2. Boot to bootloader
adb reboot bootloader

# 3. Flash
fastboot flash boot boot.img
fastboot flash dtbo dtbo.img
fastboot flash vbmeta vbmeta.img
fastboot flash vendor_boot vendor_boot.img
fastboot flash super super.img
fastboot flash recovery recovery.img

# 4. Reboot
fastboot reboot
```

## Troubleshooting

| Issue | Solution |
|-------|----------|
| "No device found" | Enable USB debugging, try different cable/port |
| "Bootloader locked" | Enable OEM unlocking in Developer Options |
| "Fastboot not found" | Install Android Platform Tools |
| "SHA256 mismatch" | Re-download, check internet connection |
| "Boot loop" | Flash stock factory image, retry |

## Support

- **Documentation**: https://conexus-mobile-os.github.io/docs
- **Issues**: https://github.com/conexus-mobile-os/conexus-mobile-os/issues
- **Matrix**: https://matrix.to/#/#conexus-mobile-os:matrix.org
- **Discussions**: https://github.com/conexus-mobile-os/conexus-mobile-os/discussions

## License

Apache-2.0 - See LICENSE file

## Segurança do instalador (fail closed)

- **Sem hash, sem instalação.** A release precisa ter um asset `SHA256SUMS` com a entrada do `.zip`. Não há "pular verificação".
- **Autenticidade ainda não é verificada.** Só a integridade (SHA-256). A verificação de assinatura (`.asc`/minisign) depende da chave de release do A5 e não existe ainda.
- **Extração segura.** O pacote só pode conter `.img`, `.txt` e `.json` no nível raiz; caminhos absolutos, `..`, subpastas, links simbólicos, excesso de entradas ou de tamanho são recusados. `boot.img`, `vbmeta.img` e `super.img` são obrigatórios.
- **Aparelho certo.** O `fastboot getvar product` precisa ser igual ao codinome detectado, senão não grava nada.
- **Autorização explícita.** Para gravar, digite a frase `FLASH <codinome> <serial>` mostrada na tela. `--dry-run` nunca grava.
- **Sem release de demonstração.** Se não houver release publicada, o instalador diz que não encontrou e para.

Antes de usar em qualquer aparelho, siga o ADR-005: o procedimento de recovery precisa estar testado. Hoje não há release publicada.

## Testes

```bash
python3 -m unittest discover -s scripts/tests -v
```
