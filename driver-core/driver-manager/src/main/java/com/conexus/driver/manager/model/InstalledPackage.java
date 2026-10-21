package com.conexus.driver.manager.model;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import java.util.*;

/**
 * Information about an installed driver package
 */
public class InstalledPackage {
    private final String name;
    private final String version;
    private final int epoch;
    private final String architecture;
    private final String installDate;
    private final String installPath;
    private final String channel;
    private final String status;
    private final List<String> installedFiles;
    private final Map<String, String> fileHashes;
    private final Map<String, Object> metadata;

    public InstalledPackage() {}

    // Getters
    public String getName() { return name; }
    public String getVersion() { return version; }
    public int getEpoch() { return epoch; }
    public String getArchitecture() { return architecture; }
    public String getInstallDate() { return installDate; }
    public String getInstallPath() { return installPath; }
    public String getChannel() { return channel; }
    public String getStatus() { return status; }
    public List<String> getInstalledFiles() { return installedFiles; }
    public Map<String, String> getFileHashes() { return fileHashes; }
    public Map<String, Object> getMetadata() { return metadata; }
}