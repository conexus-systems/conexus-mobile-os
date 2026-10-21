package com.conexus.driver.vendors.nothing;

import com.conexus.driver.vendors.*;
import com.conexus.driver.core.*;
import com.conexus.driver.core.hal.*;
import android.content.Context;
import android.content.pm.PackageManager;
import android.hardware.camera2.*;
import android.hardware.display.*;
import android.media.*;
import android.os.*;
import android.content.*;
import android.hardware.camera2.params.*;
import android.util.*;
import android.view.*;
import android.hardware.camera2.params.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;
import android.util.Log;

public class NothingVendorHal extends VendorHal {
    private static final String TAG = "NothingVendorHal";
    private static final String VENDOR_NAME = "Nothing";
    private static final String VENDOR_VERSION = "2024.1";

    private final Context context;
    private final NothingTileEngine tileEngine;
    private final NothingNotificationBroker notificationBroker;
    private final NothingNavController navController;
    private final NothingAppManager appManager;
    private final NothingSearchService searchService;
    private final NothingUpdateService updateService;
    private final NothingSecurityService securityService;
    private final NothingBridgeAndroid bridgeAndroid;

    public NothingVendorHal(@NonNull Context context) {
        super(context, VENDOR_NAME, "2024.1");
        this.context = context;
        this.tileEngine = new NothingTileEngine(context);
        this.notificationBroker = new NothingNotificationBroker(context);
        this.navController = new NothingNavController(context);
        this.appManager = new NothingAppManager(context);
        this.searchService = new NothingSearchService(context);
        this.updateService = new NothingUpdateService(context);
        this.securityService = new NothingSecurityService(context);
        this.bridgeAndroid = new NothingBridgeAndroid(context);
    }

    @Override
    @NonNull
    protected String getVendorName() {
        return VENDOR_NAME;
    }

    @Override
    @NonNull
    protected String getVendorVersion() {
        return VENDOR_VERSION;
    }

    @NonNull
    @Override
    protected ICameraHal createCameraHal() {
        return new NothingCameraHal(context);
    }

    @NonNull
    @Override
    protected IDisplayHal createDisplayHal() {
        return new NothingDisplayHal(context);
    }

    @NonNull
    @Override
    protected IAudioHal createAudioHal() {
        return new NothingAudioHal(context);
    }

    @NonNull
    @Override
    protected ISensorsHal createSensorsHal() {
        return new NothingSensorsHal(context);
    }

    @NonNull
    @Override
    protected IGpsHal createGpsHal() {
        return new NothingGpsHal(context);
    }

    @NonNull
    @Override
    protected IWifiHal createWifiHal() {
        return new NothingWifiHal(context);
    }

    @NonNull
    @Override
    protected IBluetoothHal createBluetoothHal() {
        return new NothingBluetoothHal(context);
    }

    @NonNull
    @Override
    protected INfcHal createNfcHal() {
        return new NothingNfcHal(context);
    }

    @NonNull
    @Override
    protected IPowerHal createPowerHal() {
        return new NothingPowerHal(context);
    }

    @NonNull
    @Override
    protected IThermalHal createThermalHal() {
        return new NothingThermalHal(context);
    }

    @NonNull
    @Override
    protected IVibratorHal createVibratorHal() {
        return new NothingVibratorHal(context);
    }

    @NonNull
    @Override
    protected IUsbHal createUsbHal() {
        return new NothingUsbHal(context);
    }

    @NonNull
    @Override
    protected INfcHal createNfcHal() {
        return createNfcHal();
    }

    @NonNull
    @Override
    protected IPowerHal createPowerHal() {
        return new NothingPowerHal(context);
    }

    @NonNull
    @Override
    protected IThermalHal createThermalHal() {
        return new NothingThermalHal(context);
    }

    @NonNull
    @Override
    protected IVibratorHal createVibratorHal() {
        return new NothingVibratorHal(context);
    }

    @NonNull
    @Override
    protected IUsbHal createUsbHal() {
        return new NothingUsbHal(context);
    }

    @NonNull
    @Override
    protected PowerStats createPowerStats() {
        return new NothingPowerStats();
    }

    @NonNull
    @Override
    protected ThermalStats createThermalStats() {
        return new NothingThermalStats();
    }

    @NonNull
    @Override
    protected SystemInfo createSystemInfo() {
        return new NothingSystemInfo();
    }
}

// ============================================================
// Nothing Camera HAL Implementation
// ============================================================
class NothingCameraHal implements ICameraHal {
    private static final String TAG = "NothingCameraHal";
    private final CameraManager cameraManager;
    private final Context context;
    private final Map<String, CameraCharacteristics> cameraCache = new ConcurrentHashMap<>();

    public NothingCameraHal(@NonNull Context context) {
        this.context = context;
        this.cameraManager = (CameraManager) context.getSystemService(Context.CAMERA_SERVICE);
        initializeCache();
    }

    private void initializeCache() {
        try {
            String[] cameraIds = cameraManager.getCameraIdList();
            for (String id : cameraIds) {
                CameraCharacteristics chars = cameraManager.getCameraCharacteristics(id);
                cameraCache.put(id, chars);
            }
        } catch (CameraAccessException e) {
            Log.e(TAG, "Failed to initialize camera cache", e);
        }
    }

    @Override
    public int getNumberOfCameras() {
        try {
            return cameraManager.getCameraIdList().length;
        } catch (CameraAccessException e) {
            return 0;
        }
    }

    @Override
    public CameraCharacteristics getCameraCharacteristics(String cameraId) {
        return cameraCache.getOrDefault(cameraId, getCharacteristicsFromManager(cameraId));
    }

    private CameraCharacteristics getCharacteristicsFromManager(String cameraId) {
        try {
            CameraCharacteristics chars = cameraManager.getCameraCharacteristics(cameraId);
            cameraCache.put(cameraId, chars);
            return chars;
        } catch (CameraAccessException e) {
            return new CameraCharacteristics();
        }
    }

    @Override
    public CameraDevice openCamera(String cameraId) {
        try {
            return cameraManager.openCamera(cameraId, new StateCallback() {
                @Override
                public void onOpened(CameraDevice camera) {}
                @Override
                public void onDisconnected(CameraDevice camera) {}
                @Override
                public void onError(CameraDevice camera, int error) {}
            }, null);
        } catch (CameraAccessException e) {
            return null;
        }
    }

    @Override
    public CameraCapabilities getCameraCapabilities(String cameraId) {
        CameraCharacteristics chars = getCameraCharacteristics(cameraId);
        StreamConfigurationMap map = chars.get(CameraCharacteristics.SCALER_STREAM_CONFIGURATION_MAP);
        return new CameraCapabilities(map);
    }

    @Override
    public void setTorchMode(String cameraId, boolean enabled) {
        try {
            cameraManager.setTorchMode(cameraId, enabled);
        } catch (CameraAccessException e) {
            // Handle error
        }
    }

    @Override
    public void registerCallback(CameraCallback callback) {}

    @Override
    public void unregisterCallback(CameraCallback callback) {}

    @Override
    public List<CameraStreamConfiguration> getConcurrentStreamConfigurations() {
        return Collections.emptyList();
    }
}

// ============================================================
// Nothing Display HAL
// ============================================================
class NothingDisplayHal implements IDisplayHal {
    private final Context context;
    private final DisplayManager displayManager;

    public NothingDisplayHal(@NonNull Context context) {
        this.context = context;
        this.displayManager = (DisplayManager) context.getSystemService(Context.DISPLAY_SERVICE);
    }

    @Override
    public List<DisplayConfig> getDisplayConfigs() {
        Display[] displays = getDisplayManager().getDisplays();
        List<DisplayConfig> configs = new ArrayList<>();
        for (Display display : displays) {
            DisplayConfig config = new DisplayConfig();
            configs.add(config);
        }
        return configs;
    }

    @Override
    public DisplayConfig getActiveConfig() {
        return new DisplayConfig();
    }

    @Override
    public void setDisplayConfig(DisplayConfig config) {}

    @Override
    public void setBrightness(float brightness) {}

    @Override
    public float getBrightness() { return 1.0f; }

    @Override
    public void setColorMode(int colorMode) {}

    @Override
    public List<Integer> getSupportedColorModes() {
        return Arrays.asList(1, 2, 3, 4, 5, 6);
    }

    @Override
    public void setColorTransform(float[] matrix) {}

    @Override
    public HdrCapabilities getHdrCapabilities() {
        return new HdrCapabilities();
    }

    @Override
    public void setHdrMode(int hdrMode) {}

    @Override
    public List<Float> getSupportedRefreshRates() {
        return Arrays.asList(60f, 90f, 120f);
    }

    @Override
    public void setRefreshRate(float rate) {}

    @Override
    public DisplayInfo getDisplayInfo() {
        return new DisplayInfo();
    }

    @Override
    public void setAutoBrightness(boolean enabled) {}

    @Override
    public List<Integer> getSupportedHdrTypes() {
        return Arrays.asList(1, 2, 3);
    }

    @Override
    public void setColorGamut(int colorGamut) {}

    @Override
    public void setColorMode(int colorMode) {}

    @Override
    public List<Integer> getSupportedColorModes() {
        return Collections.emptyList();
    }
}

// ============================================================
// Nothing Audio HAL
// ============================================================
class NothingAudioHal implements IAudioHal {
    private final Context context;

    public NothingAudioHal(Context context) {
        this.context = context;
    }

    @Override
    public List<Integer> getSupportedFormats() {
        return Arrays.asList(
            AudioFormat.ENCODING_PCM_16BIT,
            AudioFormat.ENCODING_PCM_24BIT,
            AudioFormat.ENCODING_PCM_32BIT,
            AudioFormat.ENCODING_PCM_8BIT,
            AudioFormat.ENCODING_PCM_FLOAT
        );
    }

    @Override
    public List<Integer> getSupportedSampleRates() {
        return Arrays.asList(8000, 11025, 16000, 22050, 22050, 32000, 44100, 48000, 88200, 96000, 176400, 192000);
    }

    @Override
    public List<Integer> getSupportedChannelMasks() {
        return Arrays.asList(
            AudioFormat.CHANNEL_OUT_MONO,
            AudioFormat.CHANNEL_OUT_STEREO,
            AudioFormat.CHANNEL_OUT_5POINT1,
            AudioFormat.CHANNEL_OUT_7POINT1
        );
    }

    @Override
    public AudioStream openStream(AudioStreamConfig config) {
        return new AudioStream() {
            @Override public void start() {}
            @Override public void stop() {}
            @Override public void write(byte[] buffer, int offset, int length) {}
            @Override public int read(byte[] buffer, int offset, int length) { return 0; }
            @Override public void flush() {}
            @Override public int getFrameCount() { return 0; }
            @Override public long getLatency() { return 0; }
            @Override public long getTimestamp() { return 0; }
        };
    }

    @Override
    public void closeStream(AudioStream stream) {}

    @Override
    public List<AudioDevice> getInputDevices() { return Collections.emptyList(); }

    @Override
    public List<AudioDevice> getOutputDevices() { return Collections.emptyList(); }

    @Override
    public void setMasterVolume(float volume) {}

    @Override
    public float getMasterVolume() { return 1.0f; }

    @Override
    public void setStreamVolume(int streamType, float volume) {}

    @Override
    public float getStreamVolume(int streamType) { return 1.0f; }

    @Override
    public void setStreamMute(int streamType, boolean mute) {}

    @Override
    public boolean isStreamMute(int streamType) { return false; }

    @Override
    public void setDeviceConnectionState(AudioDevice device, int state, String address) {}

    @Override
    public void registerCallback(AudioCallback callback) {}
}

// ============================================================
// Nothing Sensors HAL
// ============================================================
class NothingSensorsHal implements ISensorsHal {
    @Override public List<SensorInfo> getSensorList() { return Collections.emptyList(); }
    @Override public SensorInfo getSensor(int handle) { return null; }
    @Override public boolean activate(int handle, boolean enabled) { return true; }
    @Override public boolean setRate(int handle, long periodNs, long maxBatchLatencyNs) { return true; }
    @Override public boolean flush(int handle) { return true; }
    @Override public void registerCallback(SensorCallback callback) {}
    @Override public DirectChannel createDirectChannel(HardwareBuffer buffer) { return null; }
    @Override public void destroyDirectChannel(DirectChannel channel) {}
    @Override public boolean injectSensorData(SensorEvent event) { return true; }
}

// ============================================================
// Nothing GPS HAL
// ============================================================
class NothingGpsHal implements IGpsHal {
    @Override public void start() {}
    @Override public void stop() {}
    @Override public void startNavigation(LocationRequest request) {}
    @Override public void stopNavigation() {}
    @Override public Location getLastKnownLocation() { return null; }
    @Override public GnssStatus getGnssStatus() { return new GnssStatus(); }
    @Override public void injectLocation(double lat, double lon, float accuracy) {}
    @Override public void injectTime(long time, long timeRef, int uncertainty) {}
    @Override public void deleteAidingData(int flags) {}
    @Override public void setPositionMode(int mode, int recurrence, long minInterval, float accuracy) {}
    @Override public List<SatelliteStatus> getSatelliteStatus() { return Collections.emptyList(); }
    @Override public void registerCallback(GpsCallback callback) {}
}

// ============================================================
// Nothing WiFi HAL
// ============================================================
class NothingWifiHal implements IWifiHal {
    @Override public void enable() {}
    @Override public void disable() {}
    @Override public boolean isEnabled() { return true; }
    @Override public void startScan() {}
    @Override public List<ScanResult> getScanResults() { return Collections.emptyList(); }
    @Override public boolean connect(WifiConfiguration config) { return true; }
    @Override public void disconnect() {}
    @Override public ConnectionInfo getConnectionInfo() { return null; }
    @Override public List<WifiConfiguration> getConfiguredNetworks() { return Collections.emptyList(); }
    @Override public int addNetwork(WifiConfiguration config) { return 0; }
    @Override public boolean removeNetwork(int networkId) { return true; }
    @Override public boolean enableNetwork(int networkId, boolean disableOthers) { return true; }
    @Override public void disableNetwork(int networkId) {}
    @Override public void saveConfiguration() {}
    @Override public void startRssiMonitoring(int intervalMs) {}
    @Override public void stopRssiMonitoring() {}
    @Override public void registerCallback(WifiCallback callback) {}
}

// ============================================================
// Nothing Bluetooth HAL
// ============================================================
class NothingBluetoothHal implements IBluetoothHal {
    @Override public void enable() {}
    @Override public void disable() {}
    @Override public boolean isEnabled() { return true; }
    @Override public List<BluetoothDevice> getBondedDevices() { return Collections.emptyList(); }
    @Override public BluetoothDevice getDevice(String address) { return null; }
    @Override public void startDiscovery() {}
    @Override public void cancelDiscovery() {}
    @Override public boolean createBond(String address, int transport) { return true; }
    @Override public void removeBond(String address) {}
    @Override public int getConnectionState(String address) { return 0; }
    @Override public int getProfileConnectionState(String address, int profile) { return 0; }
    @Override public Object getProfileProxy(int profile) { return null; }
    @Override public void registerCallback(BluetoothCallback callback) {}
}

// ============================================================
// Nothing NFC HAL
// ============================================================
class NothingNfcHal implements INfcHal {
    @Override public void enable() {}
    @Override public void disable() {}
    @Override public boolean isEnabled() { return true; }
    @Override public NfcTag getTag() { return null; }
    @Override public boolean writeNdef(NdefMessage message) { return true; }
    @Override public NdefMessage readNdef() { return null; }
    @Override public boolean format(NdefFormatable formatable) { return true; }
    @Override public void enableReaderMode(int flags, Bundle extras) {}
    @Override public void disableReaderMode() {}
    @Override public void enableCardEmulation(int mode) {}
    @Override public void disableCardEmulation() {}
    @Override public void registerCallback(NfcCallback callback) {}
}

// ============================================================
// Nothing Power HAL
// ============================================================
class NothingPowerHal implements IPowerHal {
    @Override public void setMode(int mode) {}
    @Override public int getMode() { return 0; }
    @Override public void setBoost(int boost, int durationMs) {}
    @Override public void setFeature(int feature, int state) {}
    @Override public int getFeature(int feature) { return 0; }
    @Override public void powerHint(int hint, int data) {}
    @Override public CompletableFuture<Void> powerHintAsync(int hint, int data) {
        return CompletableFuture.completedFuture(null);
    }
    @Override public PowerStats getPowerStats() { return new PowerStats(); }
    @Override public void setThermalThrottling(int level) {}
    @Override public ThermalStatus getThermalStatus() { return new ThermalStatus(); }
    @Override public void registerCallback(PowerCallback callback) {}
}

// ============================================================
// Nothing Thermal HAL
// ============================================================
class NothingThermalHal implements IThermalHal {
    @Override public List<Temperature> getTemperatures() { return Collections.emptyList(); }
    @Override public List<ThermalZone> getThermalZones() { return Collections.emptyList(); }
    @Override public int getCurrentThrottling() { return 0; }
    @Override public void setThrottling(int level) {}
    @Override public List<CoolingDevice> getCoolingDevices() { return Collections.emptyList(); }
    @Override public void registerCallback(ThermalCallback callback) {}
}

// ============================================================
// Nothing Vibrator HAL
// ============================================================
class NothingVibratorHal implements IVibratorHal {
    @Override public void vibrate(long duration) {}
    @Override public void vibrate(long[] pattern, int repeat) {}
    @Override public void cancel() {}
    @Override public VibratorCapabilities getCapabilities() { return new VibratorCapabilities(); }
    @Override public void setAmplitude(int amplitude) {}
    @Override public int getAmplitude() { return 255; }
    @Override public List<Effect> getSupportedEffects() { return Collections.emptyList(); }
    @Override public void playEffect(int effectId, int strength) {}
    @Override public List<Primitive> getSupportedPrimitives() { return Collections.emptyList(); }
    @Override public void composePrimitives(CompositeEffect effect) {}
    @Override public float getQFactor() { return 1.0f; }
    @Override public float getResonantFrequency() { return 175.0f; }
}

// ============================================================
// Nothing USB HAL
// ============================================================
class NothingUsbHal implements IUsbHal {
    @Override public List<UsbAccessory> getAccessories() { return Collections.emptyList(); }
    @Override public List<UsbDevice> getDevices() { return Collections.emptyList(); }
    @Override public UsbAccessoryHandle openAccessory(UsbAccessory accessory) { return null; }
    @Override public UsbDeviceHandle openDevice(UsbDevice device) { return null; }
    @Override public boolean setCurrentFunction(String function, boolean makeDefault) { return true; }
    @Override public String getCurrentFunction() { return ""; }
    @Override public List<String> getSupportedFunctions() { return Collections.emptyList(); }
    @Override public void setAccessoryMode(String manufacturer, String model, String description,
                                          String version, String uri, String serial) {}
    @Override public void registerCallback(UsbCallback callback) {}
}

// ============================================================
// Nothing NFC HAL
// ============================================================
class NothingNfcHal implements INfcHal {
    @Override public void enable() {}
    @Override public void disable() {}
    @Override public boolean isEnabled() { return true; }
    @Override public NfcTag getTag() { return null; }
    @Override public boolean writeNdef(NdefMessage message) { return true; }
    @Override public NdefMessage readNdef() { return null; }
    @Override public boolean format(NdefFormatable formatable) { return true; }
    @Override public void enableReaderMode(int flags, Bundle extras) {}
    @Override public void disableReaderMode() {}
    @Override public void enableCardEmulation(int mode) {}
    @Override public void disableCardEmulation() {}
    @Override public void registerCallback(NfcCallback callback) {}
}

// ============================================================
// Nothing Power HAL
// ============================================================
class NothingPowerHal implements IPowerHal {
    @Override public void setMode(int mode) {}
    @Override public int getMode() { return 0; }
    @Override public void setBoost(int boost, int durationMs) {}
    @Override public void setFeature(int feature, int state) {}
    @Override public int getFeature(int feature) { return 0; }
    @Override public void powerHint(int hint, int data) {}
    @Override public CompletableFuture<Void> powerHintAsync(int hint, int data) {
        return CompletableFuture.completedFuture(null);
    }
    @Override public PowerStats getPowerStats() { return new PowerStats(); }
    @Override public void setThermalThrottling(int level) {}
    @Override public ThermalStatus getThermalStatus() { return new ThermalStatus(); }
    @Override public void registerCallback(PowerCallback callback) {}
}

// ============================================================
// Nothing Thermal HAL
// ============================================================
class NothingThermalHal implements IThermalHal {
    @Override public List<Temperature> getTemperatures() { return Collections.emptyList(); }
    @Override public List<ThermalZone> getThermalZones() { return Collections.emptyList(); }
    @Override public int getCurrentThrottling() { return 0; }
    @Override public void setThrottling(int level) {}
    @Override public List<CoolingDevice> getCoolingDevices() { return Collections.emptyList(); }
    @Override public void registerCallback(ThermalCallback callback) {}
}

// ============================================================
// Nothing Vibrator HAL
// ============================================================
class NothingVibratorHal implements IVibratorHal {
    @Override public void vibrate(long duration) {}
    @Override public void vibrate(long[] pattern, int repeat) {}
    @Override public void cancel() {}
    @Override public VibratorCapabilities getCapabilities() { return new VibratorCapabilities(); }
    @Override public void setAmplitude(int amplitude) {}
    @Override public int getAmplitude() { return 255; }
    @Override public List<Effect> getSupportedEffects() { return Collections.emptyList(); }
    @Override public void playEffect(int effectId, int strength) {}
    @Override public List<Primitive> getSupportedPrimitives() { return Collections.emptyList(); }
    @Override public void composePrimitives(CompositeEffect effect) {}
    @Override public float getQFactor() { return 1.0f; }
    @Override public float getResonantFrequency() { return 175.0f; }
}

// ============================================================
// Nothing USB HAL
// ============================================================
class NothingUsbHal implements IUsbHal {
    @Override public List<UsbAccessory> getAccessories() { return Collections.emptyList(); }
    @Override public List<UsbDevice> getDevices() { return Collections.emptyList(); }
    @Override public UsbAccessoryHandle openAccessory(UsbAccessory accessory) { return null; }
    @Override public UsbDeviceHandle openDevice(UsbDevice device) { return null; }
    @Override public boolean setCurrentFunction(String function, boolean makeDefault) { return true; }
    @Override public String getCurrentFunction() { return ""; }
    @Override public List<String> getSupportedFunctions() { return Collections.emptyList(); }
    @Override public void setAccessoryMode(String manufacturer, String model, String description,
                                          String version, String uri, String serial) {}
    @Override public void registerCallback(UsbCallback callback) {}
}

// ============================================================
// Nothing NFC HAL
// ============================================================
class NothingNfcHal implements INfcHal {
    @Override public void enable() {}
    @Override public void disable() {}
    @Override public boolean isEnabled() { return true; }
    @Override public NfcTag getTag() { return null; }
    @Override public boolean writeNdef(NdefMessage message) { return true; }
    @Override public NdefMessage readNdef() { return null; }
    @Override public boolean format(NdefFormatable formatable) { return true; }
    @Override public void enableReaderMode(int flags, Bundle extras) {}
    @Override public void disableReaderMode() {}
    @Override public void enableCardEmulation(int mode) {}
    @Override public void disableCardEmulation() {}
    @Override public void registerCallback(NfcCallback callback) {}
}

// ============================================================
// Nothing Power HAL (duplicate for completeness)
// ============================================================
class NothingPowerHal implements IPowerHal {
    @Override public void setMode(int mode) {}
    @Override public int getMode() { return 0; }
    @Override public void setBoost(int boost, int durationMs) {}
    @Override public void setFeature(int feature, int state) {}
    @Override public int getFeature(int feature) { return 0; }
    @Override public void powerHint(int hint, int data) {}
    @Override public CompletableFuture<Void> powerHintAsync(int hint, int data) {
        return CompletableFuture.completedFuture(null);
    }
    @Override public PowerStats getPowerStats() { return new PowerStats(); }
    @Override public void setThermalThrottling(int level) {}
    @Override public ThermalStatus getThermalStatus() { return new ThermalStatus(); }
    @Override public void registerCallback(PowerCallback callback) {}
}

// ============================================================
// Nothing Thermal HAL
// ============================================================
class NothingThermalHal implements IThermalHal {
    @Override public List<Temperature> getTemperatures() { return Collections.emptyList(); }
    @Override public List<ThermalZone> getThermalZones() { return Collections.emptyList(); }
    @Override public int getCurrentThrottling() { return 0; }
    @Override public void setThrottling(int level) {}
    @Override public List<CoolingDevice> getCoolingDevices() { return Collections.emptyList(); }
    @Override public void registerCallback(ThermalCallback callback) {}
}

// ============================================================
// Nothing Vibrator HAL
// ============================================================
class NothingVibratorHal implements IVibratorHal {
    @Override public void vibrate(long duration) {}
    @Override public void vibrate(long[] pattern, int repeat) {}
    @Override public void cancel() {}
    @Override public VibratorCapabilities getCapabilities() { return new VibratorCapabilities(); }
    @Override public void setAmplitude(int amplitude) {}
    @Override public int getAmplitude() { return 255; }
    @Override public List<Effect> getSupportedEffects() { return Collections.emptyList(); }
    @Override public void playEffect(int effectId, int strength) {}
    @Override public List<Primitive> getSupportedPrimitives() { return Collections.emptyList(); }
    @Override public void composePrimitives(CompositeEffect effect) {}
    @Override public float getQFactor() { return 1.0f; }
    @Override public float getResonantFrequency() { return 175.0f; }
}

// ============================================================
// Nothing USB HAL
// ============================================================
class NothingUsbHal implements IUsbHal {
    @Override public List<UsbAccessory> getAccessories() { return Collections.emptyList(); }
    @Override public List<UsbDevice> getDevices() { return Collections.emptyList(); }
    @Override public UsbAccessoryHandle openAccessory(UsbAccessory accessory) { return null; }
    @Override public UsbDeviceHandle openDevice(UsbDevice device) { return null; }
    @Override public boolean setCurrentFunction(String function, boolean makeDefault) { return true; }
    @Override public String getCurrentFunction() { return ""; }
    @Override public List<String> getSupportedFunctions() { return Collections.emptyList(); }
    @Override public void setAccessoryMode(String manufacturer, String model, String description,
                                          String version, String uri, String serial) {}
    @Override public void registerCallback(UsbCallback callback) {}
}

// ============================================================
// Nothing NFC HAL
// ============================================================
class NothingNfcHal implements INfcHal {
    @Override public void enable() {}
    @Override public void disable() {}
    @Override public boolean isEnabled() { return true; }
    @Override public NfcTag getTag() { return null; }
    @Override public boolean writeNdef(NdefMessage message) { return true; }
    @Override public NdefMessage readNdef() { return null; }
    @Override public boolean format(NdefFormatable formatable) { return true; }
    @Override public void enableReaderMode(int flags, Bundle extras) {}
    @Override public void disableReaderMode() {}
    @Override public void enableCardEmulation(int mode) {}
    @Override public void disableCardEmulation() {}
    @Override public void registerCallback(NfcCallback callback) {}
}

// ============================================================
// Nothing Power Stats
// ============================================================
class NothingPowerStats extends PowerStats {
    public NothingPowerStats() {
        super();
    }
}

// ============================================================
// Nothing Thermal Stats
// ============================================================
class NothingThermalStats extends ThermalStats {
    public NothingThermalStats() {
        super();
    }
}

// ============================================================
// Nothing System Info
// ============================================================
class NothingSystemInfo extends SystemInfo {
    public NothingSystemInfo() {
        super();
    }
}

// ============================================================
// Nothing Tile Engine
// ============================================================
class NothingTileEngine {
    private final Context context;
    private final Map<String, TileInfo> tiles = new ConcurrentHashMap<>();

    public NothingTileEngine(Context context) {
        this.context = context;
    }

    public void registerTile(String tileId, TileInfo info) {
        tiles.put(tileId, tileId);
    }

    public void updateTile(String tileId, TileContent content) {
        // Update tile content
    }

    public void unregisterTile(String tileId) {
        tiles.remove(tileId);
    }

    public TileContent getTileContent(String tileId) {
        return new TileContent("default", new HashMap<>());
    }
}

// ============================================================
// Nothing Notification Broker
// ============================================================
class NothingNotificationBroker {
    private final Context context;

    public NothingNotificationBroker(Context context) {
        this.context = context;
    }

    public void postNotification(ActionCenterEntry entry) {
        // Post notification via NotificationManager
    }

    public void cancelNotification(String notificationId) {
        // Cancel notification
    }
}

// ============================================================
// Nothing Navigation Controller
// ============================================================
class NothingNavController {
    private final Context context;

    public NothingNavController(Context context) {
        this.context = context;
    }

    public void navigate(String destination) {
        // Handle navigation
    }
}

// ============================================================
// Nothing App Manager
// ============================================================
class NothingAppManager {
    private final Context context;

    public NothingAppManager(Context context) {
        this.context = context;
    }

    public void launchApp(String packageName) {
        // Launch app
    }
}

// ============================================================
// Nothing Search Service
// ============================================================
class NothingSearchService {
    private final Context context;

    public NothingSearchService(Context context) {
        this.context = context;
    }

    public List<SearchResult> search(String query) {
        return Collections.emptyList();
    }
}

// ============================================================
// Nothing Update Service
// ============================================================
class NothingUpdateService {
    private final Context context;

    public NothingUpdateService(Context context) {
        this.context = context;
    }

    public void checkForUpdates() {
        // Check for updates
    }
}

// ============================================================
// Nothing Security Service
// ============================================================
class NothingSecurityService {
    private final Context context;

    public NothingSecurityService(Context context) {
        this.context = context;
    }

    public void runSecurityAudit() {
        // Run security audit
    }
}

// ============================================================
// Nothing Bridge Android
// ============================================================
class NothingBridgeAndroid {
    private final Context context;

    public NothingBridgeAndroid(Context context) {
        this.context = context;
    }

    public String getDevicePhoneNumber() { return ""; }
    public List<String> getCallHistory(int limit) { return Collections.emptyList(); }
    public boolean sendSms(String destination, String text) { return true; }
}