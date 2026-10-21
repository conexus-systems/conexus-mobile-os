package com.conexus.driver.manager.model;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import java.util.*;

/**
 * Package information from repository
 */
public class PackageInfo {
    private final String name;
    private final String version;
    private final String release;
    private final int epoch;
    private final String architecture;
    private final String vendor;
    private final List<String> devices;
    private final String description;
    private final String maintainer;
    private final String license;
    private final String homepage;
    private final String source;
    private final long size;
    private final String sha256;
    private final String downloadUrl;
    private final String signatureUrl;
    private final String changelogUrl;
    private final Map<String, Object> kernel;
    private final Map<String, Object> hal;
    private final Map<String, Object> firmware;
    private final Map<String, Object> blobs;
    private final Map<String, String> dependencies;
    private final List<String> conflicts;
    private final List<String> replaces;
    private final List<String> provides;
    private final Map<String, Object> scripts;
    private final Map<String, String> signatures;
    private final String releaseDate;
    private final long size;

    // Constructor, getters...
    public PackageInfo() {}

    // Getters
    public String getName() { return name; }
    public String getVersion() { return version; }
    public String getRelease() { return release; }
    public int getEpoch() { return epoch; }
    public String getArchitecture() { return architecture; }
    public String getVendor() { return vendor; }
    public List<String> getDevices() { return devices; }
    public String getDescription() { return description; }
    public String getMaintainer() { return maintainer; }
    public String getLicense() { return license; }
    public String getHomepage() { return homepage; }
    public String getSource() { return source; }
    public long getSize() { return size; }
    public String getSha256() { return sha256; }
    public String getDownloadUrl() { return downloadUrl; }
    public String getSignatureUrl() { return signatureUrl; }
    public String getChangelogUrl() { return changelogUrl; }
    public Map<String, Object> getKernel() { return kernel; }
    public Map<String, Object> getHal() { return hal; }
    public Map<String, Object> getFirmware() { return firmware; }
    public Map<String, Object> getBlobs() { return blobs; }
    public Map<String, String> getDependencies() { return dependencies; }
    public List<String> getConflicts() { return conflicts; }
    public List<String> getReplaces() { return replaces; }
    public List<String> getProvides() { return provides; }
    public String getReleaseDate() { return releaseDate; }
    public long getSize() { return size; }
}