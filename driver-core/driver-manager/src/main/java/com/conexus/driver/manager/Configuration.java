package com.conexus.driver.manager;

import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;

/**
 * Configuration for Driver Package Manager
 */
public class Configuration {
    private final Map<String, Object> properties = new HashMap<>();

    private Configuration() {}

    @NonNull
    public static Configuration defaultConfig() {
        Configuration config = new Configuration();
        config.set("repositories", Collections.emptyList());
        config.set("channel", "stable");
        config.set("verify_signatures", true);
        config.set("check_hashes", true);
        config.set("check_permissions", true);
        config.set("auto_reboot", false);
        config.set("backup_before_install", true);
        config.set("parallel_downloads", 4);
        config.set("auto_check_updates", true);
        config.set("check_interval_hours", 24);
        config.set("install_timeout_seconds", 300);
        config.set("download_timeout_seconds", 300);
        config.set("temp_dir", "/data/local/tmp/conexus-install");
        config.set("install_root", "/system/driver");
        config.set("index_path", "/data/conexus/driver-index.json");
        config.set("log_level", "INFO");
        return config;
    }

    @Nullable
    public Object get(@NonNull String key) {
        return properties.get(key);
    }

    public void set(@NonNull String key, @NonNull Object value) {
        properties.put(key, value);
    }

    @SuppressWarnings("unchecked")
    @NonNull
    public <T> T get(@NonNull String key, @NonNull T defaultValue) {
        Object value = properties.get(key);
        if (value == null) return defaultValue;
        try {
            return (T) value;
        } catch (ClassCastException e) {
            return defaultValue;
        }
    }

    public boolean getBoolean(@NonNull String key, boolean defaultValue) {
        Object value = properties.get(key);
        if (value instanceof Boolean) return (Boolean) value;
        if (value instanceof String) return Boolean.parseBoolean((String) value);
        return defaultValue;
    }

    public int getInt(@NonNull String key, int defaultValue) {
        Object value = properties.get(key);
        if (value instanceof Integer) return (Integer) value;
        if (value instanceof String) {
            try { return Integer.parseInt((String) value); }
            catch (NumberFormatException e) { return defaultValue; }
        }
        if (value instanceof Number) return ((Number) value).intValue();
        return defaultValue;
    }

    public long getLong(@NonNull String key, long defaultValue) {
        Object value = properties.get(key);
        if (value instanceof Long) return (Long) value;
        if (value instanceof String) {
            try { return Long.parseLong((String) value); }
            catch (NumberFormatException e) { return defaultValue; }
        }
        if (value instanceof Number) return ((Number) value).longValue();
        return defaultValue;
    }

    @NonNull
    public String getString(@NonNull String key, @NonNull String defaultValue) {
        Object value = properties.get(key);
        if (value instanceof String) return (String) value;
        return defaultValue;
    }

    @SuppressWarnings("unchecked")
    @NonNull
    public <T> List<T> getList(@NonNull String key, @NonNull Class<T> type) {
        Object value = properties.get(key);
        if (value instanceof List) {
            return (List<T>) value;
        }
        return Collections.emptyList();
    }

    @SuppressWarnings("unchecked")
    @NonNull
    public Map<String, Object> getMap(@NonNull String key) {
        Object value = properties.get(key);
        if (value instanceof Map) {
            return (Map<String, Object>) value;
        }
        return Collections.emptyMap();
    }

    @NonNull
    public List<RepositoryConfig> getRepositories() {
        List<?> repos = getList("repositories", Map.class);
        List<RepositoryConfig> result = new ArrayList<>();
        for (Object obj : repos) {
            if (obj instanceof Map) {
                result.add(RepositoryConfig.fromMap((Map<String, Object>) obj));
            }
        }
        return result;
    }

    @NonNull
    public static Configuration parse(@NonNull String content) {
        Configuration config = new Configuration();
        try {
            // Parse JSON
            // For simplicity, using basic parsing
            // In real implementation, use Jackson or Gson
        } catch (Exception e) {
            // Return default
        }
        return defaultConfig();
    }

    @NonNull
    public String toJson() {
        // Convert to JSON
        return "{}";
    }

    public void save(@NonNull Path path) throws IOException {
        Files.write(path, toJson().getBytes());
    }
}

/**
 * Repository Configuration
 */
public class RepositoryConfig {
    private String name;
    private String url;
    private String channel = "stable";
    private int priority = 100;
    private boolean enabled = true;

    public RepositoryConfig() {}

    @NonNull
    public String getName() { return name; }
    public void setName(String name) { this.name = name; }

    @NonNull
    public String getUrl() { return url; }
    public void setUrl(String url) { this.url = url; }

    @NonNull
    public String getChannel() { return channel; }
    public void setChannel(String channel) { this.channel = channel; }

    public int getPriority() { return priority; }
    public void setPriority(int priority) { this.priority = priority; }

    public boolean isEnabled() { return enabled; }
    public void setEnabled(boolean enabled) { this.enabled = enabled; }

    @NonNull
    public static RepositoryConfig fromMap(@NonNull Map<String, Object> map) {
        RepositoryConfig config = new RepositoryConfig();
        config.name = (String) map.get("name");
        config.url = (String) map.get("url");
        config.channel = (String) map.getOrDefault("channel", "stable");
        config.priority = (int) map.getOrDefault("priority", 100);
        config.enabled = (boolean) map.getOrDefault("enabled", true);
        return config;
    }
}

/**
 * Driver Exception
 */
public class DriverException extends Exception {
    private final String errorCode;

    public DriverException(@NonNull String message) {
        super(message);
        this.errorCode = "DRIVER_ERROR";
    }

    public DriverException(@NonNull String message, @NonNull Throwable cause) {
        super(message, cause);
        this.errorCode = "DRIVER_ERROR";
    }

    public DriverException(@NonNull String message, @NonNull String errorCode) {
        super(message);
        this.errorCode = errorCode;
    }

    @NonNull
    public String getErrorCode() { return errorCode; }
}