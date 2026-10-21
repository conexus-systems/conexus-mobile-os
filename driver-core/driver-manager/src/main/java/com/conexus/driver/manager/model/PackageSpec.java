package com.conexus.driver.manager.model;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import java.util.*;

/**
 * Package specification for installation requests
 */
public class PackageSpec {
    private final String name;
    private String version;
    private String channel;
    private String architecture;

    public PackageSpec(@NonNull String name) {
        this.name = name;
        this.channel = "stable";
    }

    public static PackageSpec parse(@NonNull String spec) {
        // Format: name[=version][@channel][:arch]
        String[] parts = spec.split("@");
        String nameVersion = parts[0];
        String channel = parts.length > 1 ? parts[1] : "stable";

        String[] nv = nameVersion.split("=");
        String name = nv[0];
        String version = nv.length > 1 ? nv[1] : null;

        PackageSpec spec = new PackageSpec(name);
        spec.version = version;
        spec.channel = channel;
        return spec;
    }

    public PackageSpec(@NonNull String name) {
        this.name = name;
    }

    @NonNull
    public String getName() { return name; }

    @Nullable
    public String getVersion() { return version; }

    public void setVersion(@Nullable String version) { this.version = version; }

    @NonNull
    public String getChannel() { return channel; }

    public void setChannel(@NonNull String channel) { this.channel = channel; }

    @Nullable
    public String getArchitecture() { return architecture; }

    public void setArchitecture(@Nullable String architecture) { this.architecture = architecture; }

    @Override
    public String toString() {
        StringBuilder sb = new StringBuilder(name);
        if (version != null) sb.append("=").append(version);
        sb.append("@").append(channel);
        if (architecture != null) sb.append(":").append(architecture);
        return sb.toString();
    }
}