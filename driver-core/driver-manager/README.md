# Conexus Driver Package Manager

Universal driver package management system for Android-based systems.
Allows installing, updating, and managing drivers outside the Android core system.

## Architecture

```
driver-manager/
├── src/
│   ├── main/
│   │   ├── java/
│   │   │   └── com/conexus/driver/manager/
│   │   │       ├── DriverPackageManager.java       # Main package manager
│   │   │       ├── DriverPackage.java              # Driver package representation
│   │   │       ├── DriverRepository.java           # Repository management
│   │   │       ├── DriverInstaller.java            # Installation logic
│   │   │       ├── DriverVerifier.java             # Signature/integrity verification
│   │   │       ├── DriverDependencyResolver.java   # Dependency resolution
│   │   │       ├── DriverIndex.java                # Package index
│   │   │       ├── DriverTransaction.java          # Transaction management
│   │   │       ├── DriverEventListener.java        # Event callbacks
│   │   │       ├── model/
│   │   │       │   ├── DriverPackage.java          # Package metadata
│   │   │       │   ├── DriverManifest.java         # Package manifest
│   │   │       │   ├── DriverDependency.java       # Dependencies
│   │   │       │   ├── DriverSignature.java        # Signatures
│   │   │       │   ├── DriverFile.java             # Package files
│   │   │       │   └── DriverVersion.java          # Version info
│   │   │       ├── repository/
│   │   │       │   ├── LocalRepository.java        # Local package repo
│   │   │       │   ├── RemoteRepository.java       # Remote repo (HTTP/HTTPS)
│   │   │       │   └── RepositoryManager.java      # Repo management
│   │   │       ├── installer/
│   │   │       │   ├── DriverInstaller.java        # Core installer
│   │   │       │   ├── KernelModuleInstaller.java  # Kernel module installer
        │   │       │   ├── HalInstaller.java            # HAL installer
        │   │       │   ├── FirmwareInstaller.java       # Firmware installer
        │   │       │   └── BlobInstaller.java           # Blob installer
        │   │       ├── verifier/
        │   │       │   ├── SignatureVerifier.java       # Signature verification
        │   │       │   ├── HashVerifier.java            # Hash verification
        │   │       │   ├── CertificateVerifier.java     # Certificate chain
        │   │       │   └── IntegrityVerifier.java       # Integrity checks
        │   │       ├── dependency/
        │   │       │   ├── DependencyGraph.java         # Dependency graph
        │   │       │   ├── DependencyResolver.java      # Resolution algorithm
        │   │       │   ├── ConflictResolver.java        # Conflict resolution
        │   │       │   └── VersionConstraint.java       # Version constraints
        │   │       ├── transaction/
        │   │       │   ├── Transaction.java             # Transaction base
        │   │       │   ├── InstallTransaction.java      # Install transaction
        │   │       │   ├── UpdateTransaction.java       # Update transaction
        │   │       │   ├── RemoveTransaction.java       # Remove transaction
        │   │       │   └── TransactionManager.java      # Transaction manager
        │   │       ├── event/
        │   │       │   ├── DriverEvent.java             # Event types
        │   │       │   ├── DriverEventListener.java     # Listener interface
        │   │       │   └── EventBus.java                # Event bus
        │   │       └── cli/
        │   │           ├── CliCommand.java              # CLI commands
        │   │           └── DriverManagerCLI.java        # CLI entry point
        │   └── resources/
        │       ├── signatures/
        │       │   ├── root-ca.pem
        │       │   ├── intermediate-ca.pem
        │       │   └── driver-signing-key.pem
        │       └── policies/
        │           ├── installation-policy.json
        │           ├── security-policy.json
        │           └── update-policy.json
        ├── installer/
        │   ├── build.sh
        │   ├── install.sh
        │   └── verify.sh
        ├── packages/
        │   ├── google/
        │   ├── samsung/
        │   ├── nothing/
        │   ├── oneplus/
        │   ├── xiaomi/
        │   └── generic/
        ├── repository/
        │   ├── index.json
        │   ├── metadata/
        │   └── packages/
        ├── tools/
        │   ├── driver-build.sh
        │   ├── driver-sign.sh
        │   ├── driver-verify.sh
        │   ├── repo-index.sh
        │   └── driver-package.sh
        └── docs/
            ├── package-format.md
            ├── repository-setup.md
            ├── signing-guide.md
            └── development-guide.md
```

## Package Format (`.cdrv` - Conexus Driver Package)

```
driver-package.cdrv/
├── manifest.json              # Package manifest
├── signature.sig              # Package signature
├── certificate.pem            # Signing certificate
├── metadata/
│   ├── dependencies.json      # Dependencies
│   ├── compatibility.json     # Compatibility matrix
│   └── changelog.md          # Changelog
├── kernel/
│   ├── modules/
        │   ├── <module>.ko
        │   └── ...
│   ├── dtb/
        │   └── *.dtbo
│   └── config/
        │   └── kernel.config
├── hal/
│   ├── camera/
│   ├── display/
│   ├── audio/
│   ├── sensors/
│   ├── wifi/
│   ├── bluetooth/
│   ├── nfc/
│   ├── power/
│   ├── thermal/
│   ├── vibrator/
│   ├── usb/
│   └── nfc/
├── firmware/
│   ├── wifi/
│   ├── bluetooth/
│   ├── camera/
│   ├── audio/
│   ├── display/
│   ├── modem/
│   ├── gps/
│   ├── touch/
│   └── sensor/
├── blobs/
│   ├── vendor/
│   └── proprietary/
├── scripts/
│   ├── pre-install.sh
│   ├── post-install.sh
│   ├── pre-remove.sh
│   └── post-remove.sh
└── kernel-module.ko          # Kernel module (optional)
```

## Manifest Format (manifest.json)

```json
{
  "format_version": "1.0",
  "package": {
    "name": "conexus-driver-google-pixel8a",
    "version": "1.0.0",
    "release": "stable",
    "epoch": 1,
    "arch": "arm64",
    "vendor": "google",
    "devices": ["akita"],
    "description": "Conexus Driver Pack for Google Pixel 8a",
    "maintainer": "Conexus Team <team@conexus.systems>",
    "license": "Apache-2.0",
    "homepage": "https://github.com/conexus-mobile-os/conexus-driver-core",
    "source": "https://github.com/conexus-mobile-os/conexus-driver-core"
  },
  "kernel": {
    "min_version": "6.1.0",
    "max_version": "6.1.99",
    "required_config": [
      "CONFIG_ANDROID",
      "CONFIG_ANDROID_BINDER_IPC",
      "CONFIG_ANDROID_BINDERFS"
    ],
    "modules": [
      {
        "name": "conexus_tile_engine",
        "path": "kernel/modules/conexus_tile_engine.ko",
        "license": "GPL-2.0",
        "description": "Conexus Tile Engine kernel module"
      }
    ],
    "dtbos": [
      {
        "name": "conexus-akita",
        "path": "kernel/dtbo/conexus-akita.dtbo",
        "compatible": "google,akita"
      }
    ]
  },
  "hal": {
    "camera": {
      "version": "3.5",
      "path": "hal/camera/",
      "interfaces": ["ICameraProvider", "ICameraDevice"]
    },
    "display": {
      "version": "1.0",
      "path": "hal/display/",
      "interfaces": ["IDisplay", "IComposer"]
    },
    "audio": {
      "version": "7.0",
      "path": "hal/audio/",
      "interfaces": ["IAudioControl", "IAudioStream"]
    },
    "sensors": {
      "version": "2.1",
      "path": "hal/sensors/",
      "interfaces": ["ISensors", "ISensorEvent"]
    },
    "wifi": {
      "version": "1.0",
      "path": "hal/wifi/",
      "interfaces": ["IWifi", "IWifiChip"]
    },
    "bluetooth": {
      "version": "1.0",
      "path": "hal/bluetooth/",
      "interfaces": ["IBluetooth"]
    },
    "nfc": {
      "version": "1.0",
      "path": "hal/nfc/",
      "interfaces": ["INfc"]
    },
    "power": {
      "version": "1.0",
      "path": "hal/power/",
      "interfaces": ["IPower", "IPowerStats"]
    },
    "thermal": {
      "version": "1.0",
      "path": "hal/thermal/",
      "interfaces": ["IThermal"]
    },
    "vibrator": {
      "version": "1.0",
      "path": "hal/vibrator/",
      "interfaces": ["IVibrator"]
    },
    "usb": {
      "version": "1.0",
      "path": "hal/usb/",
      "interfaces": ["IUsb"]
    },
    "nfc": {
      "version": "1.0",
      "path": "hal/nfc/",
      "interfaces": ["INfc"]
    }
  },
  "firmware": {
    "wifi": ["fw_wlan.bin", "fw_wlan_ap.bin"],
    "bluetooth": ["bt_firmware.bin", "bt_firmware_ap.bin"],
    "camera": ["camera_fw.bin"],
    "audio": ["audio_fw.bin"],
    "display": ["display_fw.bin"],
    "modem": ["modem_fw.bin"],
    "gps": ["gps_fw.bin"],
    "touch": ["touch_fw.bin"],
    "sensor": ["sensor_fw.bin"]
  },
  "blobs": {
    "vendor": {
      "libcamera_algo.so": "libs/lib64/libcamera_algo.so",
      "libaudiohal.so": "libs/lib64/libaudiohal.so",
      "libcamera_metadata.so": "libs/lib64/libcamera_metadata.so"
    }
  },
  "dependencies": {
    "kernel": ">=6.1.0",
    "android": ">=14",
    "conexus-core": ">=1.0.0",
    "conexus-sdk": ">=1.0.0"
  },
  "conflicts": [],
  "replaces": [],
  "provides": ["conexus-driver-pack"],
  "conflicts": ["other-driver-pack"],
  "recommends": ["conexus-launcher", "conexus-sdk"],
  "suggests": ["conexus-apps"],
  "scripts": {
    "pre-install": "scripts/pre-install.sh",
    "post-install": "scripts/post-install.sh",
    "pre-remove": "scripts/pre-remove.sh",
    "post-remove": "scripts/post-remove.sh"
  },
  "signatures": {
    "sha256": "sha256:...",
    "signature": "signature.sig",
    "certificate": "certificate.pem"
  }
}
```

## Installation Process

```
┌─────────────────────────────────────────────────────────────┐
│                    INSTALLATION FLOW                        │
└─────────────────────────────────────────────────────────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  1. FETCH PACKAGE     │
        │  (from repo or local) │
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  2. VERIFY INTEGRITY  │
        │  - SHA256 hash        │
        │  │  Signature check   │
        │  │  Cert chain verify │
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  3. RESOLVE DEPENDENCIES│
        │  - Kernel version       │
        │  - Android version      │
        │  - Other driver packs   │
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  4. CONFLICT CHECK    │
        │  - Conflicts          │
        │  - Replaces           │
        │  - File conflicts     │
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  5. PRE-INSTALL HOOK  │
        │  (scripts/pre-install)│
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  6. INSTALL COMPONENTS│
        │  - Kernel modules     │
        │  - HAL implementations│
        │  - Firmware blobs     │
        │  - DTBOs              │
        │  - Config files       │
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  7. POST-INSTALL HOOK │
        │  (scripts/post-install)│
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  8. UPDATE INDEXES    │
        │  - Driver index       │
        │  │  Module.dep        │
        │  │  Initramfs         │
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │  9. POST-INSTALL HOOK │
        │  (scripts/post-install)│
        └───────────┬───────────┘
                    │
                    ▼
        ┌───────────────────────┐
        │ 10. VERIFY INSTALL    │
        │  - Health check       │
        │  - Module loaded      │
        │  │  Services running  │
        └───────────┬───────────┘
                    │
                    ▼
              ┌─────────┐
              │ SUCCESS │
              └─────────┘
```

## Dependency Resolution

```
┌─────────────────────────────────────────────────────────────┐
│                  DEPENDENCY RESOLUTION                      │
└─────────────────────────────────────────────────────────────┘

Input: Requested packages + Installed packages

Step 1: Build Dependency Graph
  Package A (1.0.0) ──requires──> Kernel (>=6.1)
    │
    ├─── requires ──> conexus-core (>=1.0.0)
    │
    ├─── conflicts ──> other-driver-pack (<2.0.0)
    │
    └─── recommends ──> conexus-launcher (>=1.0.0)

Step 2: Check Constraints
  ✓ Kernel version: 6.1.0 >= 6.1.0 ✓
  ✓ Android version: 14 >= 14 ✓
  ✓ conexus-core: 1.0.0 >= 1.0.0 ✓
  ✗ Conflict: other-driver-pack (1.5.0) installed
  ✗ Conflict resolution needed

Step 3: Resolution Strategies
  1. Upgrade conflicting package
  2. Remove conflicting package
  3. Keep both (if compatible)
  4. Abort installation

Step 4: Generate Install Plan
  1. Remove conflicting packages
  2. Install dependencies
  3. Install target package
  4. Run post-install hooks
```

## Repository Structure

```
repository/
├── index.json                    # Package index
├── metadata/
│   ├── conexus-driver-google-pixel8a.json
│   ├── conexus-driver-nothing-phone2.json
│   └── ...
├── packages/
│   ├── conexus-driver-google-pixel8a-1.0.0.cdrv
│   ├── conexus-driver-nothing-phone2-1.0.0.cdrv
│   └── ...
├── signatures/
│   ├── root-ca.pem
│   └── intermediate-ca.pem
└── pool/
    ├── g/
    │   └── google-driver/
    ├── n/
    │   └── nothing-driver/
    └── s/
        └── samsung-driver/
```

## Security Model

```
┌─────────────────────────────────────────────────────────────┐
│                    SECURITY MODEL                           │
└─────────────────────────────────────────────────────────────┘

Package Signing:
  1. Developer signs package with private key
  2. Certificate signed by Conexus Intermediate CA
  3. Intermediate CA signed by Conexus Root CA
  4. Root CA distributed with OS image

Verification Chain:
  Package → Intermediate CA → Root CA → Trust Anchor

Verification Steps:
  1. Verify package signature (SHA256 + RSA/ECDSA)
  2. Verify certificate chain
  3. Check certificate validity (not expired, not revoked)
  4. Verify certificate key usage (code signing)
  5. Check certificate transparency log
  6. Verify package hash matches manifest

Revocation:
  - CRL (Certificate Revocation List)
  - OCSP (Online Certificate Status Protocol)
  - Local revocation list in OS
```

## Quick Start

### Install Driver Package
```bash
# Install from local file
driverctl install ./conexus-driver-google-pixel8a-1.0.0.cdrv

# Install from repository
driverctl install conexus-driver-google-pixel8a --channel stable

# Install specific version
driverctl install conexus-driver-google-pixel8a=1.0.0
```

### Manage Drivers
```bash
# List installed drivers
driverctl list

# Check for updates
driverctl update

# Update all
driverctl upgrade

# Remove driver
driverctl remove conexus-driver-google-pixel8a

# Show package info
driverctl show conexus-driver-google-pixel8a
```

### Repository Management
```bash
# Add repository
driverctl repo add https://conexus-mobile-os.github.io/driver-repo stable

# Remove repository
driverctl repo remove conexus-stable

# List repositories
driverctl repo list

# Update repository index
driverctl repo update
```

### System Integration
```bash
# Check driver status
driverctl status

# Verify installation
driverctl verify conexus-driver-google-pixel8a

# View installed packages
driverctl list --installed

# View available updates
driverctl list --upgradable
```

## Building Driver Packages

```bash
# Create package from source
driverpkg build ./driver-source

# Sign package
driverpkg sign ./package.cdrv --key signing-key.pem --cert cert.pem

# Verify package
driverpkg verify ./package.cdrv

# Publish to repository
driverpkg publish ./package.cdrv --repo https://repo.conexus.mobile-os.org
```

## Configuration

### /etc/conexus/driver-manager.conf
```json
{
  "repositories": [
    {
      "name": "stable",
      "url": "https://repo.conexus-mobile-os.org/driver-repo/stable",
      "enabled": true,
      "priority": 100
    },
    {
      "name": "testing",
      "url": "https://repo.conexus-mobile-os.org/driver-repo/testing",
      "enabled": false,
      "priority": 50
    }
  ],
  "channels": {
    "default": "stable",
    "available": ["stable", "monthly", "weekly", "nightly"]
  },
  "security": {
    "verify_signatures": true,
    "require_signed": true,
    "allow_unsigned": false,
    "trusted_keys": [
      "/etc/conexus/keys/root-ca.pem",
      "/etc/conexus/keys/intermediate-ca.pem"
    ]
  },
  "installation": {
    "auto_reboot": false,
    "backup_before_install": true,
    "verify_signatures": true,
    "verify_hashes": true,
    "parallel_downloads": 4
  },
  "updates": {
    "auto_check": true,
    "check_interval_hours": 24,
    "auto_install": false,
    "channel": "stable"
  },
  "hooks": {
    "pre_install": "/etc/conexus/hooks/pre-install.sh",
    "post_install": "/etc/conexus/hooks/post-install.sh",
    "pre_remove": "/etc/conexus/hooks/pre-remove.sh",
    "post_remove": "/etc/conexus/hooks/post-remove.sh"
  }
}
```

## Hook Scripts

### Pre-install Hook (`/etc/conexus/hooks/pre-install.sh`)
```bash
#!/bin/bash
# Pre-installation checks and setup

set -e

PACKAGE="$1"
VERSION="$2"
DEVICE="$3"

echo "Pre-installing $PACKAGE $VERSION for $DEVICE"

# Check if we're in recovery mode
if [ "$(getprop ro.boot.mode)" != "recovery" ]; then
    echo "ERROR: Must be in recovery mode for installation"
    exit 1
done

# Backup current system
echo "Creating system backup..."
/system/bin/backup_system.sh --partition boot,dtbo,vbmeta,vendor_boot,super

# Check available space
AVAILABLE=$(df /data | tail -1 | awk '{print $4}')
if [ "$AVAILABLE" -lt 3145728 ]; then
    echo "ERROR: Insufficient space (need 3GB)"
    exit 1
fi

# Stop services that might interfere
stop vendor.camera-hal-2-0
stop vendor.hardware.camera.provider@2.4-service
stop vendor.hardware.camera.provider@2.5-service

echo "Pre-install checks passed"
exit 0
```

### Post-install Hook (`/etc/conexus/hooks/post-install.sh`)
```bash
#!/bin/bash
# Post-installation setup

set -e

PACKAGE="$1"
VERSION="$2"
DEVICE="$3"

echo "Post-installing $PACKAGE $VERSION for $DEVICE"

# Update module dependencies
depmod -a

# Update initramfs
update-initramfs -u -k all

# Regenerate module dependencies
depmod -a

# Update boot image
update-boot-image.sh

# Reload udev rules
udevadm control --reload-rules
udevadm trigger

# Restart services
start vendor.camera-hal-2-0
start vendor.hardware.camera.provider@2.4-service
start vendor.hardware.camera.provider@2.5-service

# Run health check
driverctl verify --package "$PACKAGE"

echo "Post-installation complete"
exit 0
```

## Security Checklist

- [ ] All packages signed with valid certificate
- [ ] Certificate chain validated to root CA
- [ ] SHA256 hashes verified for all files
- [ ] Certificate not expired or revoked
- [ ] Package manifest matches installed files
- [ ] Kernel module signatures verified
- [ ] SELinux contexts applied correctly
- [ ] File permissions and ownership correct
- [ ] SELinux contexts restored after install
- [ ] Kernel module signatures verified

## Testing Checklist

- [ ] Install on clean device
- [ ] Install on device with existing drivers
- [ ] Install with conflicts
- [ ] Upgrade from previous version
- [ ] Downgrade to previous version
- [ ] Remove package
- [ ] Install with dependencies
- [ ] Install with conflicts
- [ ] Install with missing dependencies
- [ ] Network interruption during install
- [ ] Power loss during install
- [ ] Verify rollback on failure
- [ ] Verify rollback on user cancel
- [ ] Verify post-install hooks run
- [ ] Verify pre-remove hooks run
- [ ] Verify post-remove hooks run
- [ ] Verify index updates
- [ ] Verify module loading
- [ ] Verify HAL registration
- [ ] Verify firmware loading
- [ ] Verify DTBO application
- [ ] Verify initramfs update
- [ ] Verify boot image update
- [ ] Verify OTA update path
- [ ] Verify rollback on boot failure