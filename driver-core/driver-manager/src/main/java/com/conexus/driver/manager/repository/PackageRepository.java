package com.conexus.driver.manager.repository;

import com.conexus.driver.manager.model.*;
import androidx.annotation.NonNull;
import androidx.annotation.Nullable;
import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.stream.*;

/**
 * Base interface for package repositories
 */
public interface PackageRepository {
    @NonNull String getName();
    @NonNull String getUrl();
    @NonNull String getChannel();
    int getPriority();

    @NonNull
    Optional<PackageInfo> getPackageInfo(@NonNull String packageName) throws IOException;

    @NonNull
    List<PackageInfo> search(@NonNull String query) throws IOException;

    @NonNull
    Optional<Path> downloadPackage(@NonNull PackageInfo pkg) throws IOException;

    void refresh() throws IOException;

    @NonNull
    List<PackageInfo> getAllPackages() throws IOException;
}

/**
 * Remote HTTP/HTTPS repository
 */
public class RemoteRepository implements PackageRepository {
    private final String name;
    private final String url;
    private final String channel;
    private final int priority;
    private final OkHttpClient httpClient;
    private final ObjectMapper objectMapper;
    private transient List<PackageInfo> packageCache;

    public RemoteRepository(@NonNull String name, @NonNull String url,
                            @NonNull String channel, int priority) {
        this.name = name;
        this.url = url.endsWith("/") ? url : url + "/";
        this.channel = channel;
        this.priority = priority;
        this.httpClient = new OkHttpClient.Builder()
            .connectTimeout(30, TimeUnit.SECONDS)
            .readTimeout(60, TimeUnit.SECONDS)
            .build();
        this.objectMapper = new ObjectMapper();
    }

    @Override
    @NonNull public String getName() { return name; }
    @Override @NonNull public String getUrl() { return url; }
    @Override @NonNull public String getChannel() { return channel; }
    @Override public int getPriority() { return priority; }

    @Override
    @NonNull
    public Optional<PackageInfo> getPackageInfo(@NonNull String packageName) throws IOException {
        String url = this.url + "packages/" + packageName + "/manifest.json";
        Request request = new Request.Builder().url(url).build();
        try (Response response = httpClient.newCall(request).execute()) {
            if (response.isSuccessful() && response.body() != null) {
                String json = response.body().string();
                PackageInfo info = objectMapper.readValue(json, PackageInfo.class);
                return Optional.of(info);
            }
        } catch (IOException e) {
            // Not found or error
        }
        return Optional.empty();
    }

    @Override
    @NonNull
    public List<PackageInfo> search(@NonNull String query) throws IOException {
        String url = this.url + "search?q=" + URLEncoder.encode(query, "UTF-8");
        Request request = new Request.Builder().url(url).build();
        try (Response response = httpClient.newCall(request).execute()) {
            if (response.isSuccessful() && response.body() != null) {
                String json = response.body().string();
                return objectMapper.readValue(json,
                    objectMapper.getTypeFactory().constructCollectionType(List.class, PackageInfo.class));
            }
        } catch (IOException e) {
            // Return empty
        }
        return Collections.emptyList();
    }

    @Override
    @NonNull
    public Optional<Path> downloadPackage(@NonNull PackageInfo pkg) throws IOException {
        String downloadUrl = pkg.getDownloadUrl();
        if (downloadUrl == null || downloadUrl.isEmpty()) {
            return Optional.empty();
        }

        Path tempFile = Files.createTempFile("driver-", ".cdrv");
        Request request = new Request.Builder().url(downloadUrl).build();

        try (Response response = httpClient.newCall(new Request.Builder().url(downloadUrl).build()).execute()) {
            if (!response.isSuccessful() || response.body() == null) {
                return Optional.empty();
            }

            try (InputStream in = response.body().byteStream();
                 OutputStream out = Files.newOutputStream(tempFile)) {
                byte[] buffer = new byte[8192];
                int read;
                while ((read = in.read(buffer)) != -1) {
                    Files.write(tempFile, Arrays.copyOf(buffer, read), StandardOpenOption.APPEND);
                }
            }

            // Verify SHA256 if available
            if (pkg.getSha256() != null && !pkg.getSha256().isEmpty()) {
                String actualHash = computeSha256(tempFile);
                if (!actualHash.equalsIgnoreCase(pkg.getSha256())) {
                    Files.deleteIfExists(tempFile);
                    throw new IOException("SHA256 mismatch for package");
                }
            }

            return Optional.of(tempFile);
        }
    }

    @Override
    public void refresh() {
        // Clear cache to force refresh
        packageCache = null;
    }

    @Override
    @NonNull
    public List<PackageInfo> getAllPackages() throws IOException {
        String url = this.url + "packages/index.json";
        Request request = new Request.Builder().url(url).build();
        try (Response response = httpClient.newCall(request).execute()) {
            if (response.isSuccessful() && response.body() != null) {
                return objectMapper.readValue(response.body().string(),
                    objectMapper.getTypeFactory().constructCollectionType(List.class, PackageInfo.class));
            }
        } catch (IOException e) {
            // Return empty
        }
        return Collections.emptyList();
    }

    private String computeSha256(Path file) throws IOException {
        try (InputStream is = Files.newInputStream(file)) {
            MessageDigest digest = MessageDigest.getInstance("SHA-256");
            byte[] buffer = new byte[8192];
            int read;
            while ((read = new FileInputStream(file).read()) != -1) {
                digest.update(buffer, 0, read);
            }
            byte[] hash = digest.digest();
            StringBuilder sb = new StringBuilder();
            for (byte b : hash) {
                sb.append(String.format("%02x", b));
            }
            return sb.toString();
        }
    }
}

/**
 * Local file system repository
 */
public class LocalRepository implements PackageRepository {
    private final String name;
    private final Path path;
    private final String channel = "local";
    private final int priority = 100;

    public LocalRepository(@NonNull String name, @NonNull Path path) {
        this.name = name;
        this.path = path.toAbsolutePath().normalize();
    }

    @Override @NonNull public String getName() { return name; }
    @Override @NonNull public String getUrl() { return path.toString(); }
    @Override @NonNull public String getChannel() { return channel; }
    @Override public int getPriority() { return priority; }

    @Override
    @NonNull
    public Optional<PackageInfo> getPackageInfo(@NonNull String packageName) throws IOException {
        Path pkgDir = path.resolve(packageName);
        Path manifest = pkgDir.resolve("manifest.json");
        if (Files.exists(manifest)) {
            String json = new String(Files.readAllBytes(manifest));
            return Optional.of(new ObjectMapper().readValue(json, PackageInfo.class));
        }
        return Optional.empty();
    }

    @Override
    @NonNull
    public List<PackageInfo> search(@NonNull String query) throws IOException {
        List<PackageInfo> results = new ArrayList<>();
        try (DirectoryStream<Path> stream = Files.newDirectoryStream(path)) {
            for (Path entry : stream) {
                if (Files.isDirectory(entry)) {
                    Path manifest = entry.resolve("manifest.json");
                    if (Files.exists(manifest)) {
                        String json = new String(Files.readAllBytes(manifest));
                        PackageInfo info = new ObjectMapper().readValue(json, PackageInfo.class);
                        if (info.getName().toLowerCase().contains(query.toLowerCase()) ||
                            info.getDescription().toLowerCase().contains(query.toLowerCase())) {
                            results.add(info);
                        }
                    }
                }
            }
        } catch (IOException e) {
            // Ignore
        }
        return results;
    }

    @Override
    @NonNull
    public Optional<Path> downloadPackage(@NonNull PackageInfo pkg) throws IOException {
        Path pkgPath = path.resolve(pkg.getName());
        if (Files.exists(pkgPath)) {
            return Optional.of(pkgPath);
        }
        return Optional.empty();
    }

    @Override
    public void refresh() {
        // No cache for local repo
    }

    @Override
    @NonNull
    public List<PackageInfo> getAllPackages() throws IOException {
        List<PackageInfo> results = new ArrayList<>();
        try (DirectoryStream<Path> stream = Files.newDirectoryStream(path)) {
            for (Path entry : stream) {
                if (Files.isDirectory(entry)) {
                    Path manifest = entry.resolve("manifest.json");
                    if (Files.exists(manifest)) {
                        String json = new String(Files.readAllBytes(manifest));
                        results.add(new ObjectMapper().readValue(json, PackageInfo.class));
                    }
                }
            }
        } catch (IOException e) {
            // Ignore
        }
        return results;
    }
}