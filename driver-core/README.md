# Conexus Driver Core - Android Hardware Abstraction Layer

Universal driver framework for Android-based systems supporting multiple vendors and kernel versions.

## Architecture

```
driver-core/
├── core/                    # Core abstractions & interfaces
├── vendors/                 # Vendor-specific implementations
│   ├── google/             # Pixel devices (akita, cheetah, panther, etc.)
│   ├── samsung/            # Galaxy devices (exynos, snapdragon)
│   ├── nothing/            # Nothing Phone (1, 2, 2a)
│   ├── oneplus/            # OnePlus devices
│   ├── xiaomi/             # Xiaomi/Redmi/POCO devices
│   └── generic/            # Fallback/generic implementations
├── kernel/                  # Kernel version abstractions
│   ├── common/             # Common kernel interfaces
│   ├── 4.19/               # Android 10/11 kernels
│   ├── 5.4/                # Android 11/12 kernels
│   ├── 5.10/               # Android 12/13 kernels
│   ├── 5.15/               # Android 13/14 kernels
│   ├── 6.1/                # Android 14/15 kernels (GKI)
│   └── 6.6/                # Android 15+ kernels
├── hal/                     # Hardware Abstraction Layer implementations
│   ├── audio/              # Audio HAL (primary, usb, bluetooth)
│   ├── camera/             # Camera HAL (camera2, camera3)
│   ├── display/            # Display HAL (HWC, DRM, HWC2)
│   ├── gps/                # GNSS HAL
│   ├── sensors/            # Sensors HAL (accel, gyro, light, proximity)
│   ├── wifi/               # WiFi HAL (legacy, aidl)
│   ├── bluetooth/          # Bluetooth HAL
│   ├── nfc/                # NFC HAL
│   ├── power/              # Power HAL (hint, mode, boost)
│   ├── thermal/            # Thermal HAL (throttling, monitoring)
│   ├── vibrator/           # Vibrator HAL (haptics)
│   ├── usb/                # USB HAL (gadget, host, accessory)
│   └── common/             # Shared HAL utilities
├── blobs/                   # Vendor proprietary blobs management
├── build/                   # Build system integration
├── configs/                 # Device configurations
├── tools/                   # Development & debugging tools
└── docs/                    # Documentation
```

## Supported Vendors & Devices

| Vendor | Devices | Kernel | Status |
|--------|---------|--------|--------|
| **Google** | Pixel 6/7/8/8a/9 series | 5.10/5.15/6.1/6.6 | ✅ Full |
| **Samsung** | Galaxy S21/22/23/24, Z Fold/Flip | 5.4/5.10/5.15 | ⚠️ Partial |
| **Nothing** | Phone 1/2/2a | 5.15/6.1 | ✅ Good |
| **OnePlus** | 10/11/12/13 series | 5.10/5.15/6.1 | ⚠️ Partial |
| **Xiaomi** | 12/13/14 series, POCO | 5.10/5.15/6.1 | ⚠️ Partial |

## Kernel Support Matrix

| Android | Kernel | GKI | Support |
|---------|--------|-----|---------|
| 10 | 4.19 | No | Legacy |
| 11 | 5.4 | No | Legacy |
| 12 | 5.10 | Yes | Supported |
| 13 | 5.15 | Yes | Supported |
| 14 | 6.1 | Yes | Supported |
| 15 | 6.6 | Yes | Current |

## Quick Start

```bash
# Initialize for a device
./tools/init_device.sh google akita

# Build for device
./build/build.sh akita

# Flash to device
./tools/flash_device.sh akita
```

## Architecture Principles

1. **Abstraction First** - Hardware differences hidden behind interfaces
2. **Vendor Isolation** - Vendor code isolated, never mixed
3. **Kernel Agnostic** - Same HAL works across kernel versions
4. **Blob Management** - Proprietary blobs tracked, verified, updated
5. **Upstream First** - Prefer mainline kernel, minimize patches
6. **Security First** - Verified boot, SELinux, encrypted storage

## License

Apache-2.0 (compatible with AOSP/LineageOS)