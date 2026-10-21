// Conexus Driver Core - Core Abstractions
// Hardware abstraction interfaces for Android-based systems

package com.conexus.driver.core;

import android.annotation.NonNull;
import android.annotation.Nullable;
import android.hardware.hidl.HidlSupport;
import android.os.IBinder;
import android.os.RemoteException;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;

/**
 * Base interface for all hardware components.
 * Every hardware component must implement this interface.
 */
public interface IHardwareComponent {
    /** Unique component identifier */
    @NonNull String getComponentId();

    /** Component type/category */
    @NonNull ComponentType getType();

    /** Current component state */
    @NonNull ComponentState getState();

    /** Initialize component */
    void initialize(@NonNull ComponentContext context) throws HardwareException;

    /** Shutdown component */
    void shutdown() throws HardwareException;

    /** Get component capabilities */
    @NonNull ComponentCapabilities getCapabilities();

    /** Health check */
    @NonNull HealthStatus performHealthCheck();

    /** Get diagnostic information */
    @NonNull Map<String, Object> getDiagnostics();
}

/** Component types */
public enum ComponentType {
    CAMERA,
    DISPLAY,
    AUDIO,
    SENSORS,
    GPS,
    WIFI,
    BLUETOOTH,
    NFC,
    USB,
    VIBRATOR,
    POWER,
    THERMAL,
    STORAGE,
    MEMORY,
    CPU,
    GPU,
    NPU,
    ISP,
    MODEM,
    FINGERPRINT,
    FACE_ID,
    IR_BLASTER,
    LIGHT,
    PROXIMITY,
    ACCELEROMETER,
    GYROSCOPE,
    MAGNETOMETER,
    BAROMETER,
    HUMIDITY,
    TEMPERATURE,
    CUSTOM
}

/** Component states */
public enum ComponentState {
    UNKNOWN,
    INITIALIZING,
    READY,
    ACTIVE,
    IDLE,
    SUSPENDED,
    ERROR,
    SHUTDOWN
}

/** Component capabilities */
public class ComponentCapabilities {
    private final Map<String, Object> capabilities;

    public ComponentCapabilities(@NonNull Map<String, Object> capabilities) {
        this.capabilities = Map.copyOf(capabilities);
    }

    public boolean hasCapability(@NonNull String name) {
        return capabilities.containsKey(name);
    }

    @Nullable
    public Object getCapability(@NonNull String name) {
        return capabilities.get(name);
    }

    @NonNull
    public <T> T getCapability(@NonNull String name, @NonNull Class<T> type) {
        Object value = capabilities.get(name);
        if (value == null) throw new IllegalArgumentException("Capability not found: " + name);
        if (!type.isInstance(value)) throw new IllegalArgumentException("Type mismatch for: " + name);
        return type.cast(value);
    }
}

/** Component context for initialization */
public class ComponentContext {
    private final Map<String, Object> properties;
    private final Map<String, IBinder> services;

    public ComponentContext(@NonNull Map<String, Object> properties,
                           @NonNull Map<String, IBinder> services) {
        this.properties = Map.copyOf(properties);
        this.services = Map.copyOf(services);
    }

    @Nullable
    public Object getProperty(@NonNull String key) {
        return properties.get(key);
    }

    @Nullable
    public <T> T getProperty(@NonNull String key, @NonNull Class<T> type) {
        Object value = properties.get(key);
        if (value == null) return null;
        if (!type.isInstance(value)) throw new IllegalArgumentException("Type mismatch");
        return type.cast(value);
    }

    @Nullable
    public IBinder getService(@NonNull String name) {
        return services.get(name);
    }
}

/** Component health status */
public class HealthStatus {
    private final boolean healthy;
    private final int score; // 0-100
    private final List<String> issues;
    private final Map<String, Object> metrics;

    public HealthStatus(boolean healthy, int score, @NonNull List<String> issues,
                       @NonNull Map<String, Object> metrics) {
        this.healthy = healthy;
        this.score = Math.max(0, Math.min(100, score));
        this.issues = List.copyOf(issues);
        this.metrics = Map.copyOf(metrics);
    }

    public boolean isHealthy() { return healthy; }
    public int getScore() { return score; }
    @NonNull public List<String> getIssues() { return issues; }
    @NonNull public Map<String, Object> getMetrics() { return metrics; }
}

/** Hardware exceptions */
public class HardwareException extends Exception {
    private final String componentId;
    private final ErrorCode errorCode;

    public HardwareException(@NonNull String componentId, @NonNull ErrorCode errorCode,
                            @NonNull String message) {
        super(message);
        this.componentId = componentId;
        this.errorCode = errorCode;
    }

    public HardwareException(@NonNull String componentId, @NonNull ErrorCode errorCode,
                            @NonNull String message, @Nullable Throwable cause) {
        super(message, cause);
        this.componentId = componentId;
        this.errorCode = errorCode;
    }

    @NonNull public String getComponentId() { return componentId; }
    @NonNull public ErrorCode getErrorCode() { return errorCode; }
}

/** Hardware error codes */
public enum ErrorCode {
    INITIALIZATION_FAILED,
    HARDWARE_NOT_FOUND,
    HARDWARE_BUSY,
    PERMISSION_DENIED,
    UNSUPPORTED_OPERATION,
    CONFIGURATION_ERROR,
    COMMUNICATION_ERROR,
    TIMEOUT,
    OVERHEATING,
    POWER_LOSS,
    FIRMWARE_ERROR,
    DRIVER_ERROR,
    UNKNOWN
}

/**
 * Hardware registry - manages all hardware components
 */
public interface IHardwareRegistry {
    /** Register a hardware component */
    void registerComponent(@NonNull IHardwareComponent component);

    /** Unregister a hardware component */
    void unregisterComponent(@NonNull String componentId);

    /** Get component by ID */
    @Nullable IHardwareComponent getComponent(@NonNull String componentId);

    /** Get all components of a type */
    @NonNull List<IHardwareComponent> getComponentsByType(@NonNull ComponentType type);

    /** Get all registered components */
    @NonNull List<IHardwareComponent> getAllComponents();

    /** Initialize all components */
    void initializeAll(@NonNull ComponentContext context) throws HardwareException;

    /** Shutdown all components */
    void shutdownAll() throws HardwareException;

    /** Perform health check on all components */
    @NonNull Map<String, HealthStatus> healthCheckAll();

    /** Get component asynchronously */
    @NonNull CompletableFuture<IHardwareComponent> getComponentAsync(@NonNull String componentId);
}

/**
 * Hardware abstraction layer interface
 * Provides vendor-agnostic access to hardware
 */
public interface IHardwareAbstractionLayer {
    /** Get camera HAL */
    @NonNull ICameraHal getCameraHal();

    /** Get display HAL */
    @NonNull IDisplayHal getDisplayHal();

    /** Get audio HAL */
    @NonNull IAudioHal getAudioHal();

    /** Get sensors HAL */
    @NonNull ISensorsHal getSensorsHal();

    /** Get GPS HAL */
    @NonNull IGpsHal getGpsHal();

    /** Get WiFi HAL */
    @NonNull IWifiHal getWifiHal();

    /** Get Bluetooth HAL */
    @NonNull IBluetoothHal getBluetoothHal();

    /** Get NFC HAL */
    @NonNull INfcHal getNfcHal();

    /** Get power HAL */
    @NonNull IPowerHal getPowerHal();

    /** Get thermal HAL */
    @NonNull IThermalHal getThermalHal();

    /** Get vibrator HAL */
    @NonNull IVibratorHal getVibratorHal();

    /** Get USB HAL */
    @NonNull IUsbHal getUsbHal();

    /** Get power stats */
    @NonNull PowerStats getPowerStats();

    /** Get thermal stats */
    @NonNull ThermalStats getThermalStats();

    /** Get system info */
    @NonNull SystemInfo getSystemInfo();
}

/** System information */
public class SystemInfo {
    private final String deviceModel;
    private final String deviceCodename;
    private final String manufacturer;
    private final String androidVersion;
    private final String kernelVersion;
    private final String buildFingerprint;
    private final String securityPatchLevel;
    private final String bootloaderVersion;
    private final String basebandVersion;
    private final long totalMemory;
    private final long availableMemory;
    private final String cpuArchitecture;
    private final String gpuRenderer;
    private final String gpuVersion;

    // Constructor, getters...
}

/** Power statistics */
public class PowerStats {
    private final int batteryLevel;
    private final boolean isCharging;
    private final long batteryCapacity;
    private final long currentNow;
    private final long voltageNow;
    private final Map<String, Long> uidPowerConsumption;

    // Constructor, getters...
}

/** Thermal statistics */
public class ThermalStats {
    private final List<ThermalZone> thermalZones;
    private final int currentThrottlingLevel;
    private final int maxThrottlingLevel;

    // Constructor, getters...
}

public class ThermalZone {
    private final String name;
    private final int temperature; // millicelsius
    private final int type;
    private final int throttlingThreshold;
    private final int shutdownThreshold;

    // Constructor, getters...
}