package com.conexus.driver.manager.repository;

import com.conexus.driver.manager.model.*;
import android.content.Context;
import android.util.Log;
import java.io.*;
import java.nio.file.*;
import java.util.*;
import java.util.concurrent.*;
import java.util.stream.*;

/**
 * Repository Manager - Manages package repositories
 */
public class RepositoryManager {
    private static final String TAG = "RepositoryManager";

    private final Configuration config;
    private final Context context;
    private final Map<String, PackageRepository> repositories = new ConcurrentHashMap<>();
    private final ExecutorService executor = Executors.newFixedThreadPool(4);

    public RepositoryManager(@NonNull Configuration config) {
        this.config = config;
        this.context = null; // Will be set via setter
    }

    public void setContext(Context context) {
        this.context = context;
    }

    public void initialize() {
        Log.i("RepositoryManager", "Initializing Repository Manager");
    }

    public void addRemoteRepository(@NonNull String name, @NonNull String url,
                                    @NonNull String channel, int priority) {
        RemoteRepository repo = new RemoteRepository(name, url, channel, priority);
        repositories.put(name, repo);
    }

    public void addLocalRepository(@NonNull String name, @NonNull Path path) {
        LocalRepository repo = new LocalRepository(name, path);
        repositories.put(name, repo);
    }

    public void removeRepository(@NonNull String name) {
        repositories.remove(name);
    }

    @NonNull
    public Optional<PackageRepository> getRepository(@NonNull String name) {
        return Optional.ofNullable(repositories.get(name));
    }

    @NonNull
    public List<PackageRepository> getRepositories() {
        return new ArrayList<>(repositories.values());
    }

    @NonNull
    public List<PackageRepository> getRepositoriesByChannel(@NonNull String channel) {
        return repositories.values().stream()
            .filter(r -> r.getChannel().equals(channel))
            .collect(Collectors.toList());
    }

    @NonNull
    public CompletableFuture<Optional<PackageInfo>> getPackageInfo(@NonNull String packageName) {
        return CompletableFuture.supplyAsync(() -> {
            // Search all repositories
            for (PackageRepository repo : repositories.values()) {
                try {
                    Optional<PackageInfo> info = repo.getPackageInfo();
                    if (info.isPresent()) return info;
                } catch (Exception e) {
                    // Continue to next repo
                }
            }
            return Optional.empty();
        }, executor);
    }

    @NonNull
    public CompletableFuture<List<PackageInfo>> search(@NonNull String query,
                                                       @Nullable SearchOptions options) {
        return CompletableFuture.supplyAsync(() -> {
            List<PackageInfo> results = new ArrayList<>();
            for (PackageRepository repo : repositories.values()) {
                try {
                    results.addAll(repo.search(query));
                } catch (Exception e) {
                    // Ignore and continue
                }
            }
            // Sort by relevance
            return results.stream()
                .limit(options != null ? options.getLimit() : 20)
                .collect(Collectors.toList());
        }, executor);
    }

    @NonNull
    public CompletableFuture<Optional<Path>> downloadPackage(@NonNull PackageInfo pkg) {
        return CompletableFuture.supplyAsync(() -> {
            for (PackageRepository repo : repositories.values()) {
                try {
                    Optional<Path> path = repo.downloadPackage(pkg);
                    if (path.isPresent()) return path;
                } catch (Exception e) {
                    // Continue to next repo
                }
            }
            return Optional.empty();
        }, executor);
    }

    public void refreshAll() {
        for (PackageRepository repo : repositories.values()) {
            try {
                repo.refresh();
            } catch (Exception e) {
                // Log error but continue
            }
        }
    }

    public void shutdown() {
        executor.shutdown();
    }

    private static class ExecutorHolder {
        static final ExecutorService executor = Executors.newFixedThreadPool(4);
    }
}