// Conexus Driver Core - HAL Interfaces
// Hardware Abstraction Layer interfaces for Android components

package com.conexus.driver.core.hal;

import android.annotation.NonNull;
import android.annotation.Nullable;
import android.hardware.camera2.CameraCharacteristics;
import android.hardware.camera2.CameraManager;
import android.graphics.ImageFormat;
import android.hardware.HardwareBuffer;
import android.media.Image;
import android.os.RemoteException;
import java.util.List;
import java.util.Map;
import java.util.concurrent.CompletableFuture;

/**
 * Camera HAL Interface
 */
public interface ICameraHal {
    /** Get number of cameras */
    int getNumberOfCameras();

    /** Get camera characteristics */
    @NonNull CameraCharacteristics getCameraCharacteristics(@NonNull String cameraId);

    /** Open camera device */
    @NonNull CameraDevice openCamera(@NonNull String cameraId);

    /** Get camera capabilities */
    @NonNull CameraCapabilities getCameraCapabilities(@NonNull String cameraId);

    /** Set torch mode */
    void setTorchMode(@NonNull String cameraId, boolean enabled);

    /** Register callback */
    void registerCallback(@NonNull CameraCallback callback);

    /** Unregister callback */
    void unregisterCallback(@NonNull CameraCallback callback);

    /** Get concurrent stream combinations */
    @NonNull List<CameraStreamConfiguration> getConcurrentStreamConfigurations();

    interface CameraCallback {
        void onCameraOpened(@NonNull String cameraId);
        void onCameraClosed(@NonNull String cameraId);
        void onCameraError(@NonNull String cameraId, int error);
        void onCaptureCompleted(@NonNull CameraCaptureResult result);
        void onCaptureFailed(@NonNull String cameraId, int error);
        void onCaptureStarted(@NonNull String cameraId, long timestamp);
    }
}

class CameraCapabilities {
    private final List<Integer> supportedFormats;
    private final List<android.util.Size> supportedSizes;
    private final int maxJpegSize;
    private final boolean supportsRaw;
    private final boolean supportsDepth;
    private final boolean supportsBokeh;
    private final boolean supportsNightMode;
    private final boolean supportsPortrait;
    private final int maxZoomRatio;
    private final List<Float> zoomRatios;
    private final boolean supportsOIS;
    private final boolean supportsEIS;

    // Constructor, getters...
}

class CameraStreamConfiguration {
    private final int format;
    private final int width;
    private final int height;
    private final int streamType;

    // Constructor, getters...
}

class CameraCaptureResult {
    private final String cameraId;
    private final long timestamp;
    private final int frameNumber;
    private final boolean success;
    private final Map<String, Object> metadata;

    // Constructor, getters...
}

/**
 * Display HAL Interface
 */
public interface IDisplayHal {
    /** Get display configurations */
    @NonNull List<DisplayConfig> getDisplayConfigs();

    /** Get active display config */
    @NonNull DisplayConfig getActiveConfig();

    /** Set display config */
    void setDisplayConfig(@NonNull DisplayConfig config);

    /** Set brightness */
    void setBrightness(float brightness);

    /** Get brightness */
    float getBrightness();

    /** Set color mode */
    void setColorMode(int colorMode);

    /** Get supported color modes */
    @NonNull List<Integer> getSupportedColorModes();

    /** Set color transform */
    void setColorTransform(float[] matrix);

    /** Get HDR capabilities */
    @NonNull HdrCapabilities getHdrCapabilities();

    /** Set HDR mode */
    void setHdrMode(int hdrMode);

    /** Get supported refresh rates */
    @NonNull List<Float> getSupportedRefreshRates();

    /** Set refresh rate */
    void setRefreshRate(float rate);

    /** Get display info */
    @NonNull DisplayInfo getDisplayInfo();

    /** Set auto brightness */
    void setAutoBrightness(boolean enabled);

    /** Get supported HDR types */
    @NonNull List<Integer> getSupportedHdrTypes();

    /** Set color gamut */
    void setColorGamut(int colorGamut);

    interface DisplayCallback {
        void onDisplayChanged(@NonNull DisplayConfig config);
        void onHotplug(@NonNull String displayId, boolean connected);
        void onVsync(long timestamp);
    }
}

class DisplayConfig {
    private final String displayId;
    private final int width;
    private final int height;
    private final float refreshRate;
    private final int colorMode;
    private final int hdrType;
    private final float density;
    private final int orientation;

    // Constructor, getters...
}

class DisplayInfo {
    private final String displayId;
    private final String name;
    private final int width;
    private final int height;
    private final float refreshRate;
    private final float density;
    private final int orientation;
    private final boolean isInternal;
    private final boolean isSecure;
    private final long appVsyncOffset;
    private final long presentationDeadline;

    // Constructor, getters...
}

class HdrCapabilities {
    private final List<Integer> supportedHdrTypes;
    private final float maxLuminance;
    private final float maxAverageLuminance;
    private final float minLuminance;

    // Constructor, getters...
}

/**
 * Audio HAL Interface
 */
public interface IAudioHal {
    /** Get supported audio formats */
    @NonNull List<Integer> getSupportedFormats();

    /** Get supported sample rates */
    @NonNull List<Integer> getSupportedSampleRates();

    /** Get supported channel masks */
    @NonNull List<Integer> getSupportedChannelMasks();

    /** Open audio stream */
    @NonNull AudioStream openStream(@NonNull AudioStreamConfig config);

    /** Close audio stream */
    void closeStream(@NonNull AudioStream stream);

    /** Get input devices */
    @NonNull List<AudioDevice> getInputDevices();

    /** Get output devices */
    @NonNull List<AudioDevice> getOutputDevices();

    /** Set master volume */
    void setMasterVolume(float volume);

    /** Get master volume */
    float getMasterVolume();

    /** Set stream volume */
    void setStreamVolume(int streamType, float volume);

    /** Get stream volume */
    float getStreamVolume(int streamType);

    /** Set mute */
    void setStreamMute(int streamType, boolean mute);

    /** Get mute */
    boolean isStreamMute(int streamType);

    /** Set device connection state */
    void setDeviceConnectionState(@NonNull AudioDevice device, int state, String address);

    /** Register callback */
    void registerCallback(@NonNull AudioCallback callback);

    interface AudioCallback {
        void onVolumeChanged(int streamType, float volume);
        void onDeviceAdded(@NonNull AudioDevice device);
        void onDeviceRemoved(@NonNull AudioDevice device);
        void onAudioFocusChange(int focusChange);
        void onAudioPortUpdate(@NonNull AudioPort port);
    }
}

class AudioStreamConfig {
    private final int streamType;
    private final int sampleRate;
    private final int channelMask;
    private final int format;
    private final int flags;
    private final int bufferSize;

    // Constructor, getters...
}

class AudioStream {
    private final int handle;
    private final AudioStreamConfig config;

    void start() throws RemoteException;
    void stop() throws RemoteException;
    void write(@NonNull byte[] buffer, int offset, int length) throws RemoteException;
    int read(@NonNull byte[] buffer, int offset, int length) throws RemoteException;
    void flush() throws RemoteException;
    int getFrameCount();
    long getLatency();
    long getTimestamp();

    // Constructor, getters...
}

class AudioDevice {
    private final int id;
    private final String name;
    private final int type;
    private final String address;
    private final boolean isInput;
    private final boolean isOutput;
    private final String address;

    // Constructor, getters...
}

class AudioPort {
    private final int id;
    private final String name;
    private final int role;
    private final int type;
    private final List<AudioGain> gains;

    // Constructor, getters...
}

class AudioGain {
    private final int id;
    private final int mode;
    private final int channelMask;
    private final int minValue;
    private final int maxValue;
    private final int stepValue;
    private final int defaultValue;
    private final String unit;

    // Constructor, getters...
}

/**
 * Sensors HAL Interface
 */
public interface ISensorsHal {
    /** Get sensor list */
    @NonNull List<SensorInfo> getSensorList();

    /** Get sensor info */
    @Nullable SensorInfo getSensor(int handle);

    /** Activate sensor */
    boolean activate(int handle, boolean enabled);

    /** Set sensor rate */
    boolean setRate(int handle, long periodNs, long maxBatchLatencyNs);

    /** Flush sensor */
    boolean flush(int handle);

    /** Register callback */
    void registerCallback(@NonNull SensorCallback callback);

    /** Get direct channel */
    @Nullable DirectChannel createDirectChannel(@NonNull HardwareBuffer buffer);

    /** Destroy direct channel */
    void destroyDirectChannel(@NonNull DirectChannel channel);

    /** Inject sensor data (for testing) */
    boolean injectSensorData(@NonNull SensorEvent event);

    interface SensorCallback {
        void onSensorChanged(@NonNull SensorEvent event);
        void onAccuracyChanged(int handle, int accuracy);
        void onSensorAdded(@NonNull SensorInfo sensor);
        void onSensorRemoved(int handle);
    }
}

class SensorInfo {
    private final int handle;
    private final String name;
    private final int type;
    private final String vendor;
    private final int version;
    private final float maxRange;
    private final float resolution;
    private final float power;
    private final int minDelay;
    private final int fifoReservedEventCount;
    private final int fifoMaxEventCount;
    private final String stringType;
    private final int requiredPermission;
    private final int maxDelay;
    private final int flags;
    private final int reserved;

    // Constructor, getters...
}

class SensorEvent {
    private final int handle;
    private final long timestamp;
    private final float[] values;
    private final int accuracy;
    private final int sensorType;

    // Constructor, getters...
}

class DirectChannel {
    private final int id;
    private final HardwareBuffer buffer;

    // Constructor, getters...
}

/**
 * GPS/GNSS HAL Interface
 */
public interface IGpsHal {
    /** Start GPS */
    void start();

    /** Stop GPS */
    void stop();

    /** Start navigation */
    void startNavigation(@NonNull LocationRequest request);

    /** Stop navigation */
    void stopNavigation();

    /** Get last known location */
    @Nullable Location getLastKnownLocation();

    /** Get GNSS status */
    @NonNull GnssStatus getGnssStatus();

    /** Inject location */
    void injectLocation(double latitude, double longitude, float accuracy);

    /** Inject time */
    void injectTime(long time, long timeReference, int uncertainty);

    /** Delete aiding data */
    void deleteAidingData(int flags);

    /** Set position mode */
    void setPositionMode(int mode, int recurrence, long minInterval, float accuracy);

    /** Get satellite status */
    @NonNull List<SatelliteStatus> getSatelliteStatus();

    /** Register callback */
    void registerCallback(@NonNull GpsCallback callback);

    interface GpsCallback {
        void onLocationChanged(@NonNull Location location);
        void onStatusChanged(int status);
        void onSvStatusChanged(@NonNull GnssStatus status);
        void onNmeaReceived(long timestamp, @NonNull String nmea);
        void onLocationFailed(int error);
    }
}

class Location {
    private final double latitude;
    private final double longitude;
    private final double altitude;
    private final float accuracy;
    private final float bearing;
    private final float speed;
    private final long time;
    private final String provider;

    // Constructor, getters...
}

class LocationRequest {
    private final int priority;
    private final long interval;
    private final long fastestInterval;
    private final long expireAt;
    private final int maxWaitTime;

    // Constructor, getters...
}

class GnssStatus {
    private final int satelliteCount;
    private final List<SatelliteStatus> satellites;

    // Constructor, getters...
}

class SatelliteStatus {
    private final int svid;
    private final int constellationType;
    private final float cn0;
    private final float elevation;
    private final float azimuth;
    private final boolean usedInFix;
    private final boolean hasEphemeris;
    private final boolean hasAlmanac;

    // Constructor, getters...
}

/**
 * WiFi HAL Interface
 */
public interface IWifiHal {
    /** Enable WiFi */
    void enable();

    /** Disable WiFi */
    void disable();

    /** Is WiFi enabled */
    boolean isEnabled();

    /** Start scan */
    void startScan();

    /** Get scan results */
    @NonNull List<ScanResult> getScanResults();

    /** Connect to network */
    boolean connect(@NonNull WifiConfiguration config);

    /** Disconnect */
    void disconnect();

    /** Get connection info */
    @Nullable ConnectionInfo getConnectionInfo();

    /** Get configured networks */
    @NonNull List<WifiConfiguration> getConfiguredNetworks();

    /** Add network */
    int addNetwork(@NonNull WifiConfiguration config);

    /** Remove network */
    boolean removeNetwork(int networkId);

    /** Enable network */
    boolean enableNetwork(int networkId, boolean disableOthers);

    /** Disable network */
    void disableNetwork(int networkId);

    /** Save configuration */
    void saveConfiguration();

    /** Start RSSI monitoring */
    void startRssiMonitoring(int intervalMs);

    /** Stop RSSI monitoring */
    void stopRssiMonitoring();

    /** Register callback */
    void registerCallback(@NonNull WifiCallback callback);

    interface WifiCallback {
        void onScanResults(@NonNull List<ScanResult> results);
        void onConnectionChanged(boolean connected, @Nullable ConnectionInfo info);
        void onRssiChanged(int rssi);
        void onNetworkAdded(int networkId);
        void onNetworkRemoved(int networkId);
    }
}

class ScanResult {
    private final String ssid;
    private final String bssid;
    private final String capabilities;
    private final int frequency;
    private final int level;
    private final long timestamp;
    private final String venueName;
    private final boolean is80211mcResponder;

    // Constructor, getters...
}

class WifiConfiguration {
    private final String ssid;
    private final String bssid;
    private final String preSharedKey;
    private final int networkId;
    private final int priority;
    private final int allowedKeyManagement;
    private final int allowedProtocols;
    private final int allowedAuthAlgorithms;
    private final int allowedPairwiseCiphers;
    private final int allowedGroupCiphers;

    // Constructor, getters...
}

class ConnectionInfo {
    private final String ssid;
    private final String bssid;
    private final String ipAddress;
    private final int networkId;
    private final int linkSpeed;
    private final int rssi;
    private final String macAddress;
    private final boolean isHidden;

    // Constructor, getters...
}

class ScanResult {
    private final String ssid;
    private final String bssid;
    private final String capabilities;
    private final int frequency;
    private final int level;
    private final long timestamp;
    private final String venueName;
    private final boolean is80211mcResponder;

    // Constructor, getters...
}

/**
 * Bluetooth HAL Interface
 */
public interface IBluetoothHal {
    /** Enable Bluetooth */
    void enable();

    /** Disable Bluetooth */
    void disable();

    /** Is Bluetooth enabled */
    boolean isEnabled();

    /** Get bonded devices */
    @NonNull List<BluetoothDevice> getBondedDevices();

    /** Get device */
    @Nullable BluetoothDevice getDevice(String address);

    /** Start discovery */
    void startDiscovery();

    /** Cancel discovery */
    void cancelDiscovery();

    /** Create bond */
    void createBond(String address, int transport);

    /** Remove bond */
    void removeBond(String address);

    /** Get connection state */
    int getConnectionState(String address);

    /** Get profile connection state */
    int getProfileConnectionState(String address, int profile);

    /** Connect profile */
    boolean connectProfile(String address, int profile);

    /** Disconnect profile */
    boolean disconnectProfile(String address, int profile);

    /** Get profile proxy */
    @Nullable Object getProfileProxy(int profile);

    /** Register callback */
    void registerCallback(BluetoothCallback callback);

    interface BluetoothCallback {
        void onStateChanged(int state);
        void onDeviceFound(BluetoothDevice device);
        void onDeviceLost(BluetoothDevice device);
        void onBondStateChanged(BluetoothDevice device, int state);
        void onConnectionStateChanged(String address, int state);
        void onProfileConnectionStateChanged(String address, int profile, int state);
    }
}

class BluetoothDevice {
    private final String name;
    private final String address;
    private final int type;
    private final int bondState;
    private final int connectionState;
    private final String name;

    // Constructor, getters...
}

/**
 * NFC HAL Interface
 */
public interface INfcHal {
    /** Enable NFC */
    void enable();

    /** Disable NFC */
    void disable();

    /** Is NFC enabled */
    boolean isEnabled();

    /** Get NFC tag */
    @Nullable NfcTag getTag();

    /** Write NDEF */
    boolean writeNdef(NdefMessage message);

    /** Read NDEF */
    @Nullable NdefMessage readNdef();

    /** Format tag */
    boolean format(NdefFormatable formatable);

    /** Enable reader mode */
    void enableReaderMode(int flags, Bundle extras);

    /** Disable reader mode */
    void disableReaderMode();

    /** Enable card emulation */
    void enableCardEmulation(int mode);

    /** Disable card emulation */
    void disableCardEmulation();

    /** Register callback */
    void registerCallback(NfcCallback callback);

    interface NfcCallback {
        void onTagDiscovered(NfcTag tag);
        void onTagLost(NfcTag tag);
        void onNdefRead(NdefMessage message);
        void onNdefWrite(boolean success);
    }
}

class NfcTag {
    private final byte[] id;
    private final String[] techList;
    private final Map<Byte, byte[]> techExtras;

    // Constructor, getters...
}

class NdefMessage {
    private final NdefRecord[] records;

    // Constructor, getters...
}

class NdefRecord {
    private final short tnf;
    private final byte[] type;
    private final byte[] id;
    private final byte[] payload;

    // Constructor, getters...
}

/**
 * Power HAL Interface
 */
public interface IPowerHal {
    /** Set power mode */
    void setMode(int mode);

    /** Get current mode */
    int getMode();

    /** Set boost */
    void setBoost(int boost, int durationMs);

    /** Set feature */
    void setFeature(int feature, int state);

    /** Get feature state */
    int getFeature(int feature);

    /** Get power hint */
    void powerHint(int hint, int data);

    /** Get power hint async */
    CompletableFuture<Void> powerHintAsync(int hint, int data);

    /** Get power stats */
    PowerStats getPowerStats();

    /** Set thermal throttling */
    void setThermalThrottling(int level);

    /** Get thermal status */
    ThermalStatus getThermalStatus();

    /** Register callback */
    void registerCallback(PowerCallback callback);

    interface PowerCallback {
        void onModeChanged(int mode);
        void onBoostChanged(int boost);
        void onThermalThrottling(int level);
        void onPowerHint(int hint, int data);
    }
}

enum PowerMode {
    NORMAL,
    PERFORMANCE,
    POWER_SAVE,
    LOW_POWER,
    SUSTAINED_PERFORMANCE,
    VR_PERFORMANCE,
    LOW_LATENCY
}

enum PowerFeature {
    CPU_BOOST,
    GPU_BOOST,
    DISPLAY_BOOST,
    CAMERA_BOOST,
    AUDIO_BOOST,
    VIDEO_BOOST,
    NETWORK_BOOST
}

/**
 * Thermal HAL Interface
 */
public interface IThermalHal {
    /** Get temperatures */
    @NonNull List<Temperature> getTemperatures();

    /** Get thermal zones */
    @NonNull List<ThermalZone> getThermalZones();

    /** Get current throttling */
    int getCurrentThrottling();

    /** Set thermal throttling */
    void setThrottling(int level);

    /** Get cooling devices */
    @NonNull List<CoolingDevice> getCoolingDevices();

    /** Register callback */
    void registerCallback(ThermalCallback callback);

    interface ThermalCallback {
        void onTemperatureChanged(Temperature temperature);
        void onThrottlingChanged(int level);
        void onCoolingDeviceChanged(CoolingDevice device);
    }
}

class Temperature {
    private final String name;
    private final int type;
    private final float value;
    private final long timestamp;

    // Constructor, getters...
}

class ThermalZone {
    private final int id;
    private final String name;
    private final float temperature;
    private final float threshold;
    private final float criticalTemperature;
    private final int type;

    // Constructor, getters...
}

class CoolingDevice {
    private final String name;
    private final int type;
    private final int maxState;
    private final int currentState;

    // Constructor, getters...
}

/**
 * Vibrator HAL Interface
 */
public interface IVibratorHal {
    /** Vibrate */
    void vibrate(long duration);

    /** Vibrate with pattern */
    void vibrate(long[] pattern, int repeat);

    /** Cancel vibration */
    void cancel();

    /** Get capabilities */
    VibratorCapabilities getCapabilities();

    /** Set amplitude */
    void setAmplitude(int amplitude);

    /** Get amplitude */
    int getAmplitude();

    /** Get supported effects */
    @NonNull List<Effect> getSupportedEffects();

    /** Play effect */
    void playEffect(int effectId, int strength);

    /** Get supported primitives */
    @NonNull List<Primitive> getSupportedPrimitives();

    /** Compose primitives */
    void composePrimitives(CompositeEffect effect);

    /** Get Q factor */
    float getQFactor();

    /** Get resonant frequency */
    float getResonantFrequency();

    interface VibratorCallback {
        void onComplete();
        void onError(int error);
    }
}

class VibratorCapabilities {
    private final boolean supportsAmplitudeControl;
    private final boolean supportsEffects;
    private final boolean supportsPrimitives;
    private final boolean supportsComposePrimitives;
    private final boolean supportsAlwaysOn;
    private final int maxAmplitude;
    private final int minAmplitude;

    // Constructor, getters...
}

enum Effect {
    CLICK,
    DOUBLE_CLICK,
    TICK,
    HEAVY_CLICK,
    LONG_PRESS,
    KEYBOARD_TAP,
    VIRTUAL_KEY,
    TEXT_HANDLE_MOVE
}

class Primitive {
    private final int id;
    private final String name;
    private final long duration;

    // Constructor, getters...
}

class CompositeEffect {
    private final Primitive[] primitives;
    private final float[] scale;
    private final long[] delay;

    // Constructor, getters...
}

/**
 * USB HAL Interface
 */
public interface IUsbHal {
    /** Get accessories */
    @NonNull List<UsbAccessory> getAccessories();

    /** Get devices */
    @NonNull List<UsbDevice> getDevices();

    /** Open accessory */
    @NonNull UsbAccessoryHandle openAccessory(UsbAccessory accessory);

    /** Open device */
    @NonNull UsbDeviceHandle openDevice(UsbDevice device);

    /** Set current function */
    boolean setCurrentFunction(String function, boolean makeDefault);

    /** Get current function */
    String getCurrentFunction();

    /** Get supported functions */
    List<String> getSupportedFunctions();

    /** Set accessory mode */
    void setAccessoryMode(String manufacturer, String model, String description,
                         String version, String uri, String serial);

    /** Register callback */
    void registerCallback(UsbCallback callback);

    interface UsbCallback {
        void onAccessoryAttached(UsbAccessory accessory);
        void onAccessoryDetached(UsbAccessory accessory);
        void onDeviceAttached(UsbDevice device);
        void onDeviceDetached(UsbDevice device);
        void onFunctionChanged(String function);
    }
}

class UsbAccessory {
    private final String manufacturer;
    private final String model;
    private final String description;
    private final String version;
    private final String uri;
    private final String serial;
    private final int versionCode;

    // Constructor, getters...
}

class UsbDevice {
    private final String name;
    private final int vendorId;
    private final int productId;
    private final int deviceClass;
    private final int deviceSubclass;
    private final int deviceProtocol;
    private final int vendorId;
    private final int productId;

    // Constructor, getters...
}

class UsbAccessoryHandle {
    private final int fileDescriptor;

    FileDescriptor getFileDescriptor();
    void close();
}

class UsbDeviceHandle {
    private final String deviceName;
    private final int fileDescriptor;

    FileDescriptor getFileDescriptor();
    void close();
    UsbInterface getInterface(int index);
    UsbEndpoint getEndpoint(int interfaceIndex, int endpointIndex);
    int controlTransfer(int requestType, int request, int value, int index,
                       byte[] buffer, int length, int timeout);
    int bulkTransfer(UsbEndpoint endpoint, byte[] buffer, int length, int timeout);
}

/**
 * NFC HAL Interface
 */
public interface INfcHal {
    void enable();
    void disable();
    boolean isEnabled();
    NfcTag getTag();
    boolean writeNdef(NdefMessage message);
    NdefMessage readNdef();
    boolean format(NdefFormatable formatable);
    void enableReaderMode(int flags, Bundle extras);
    void disableReaderMode();
    void enableCardEmulation(int mode);
    void disableCardEmulation();
    void registerCallback(NfcCallback callback);

    interface NfcCallback {
        void onTagDiscovered(NfcTag tag);
        void onTagLost(NfcTag tag);
        void onNdefRead(NdefMessage message);
        void onNdefWrite(boolean success);
    }
}

class NfcTag {
    private byte[] id;
    private String[] techList;
    Map<Byte, byte[]> techExtras;
}

class NdefMessage {
    NdefRecord[] records;
}

class NdefRecord {
    short tnf;
    byte[] type;
    byte[] id;
    byte[] payload;
}