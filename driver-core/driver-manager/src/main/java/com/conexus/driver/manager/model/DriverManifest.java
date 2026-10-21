package com.conexus.driver.manager.model;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import java.util.*;

/**
 * Driver manifest containing package metadata
 */
public class DriverManifest {
    private final String formatVersion;
    private final PackageInfo packageInfo;
    private final KernelManifest kernel;
    private final HalManifest hal;
    private final FirmwareManifest firmware;
    private final BlobsManifest blobs;
    private final Map<String, String> dependencies;
    private final List<String> conflicts;
    private final List<String> replaces;
    private final List<String> provides;
    private final Map<String, Object> scripts;
    private final Map<String, String> signatures;
    private final Map<String, Object> compatibility;

    public DriverManifest() {}

    // Getters
    public String getFormatVersion() { return formatVersion; }
    public PackageInfo getPackageInfo() { return packageInfo; }
    public KernelManifest getKernel() { return kernel; }
    public HalManifest getHal() { return hal; }
    public FirmwareManifest getFirmware() { return firmware; }
    public BlobsManifest getBlobs() { return blobs; }
    public Map<String, String> getDependencies() { return dependencies; }
    public List<String> getConflicts() { return conflicts; }
    public List<String> getReplaces() { return replaces; }
    public List<String> getProvides() { return provides; }
    public Map<String, Object> getScripts() { return scripts; }
    public Map<String, String> getSignatures() { return signatures; }
    public Map<String, Object> getCompatibility() { return compatibility; }
}

/** Kernel manifest section */
class KernelManifest {
    private String minVersion;
    private String maxVersion;
    private List<String> requiredConfig;
    private List<KernelModule> modules;
    private List<Dtbo> dtbos;
    private String config;

    // Getters and setters...
}

/** Kernel module entry */
class KernelModule {
    private String name;
    private String path;
    private String license;
    private String description;
    private String version;
    private String srcVersion;
    private List<String> depends;
    private String vermagic;
}

/** DTBO entry */
class Dtbo {
    private String name;
    private String path;
    private String compatible;
    private int version;
}

/** HAL Manifest */
class HalManifest {
    private Map<String, HalComponent> components;

    public HalComponent getCamera() { return components.get("camera"); }
    public HalComponent getDisplay() { return components.get("display"); }
    // ... other HAL components
}

/** HAL Component */
class HalComponent {
    private String version;
    private String path;
    private List<String> interfaces;
}

/** Firmware Manifest */
class FirmwareManifest {
    private Map<String, List<String>> firmwares;
}

/** Blobs Manifest */
class BlobsManifest {
    private Map<String, Map<String, String>> blobs;
}