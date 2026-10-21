package com.conexus.driver.kernel;

import android.annotation.NonNull;
import android.annotation.Nullable;
import android.os.Build;
import android.util.Log;

import java.io.BufferedReader;
import java.io.FileReader;
import java.io.IOException;

/**
 * Kernel Factory - Selects appropriate kernel implementation based on device
 */
public class KernelFactory {
    private static final String TAG = "KernelFactory";

    /**
     * Get kernel implementation for current device
     */
    @NonNull
    public static IKernelInterface getKernelForDevice() {
        String version = getKernelVersion();
        Log.d(TAG, "Detected kernel version: " + version);

        if (version.startsWith("6.6")) {
            return new Kernel66();
        } else if (version.startsWith("6.1")) {
            return new Kernel61();
        } else if (version.startsWith("5.15")) {
            return new Kernel515();
        } else if (version.startsWith("5.10")) {
            return new Kernel510();
        } else if (version.startsWith("5.4")) {
            return new Kernel54();
        } else if (version.startsWith("4.19")) {
            return new Kernel419();
        } else {
            Log.w(TAG, "Unknown kernel version: " + version + ", defaulting to 6.1");
            return new Kernel61();
        }
    }

    /**
     * Get kernel implementation for specific device
     */
    @NonNull
    public static IKernelInterface getKernelForDevice(@NonNull String deviceCodename) {
        switch (deviceCodename) {
            case "akita": // Pixel 8a
            case "cheetah": // Pixel 7
            case "panther": // Pixel 7 Pro
            case "lynx": // Pixel 7a
            case "tokay": // Pixel 9
            case "caiman": // Pixel 9 Pro
            case "komodo": // Pixel 9 Pro XL
            case "tegu": // Pixel 9a
                return new Kernel61(); // Tensor G3/G4 devices use 6.1 GKI

            case "pong": // Nothing Phone 2
            case "tetris": // Nothing CMF Phone 1
                return new Kernel61(); // Nothing devices use 6.1

            case "waffle": // OnePlus 12
            case "sm8650": // OnePlus 13
                return new Kernel61(); // OnePlus 12/13 use 6.1

            case "houji": // Xiaomi 14
            case "aurora": // Xiaomi 14 Pro
                return new Kernel61(); // Xiaomi 14 uses 6.1

            // Samsung devices
            case "dm3q": // Galaxy S23
            case "dm1q": // Galaxy S23+
            case "dm5q": // Galaxy S23 Ultra
            case "e2q": // Galaxy S24
            case "e3q": // Galaxy S24+
            case "e4q": // Galaxy S24 Ultra
                return new Kernel515(); // Samsung uses 5.15

            // Legacy devices
            default:
                // Try to detect from kernel version
                String version = getKernelVersion();
                if (version.startsWith("6.6")) return new Kernel66();
                if (version.startsWith("6.1")) return new Kernel61();
                if (version.startsWith("5.15")) return new Kernel515();
                if (version.startsWith("5.10")) return new Kernel510();
                if (version.startsWith("5.4")) return new Kernel54();
                if (version.startsWith("4.19")) return new Kernel419();

                // Default to 6.1
                return new Kernel61();
        }
    }

    /**
     * Get current kernel version from /proc/version
     */
    @NonNull
    private static String getKernelVersion() {
        try (BufferedReader reader = new BufferedReader(new FileReader("/proc/version"))) {
            String line = reader.readLine();
            if (line != null) {
                // Format: "Linux version 6.1.0-12345-gabc123 (build@host) (gcc 11.2.0, GNU ld (GNU Binutils) 2.38) #1 SMP PREEMPT Wed Oct 19 10:30:00 UTC 2026"
                String[] parts = line.split("\\s+");
                for (int i = 0; i < parts.length; i++) {
                    if (parts[i].matches("\\d+\\.\\d+(\\.\\d+)?")) {
                        return parts[i];
                    }
                }
            }
        } catch (IOException e) {
            // Fallback
        }
        return "6.1"; // Default
    }

    /**
     * Create kernel for specific Android version
     */
    @NonNull
    public static IKernelInterface createForAndroidVersion(int androidVersion) {
        if (androidVersion >= 15) {
            return new Kernel66(); // Android 15+ = 6.6
        } else if (androidVersion >= 14) {
            return new Kernel66(); // Android 14 = 6.6
        } else if (androidVersion >= 13) {
            return new Kernel61(); // Android 13/14 = 6.1
        } else if (androidVersion >= 12) {
            return new Kernel515(); // Android 12/13 = 5.15
        } else if (androidVersion >= 11) {
            return new Kernel510(); // Android 11/12 = 5.10
        } else if (androidVersion >= 10) {
            return new Kernel54(); // Android 10/11 = 5.4
        } else {
            return new Kernel419(); // Android 9- = 4.19
        }
    }
}