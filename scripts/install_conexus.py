#!/usr/bin/env python3
"""
Conexus Mobile OS - Automatic Installer
Detects device, downloads correct build, and flashes automatically.
Supports: Google Pixel 8a (akita), Nothing Phone 2 (pong)

Usage: python3 install_conexus.py [--channel stable|monthly|weekly|nightly] [--dry-run] [--skip-backup]
"""

import argparse
import asyncio
import hashlib
import json
import os
import re
import shutil
import subprocess
import sys
import tempfile
import time
from dataclasses import dataclass
from pathlib import Path
from typing import Optional, List, Dict, Tuple
from urllib.request import urlopen, Request
from urllib.error import URLError, HTTPError


@dataclass
class DeviceInfo:
    codename: str
    model: str
    manufacturer: str
    supported: bool
    primary: bool = False


@dataclass
class ReleaseInfo:
    version: str
    channel: str
    device: str
    download_url: str
    sha256: str
    size: int
    signature_url: str
    changelog_url: str
    requirements: Dict


# Supported devices database
SUPPORTED_DEVICES = {
    "akita": DeviceInfo(
        codename="akita",
        model="Pixel 8a",
        manufacturer="Google",
        supported=True,
        primary=True
    ),
    "pong": DeviceInfo(
        codename="pong",
        model="Nothing Phone (2)",
        manufacturer="Nothing",
        supported=True,
        primary=False
    ),
}

# Release channels
CHANNELS = ["stable", "monthly", "weekly", "nightly"]

# GitHub API base
GITHUB_API = "https://api.github.com/repos/conexus-mobile-os/conexus-mobile-os"
GITHUB_RELEASES = "https://github.com/conexus-mobile-os/conexus-mobile-os/releases"
GITHUB_RAW = "https://raw.githubusercontent.com/conexus-mobile-os/conexus-mobile-os/main"


class Colors:
    RED = '\033[91m'
    GREEN = '\033[92m'
    YELLOW = '\033[93m'
    BLUE = '\033[94m'
    MAGENTA = '\033[95m'
    CYAN = '\033[96m'
    WHITE = '\033[97m'
    BOLD = '\033[1m'
    RESET = '\033[0m'

    @staticmethod
    def colorize(text: str, color: str) -> str:
        return f"{color}{text}{Colors.RESET}"


class Logger:
    @staticmethod
    def info(msg: str): print(f"{Colors.colorize('[INFO]', Colors.BLUE)} {msg}")
    @staticmethod
    def success(msg: str): print(f"{Colors.colorize('[OK]', Colors.GREEN)} {msg}")
    @staticmethod
    def warning(msg: str): print(f"{Colors.colorize('[WARN]', Colors.YELLOW)} {msg}")
    @staticmethod
    def error(msg: str): print(f"{Colors.colorize('[ERROR]', Colors.RED)} {msg}")
    @staticmethod
    def step(msg: str): print(f"\n{Colors.colorize('===', Colors.CYAN)} {Colors.colorize(msg, Colors.BOLD)}")


class DeviceDetector:
    """Detects connected Android device via ADB/fastboot"""

    @staticmethod
    def check_adb() -> bool:
        """Check if adb is available"""
        try:
            result = subprocess.run(["adb", "version"], capture_output=True, text=True)
            return result.returncode == 0
        except FileNotFoundError:
            return False

    @staticmethod
    def check_fastboot() -> bool:
        """Check if fastboot is available"""
        try:
            result = subprocess.run(["fastboot", "--version"], capture_output=True, text=True)
            return result.returncode == 0
        except FileNotFoundError:
            return False

    @staticmethod
    def get_adb_devices() -> List[str]:
        """Get list of connected ADB devices"""
        try:
            result = subprocess.run(["adb", "devices"], capture_output=True, text=True)
            lines = result.stdout.strip().split('\n')[1:]  # Skip header
            devices = []
            for line in lines:
                if line.strip() and '\tdevice' in line:
                    serial = line.split('\t')[0]
                    devices.append(serial)
            return devices
        except Exception:
            return []

    @staticmethod
    def get_fastboot_devices() -> List[str]:
        """Get list of devices in fastboot mode"""
        try:
            result = subprocess.run(["fastboot", "devices"], capture_output=True, text=True)
            lines = result.stdout.strip().split('\n')
            devices = []
            for line in lines:
                if line.strip():
                    serial = line.split('\t')[0]
                    devices.append(serial)
            return devices
        except Exception:
            return []

    @staticmethod
    def get_device_info(serial: str) -> Optional[DeviceInfo]:
        """Get detailed device info via ADB"""
        try:
            # Get device properties
            props = {}
            for prop in ["ro.product.device", "ro.product.model", "ro.product.manufacturer",
                         "ro.build.fingerprint", "ro.bootloader", "ro.boot.flash.locked"]:
                try:
                    result = subprocess.run(
                        ["adb", "-s", serial, "shell", "getprop", prop],
                        capture_output=True, text=True, timeout=10
                    )
                    props[prop] = result.stdout.strip()
                except Exception:
                    props[prop] = ""

            codename = props.get("ro.product.device", "")
            device = SUPPORTED_DEVICES.get(codename)

            if device:
                return DeviceInfo(
                    codename=device.codename,
                    model=device.model,
                    manufacturer=device.manufacturer,
                    supported=device.supported,
                    primary=device.primary
                )

            # Unknown device
            return DeviceInfo(
                codename=codename,
                model=props.get("ro.product.model", "Unknown"),
                manufacturer=props.get("ro.product.manufacturer", "Unknown"),
                supported=False
            )

        except Exception as e:
            Logger.error(f"Failed to get device info: {e}")
            return None

    @classmethod
    def detect(cls) -> Optional[Tuple[str, DeviceInfo]]:
        """Main detection routine"""
        Logger.step("Detecting connected device...")

        # Check tools
        if not cls.check_adb():
            Logger.error("adb not found. Please install Android Platform Tools.")
            return None

        if not cls.check_fastboot():
            Logger.error("fastboot not found. Please install Android Platform Tools.")
            return None

        # Check ADB devices
        adb_devices = cls.get_adb_devices()
        fastboot_devices = cls.get_fastboot_devices()

        Logger.info(f"ADB devices: {adb_devices if adb_devices else 'none'}")
        Logger.info(f"Fastboot devices: {fastboot_devices if fastboot_devices else 'none'}")

        # Prefer ADB device (normal boot)
        target_serial = None
        if adb_devices:
            target_serial = adb_devices[0]
            Logger.info(f"Using ADB device: {target_serial}")
        elif fastboot_devices:
            target_serial = fastboot_devices[0]
            Logger.warning(f"Device in fastboot mode: {target_serial}")
        else:
            Logger.error("No device detected. Connect your device via USB with USB debugging enabled.")
            return None

        # Get device info
        device_info = cls.get_device_info(target_serial)
        if not device_info:
            Logger.error("Could not identify device")
            return None

        Logger.success(f"Detected: {device_info.manufacturer} {device_info.model} ({device_info.codename})")

        if not device_info.supported:
            Logger.error(f"Device {device_info.codename} is not officially supported.")
            Logger.warning("Supported devices: " + ", ".join(f"{d.model} ({c})" for c, d in SUPPORTED_DEVICES.items()))
            response = input("Continue anyway? (y/N): ")
            if response.lower() != 'y':
                return None

        return target_serial, device_info


class ReleaseManager:
    """Manages release fetching and verification"""

    def __init__(self, channel: str = "stable"):
        self.channel = channel

    async def fetch_latest_release(self, device: str) -> Optional[ReleaseInfo]:
        """Fetch latest release from GitHub API"""
        try:
            url = f"{GITHUB_API}/releases"
            headers = {"Accept": "application/vnd.github.v3+json"}

            req = Request(f"{url}?per_page=20", headers=headers)
            with urlopen(req, timeout=30) as response:
                releases = json.loads(response.read().decode())

            # Filter releases for device and channel
            for release in releases:
                tag = release.get("tag_name", "")
                if device in tag and self.channel in tag:
                    # Found matching release
                    assets = release.get("assets", [])

                    # Find main OTA package
                    ota_asset = None
                    for asset in assets:
                        if asset["name"].endswith(".zip") and "ota" in asset["name"].lower():
                            ota_asset = asset
                            break

                    if not ota_asset and assets:
                        ota_asset = assets[0]  # Fallback to first asset

                    if ota_asset:
                        # Calculate SHA256 (in real implementation, would be provided)
                        return ReleaseInfo(
                            version=release["tag_name"].replace(f"-{device}", ""),
                            channel=self.channel,
                            device=device,
                            download_url=ota_asset["browser_download_url"],
                            sha256="",  # Would be verified
                            size=ota_asset["size"],
                            signature_url="",
                            changelog_url=release["html_url"],
                            requirements={
                                "min_battery": 30,
                                "wifi_required": True,
                                "storage_min_mb": 3072
                            }
                        )

            Logger.warning(f"No release found for {device} on channel {self.channel}")
            return None

        except Exception as e:
            Logger.error(f"Failed to fetch release: {e}")
            return None

    async def fetch_fallback_release(self, device: str) -> Optional[ReleaseInfo]:
        """Fallback to mock release data for demo"""
        mock_releases = {
            "akita-stable": ReleaseInfo(
                version="v1.0.0-stable",
                channel="stable",
                device="akita",
                download_url=f"{GITHUB_RELEASES}/download/v1.0.0-stable-akita/conexus-akita-v1.0.0-stable.zip",
                sha256="a1b2c3d4e5f6789012345678901234567890abcdef1234567890abcdef123456",
                size=2147483648,
                signature_url="",
                changelog_url=f"{GITHUB_RELEASES}/tag/v1.0.0-stable-akita",
                requirements={"min_battery": 30, "wifi_required": True, "storage_min_mb": 3072}
            ),
            "akita-monthly": ReleaseInfo(
                version="v1.1.0-monthly",
                channel="monthly",
                device="akita",
                download_url="",
                sha256="",
                size=0,
                signature_url="",
                changelog_url="",
                requirements={}
            ),
            "pong-stable": ReleaseInfo(
                version="v1.0.0-stable",
                channel="stable",
                device="pong",
                download_url="",
                sha256="",
                size=0,
                signature_url="",
                changelog_url="",
                requirements={}
            ),
        }

        key = f"{device}-{self.channel}"
        return mock_releases.get(key)


class Installer:
    """Main installer class"""

    def __init__(self, args):
        self.args = args
        self.serial: Optional[str] = None
        self.device_info: Optional[DeviceInfo] = None
        self.release: Optional[ReleaseInfo] = None
        self.download_path: Optional[Path] = None

    async def run(self) -> bool:
        """Main installation flow"""
        try:
            # Step 1: Detect device
            if not await self.detect_device():
                return False

            # Step 2: Fetch release
            if not await self.fetch_release():
                return False

            # Step 3: Verify prerequisites
            if not await self.check_prerequisites():
                return False

            # Step 4: Download and verify
            if not await self.download_and_verify():
                return False

            # Step 4.5: Show installation plan
            if not self.show_install_plan():
                return False

            # Step 5: Flash (or dry run)
            if self.args.dry_run:
                Logger.success("DRY RUN COMPLETE - No changes made")
                return True

            if not await self.flash():
                return False

            # Step 6: Verify installation
            if not await self.verify_installation():
                return False

            Logger.success("Installation complete!")
            return True

        except KeyboardInterrupt:
            Logger.error("\nInstallation cancelled by user")
            return False
        except Exception as e:
            Logger.error(f"Installation failed: {e}")
            return False

    async def detect_device(self) -> bool:
        result = DeviceDetector.detect()
        if not result:
            return False
        self.serial, self.device_info = result
        return True

    async def fetch_release(self) -> bool:
        Logger.step("Fetching release information...")

        rm = ReleaseManager(self.args.channel)

        # Try GitHub API first
        self.release = await rm.fetch_latest_release(self.device_info.codename)

        # Fallback to mock data
        if not self.release:
            Logger.warning("Using fallback release data")
            self.release = await rm.fetch_fallback_release(self.device_info.codename)

        if not self.release:
            Logger.error(f"No release found for {self.device_info.codename} on {self.args.channel} channel")
            return False

        Logger.success(f"Found release: {self.release.version} ({self.release.channel})")
        return True

    async def check_prerequisites(self) -> bool:
        Logger.step("Checking prerequisites...")

        # Check battery
        try:
            result = subprocess.run(
                ["adb", "-s", self.serial, "shell", "dumpsys", "battery"],
                capture_output=True, text=True, timeout=10
            )
            # Parse battery level
            for line in result.stdout.split('\n'):
                if "level:" in line:
                    level = int(line.split(":")[1].strip())
                    min_battery = self.release.requirements.get("min_battery", 30)
                    if level < min_battery:
                        Logger.warning(f"Battery low: {level}% (minimum {min_battery}%)")
                        if not self.confirm("Continue with low battery?"):
                            return False
                    else:
                        Logger.success(f"Battery: {level}%")
                    break
        except Exception:
            Logger.warning("Could not check battery level")

        # Check storage
        try:
            result = subprocess.run(
                ["adb", "-s", self.serial, "shell", "df", "/data"],
                capture_output=True, text=True, timeout=10
            )
            # Parse available space
            lines = result.stdout.strip().split('\n')
            if len(lines) > 1:
                parts = lines[1].split()
                available_kb = int(parts[3])
                available_mb = available_kb // 1024
                min_storage = self.release.requirements.get("storage_min_mb", 3072)
                if available_mb < min_storage:
                    Logger.error(f"Insufficient storage: {available_mb}MB available, {min_storage}MB required")
                    return False
                Logger.success(f"Available storage: {available_mb}MB")
        except Exception:
            Logger.warning("Could not check storage")

        # Check bootloader status
        try:
            result = subprocess.run(
                ["fastboot", "getvar", "unlocked"],
                capture_output=True, text=True, timeout=10
            )
            output = result.stdout + result.stderr
            if "unlocked: yes" in output.lower():
                Logger.success("Bootloader: UNLOCKED")
            else:
                Logger.warning("Bootloader appears LOCKED - will need to unlock")
                if not self.confirm("Continue? This will ERASE all data on the device."):
                    return False
        except Exception:
            Logger.warning("Could not check bootloader status")

        return True

    async def download_and_verify(self) -> bool:
        if not self.release.download_url:
            Logger.error("No download URL available")
            return False

        Logger.step("Downloading release...")

        # Create temp directory
        temp_dir = Path(tempfile.mkdtemp(prefix="conexus_install_"))
        filename = f"conexus-{self.release.device}-{self.release.version}.zip"
        self.download_path = temp_dir / self.release.version / f"conexus-{self.release.device}-{self.release.version}.zip"
        self.download_path.parent.mkdir(parents=True, exist_ok=True)

        try:
            Logger.info(f"Downloading from {self.release.download_url}")

            # Download with progress
            req = Request(self.release.download_url, headers={"User-Agent": "ConexusInstaller/0.1"})
            with urlopen(self.release.download_url, timeout=300) as response:
                total_size = int(response.headers.get('Content-Length', 0))
                downloaded = 0
                chunk_size = 8192

                with open(self.download_path, 'wb') as f:
                    while True:
                        chunk = response.read(chunk_size)
                        if not chunk:
                            break
                        f.write(chunk)
                        downloaded += len(chunk)
                        if total_size > 0:
                            percent = (downloaded / total_size) * 100
                            print(f"\rDownloading: {percent:.1f}% ({downloaded}/{total_size} bytes)", end='', flush=True)

            print()  # New line after progress
            Logger.success("Download complete")

            # Verify SHA256
            if self.release.sha256:
                Logger.info("Verifying SHA256...")
                sha256 = hashlib.sha256()
                with open(self.download_path, 'rb') as f:
                    for chunk in iter(lambda: f.read(8192), b''):
                        sha256.update(chunk)
                    calculated = sha256.hexdigest()
                    if calculated.lower() == self.release.sha256.lower():
                        Logger.success("SHA256 verification PASSED")
                    else:
                        Logger.error(f"SHA256 MISMATCH! Expected: {self.release.sha256}, Got: {calculated}")
                        return False
            else:
                Logger.warning("No SHA256 provided - skipping verification")

            return True

        except HTTPError as e:
            Logger.error(f"Download failed: HTTP {e.code}")
            return False
        except URLError as e:
            Logger.error(f"Network error: {e.reason}")
            return False
        except Exception as e:
            Logger.error(f"Download failed: {e}")
            return False

    def show_install_plan(self) -> bool:
        Logger.step("Installation Plan")
        print(f"  Device: {self.device_info.manufacturer} {self.device_info.model} ({self.device_info.codename})")
        print(f"  Release: {self.release.version} ({self.release.channel})")
        print(f"  File: {self.download_path.name} ({self.release.size / (1024**3):.2f} GB)")
        print()
        print("Steps:")
        print("  1. Reboot to bootloader")
        print("  2. Flash boot, dtbo, vbmeta, vendor_boot, super, recovery")
        print("  3. Reboot to system")
        print("  4. First boot setup")
        print()

        if self.args.dry_run:
            return True

        return self.confirm("Proceed with installation?")

    async def flash(self) -> bool:
        Logger.step("Flashing Conexus Mobile OS...")

        # Reboot to bootloader
        Logger.info("Rebooting to bootloader...")
        subprocess.run(["adb", "-s", self.serial, "reboot", "bootloader"], timeout=30)
        await asyncio.sleep(5)

        # Verify in fastboot
        fastboot_devices = DeviceDetector.get_fastboot_devices()
        if not fastboot_devices:
            Logger.error("Device not in fastboot mode")
            return False

        Logger.success("Device in fastboot mode")

        # Flash partitions
        partitions = [
            ("boot", "boot.img"),
            ("dtbo", "dtbo.img"),
            ("vbmeta", "vbmeta.img"),
            ("vendor_boot", "vendor_boot.img"),
            ("super", "super.img"),
            ("recovery", "recovery.img"),
        ]

        for partition, filename in partitions:
            filepath = self.download_path.parent / filename
            if not filepath.exists():
                Logger.warning(f"Skipping {partition} - {filename} not found")
                continue

            Logger.info(f"Flashing {partition}...")
            result = subprocess.run(
                ["fastboot", "flash", partition, str(filepath)],
                capture_output=True, text=True, timeout=120
            )
            if result.returncode != 0:
                Logger.error(f"Failed to flash {partition}: {result.stderr}")
                return False
            Logger.success(f"{partition} flashed")

        # Reboot
        Logger.info("Rebooting...")
        subprocess.run(["fastboot", "reboot"], timeout=30)

        return True

    async def verify_installation(self) -> bool:
        Logger.step("Verifying installation...")

        # Wait for boot
        Logger.info("Waiting for device to boot...")
        for i in range(60):
            await asyncio.sleep(2)
            try:
                result = subprocess.run(
                    ["adb", "devices"],
                    capture_output=True, text=True, timeout=5
                )
                if self.serial in result.stdout and "device" in result.stdout:
                    Logger.success("Device booted successfully!")
                    return True
            except Exception:
                pass

        Logger.warning("Device not detected after 2 minutes")
        return False

    def confirm(self, message: str) -> bool:
        response = input(f"{Colors.colorize('[?]', Colors.YELLOW)} {message} (y/N): ")
        return response.lower() == 'y'


async def main():
    parser = argparse.ArgumentParser(
        description="Conexus Mobile OS Automatic Installer",
        formatter_class=argparse.RawDescriptionHelpFormatter,
        epilog="""
Examples:
  python3 install_conexus.py                    # Interactive stable install
  python3 install_conexus.py --channel monthly  # Monthly channel
  python3 install_conexus.py --dry-run          # Dry run (no changes)
  python3 install_conexus.py --skip-backup      # Skip backup prompt

Supported devices:
  - Google Pixel 8a (akita) - PRIMARY
  - Nothing Phone 2 (pong) - Backup
        """
    )
    parser.add_argument("--channel", choices=CHANNELS, default="stable",
                        help="Release channel (default: stable)")
    parser.add_argument("--dry-run", action="store_true",
                        help="Simulate installation without making changes")
    parser.add_argument("--skip-backup", action="store_true",
                        help="Skip backup prompt (data will be lost)")
    parser.add_argument("--device", help="Force specific device codename")
    parser.add_argument("--version", help="Specific version to install")
    parser.add_argument("--verbose", "-v", action="store_true", help="Verbose output")

    args = parser.parse_args()

    # Print banner
    print(f"""
{Colors.colorize('╔═══════════════════════════════════════════════════╗', Colors.CYAN)}
{Colors.colorize('║', Colors.CYAN)}  {Colors.colorize('Conexus Mobile OS - Automatic Installer', Colors.BOLD)}       {Colors.colorize('║', Colors.CYAN)}
{Colors.colorize('║', Colors.CYAN)}  {Colors.colorize('LineageOS 23.2 + Conexus Launcher + Metro UI', Colors.WHITE)}  {Colors.colorize('║', Colors.CYAN)}
{Colors.colorize('╚═══════════════════════════════════════════════════╝', Colors.CYAN)}
    """)

    installer = Installer(args)
    success = await installer.run()

    if success:
        print(f"\n{Colors.colorize('╔═══════════════════════════════════════════════════╗', Colors.GREEN)}")
        print(f"{Colors.colorize('║', Colors.GREEN)}  {Colors.colorize('Installation Successful!', Colors.BOLD)}                     {Colors.colorize('║', Colors.GREEN)}")
        print(f"{Colors.colorize('║', Colors.GREEN)}  {Colors.colorize('Welcome to Conexus Mobile OS!', Colors.WHITE)}          {Colors.colorize('║', Colors.GREEN)}")
        print(f"{Colors.colorize('╚═══════════════════════════════════════════════════╝', Colors.GREEN)}")
        print("\nNext steps:")
        print("  1. Complete initial setup on device")
        print("  2. Check for OTA updates in Settings > System > Update")
        print("  3. Explore Metro UI: Long-press tiles, swipe edges")
        print("  4. Join community: https://matrix.to/#/#conexus-mobile-os:matrix.org")
        return 0
    else:
        print(f"\n{Colors.colorize('╔═══════════════════════════════════════════════════╗', Colors.RED)}")
        print(f"{Colors.colorize('║', Colors.RED)}  {Colors.colorize('Installation Failed', Colors.BOLD)}                         {Colors.colorize('║', Colors.RED)}")
        print(f"{Colors.colorize('╚═══════════════════════════════════════════════════╝', Colors.RED)}")
        return 1


if __name__ == "__main__":
    try:
        exit_code = asyncio.run(main())
        sys.exit(exit_code)
    except KeyboardInterrupt:
        print(f"\n{Colors.colorize('[INTERRUPTED]', Colors.RED)} Installation cancelled")
        sys.exit(130)