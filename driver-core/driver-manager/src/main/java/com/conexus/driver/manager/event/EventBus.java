package com.conexus.driver.manager.event;

import androidx.annotation.NonNull;
import java.util.*;
import java.util.concurrent.*;
import java.util.concurrent.atomic.*;

/**
 * Event Bus for driver manager events
 */
public class EventBus {
    private final Map<Class<?>, List<EventSubscription>> subscribers = new ConcurrentHashMap<>();
    private final ExecutorService executor = Executors.newFixedThreadPool(2);

    public void subscribe(@NonNull DriverEventListener listener) {
        Class<?>[] eventTypes = listener.getEventTypes();
        for (Class<?> eventType : eventTypes) {
            subscribers.computeIfAbsent(eventType, k -> new CopyOnWriteArrayList<>())
                .add(new EventSubscription(listener));
        }
    }

    public void unsubscribe(@NonNull DriverEventListener listener) {
        for (List<EventSubscription> subs : subscribers.values()) {
            subs.removeIf(s -> s.listener == listener);
        }
    }

    @SuppressWarnings("unchecked")
    public void publish(@NonNull DriverEvent event) {
        Class<?> eventClass = event.getClass();
        List<EventSubscription> subs = subscribers.get(eventClass);
        if (subs != null) {
            for (EventSubscription sub : subs) {
                executor.execute(() -> {
                    try {
                        ((DriverEventListener<Object>) sub.listener).onEvent(event);
                    } catch (Exception e) {
                        // Log error but don't fail event bus
                    }
                });
            }
        }
    }

    public void shutdown() {
        executor.shutdown();
    }

    private static class EventSubscription {
        final DriverEventListener<?> listener;

        EventSubscription(@NonNull DriverEventListener<?> listener) {
            this.listener = listener;
        }
    }
}

/**
 * Base event class
 */
public class DriverEvent {
    public enum Type {
        STARTED,
        COMPLETED,
        FAILED,
        PROGRESS
    }

    private final Type type;
    private final long timestamp;
    private final String source;

    public DriverEvent(@NonNull Type type, @NonNull String source) {
        this.type = type;
        this.timestamp = System.currentTimeMillis();
        this.source = source;
    }

    @NonNull public Type getType() { return type; }
    public long getTimestamp() { return timestamp; }
    @NonNull public String getSource() { return source; }
}

/**
 * Transaction Event
 */
public class TransactionEvent extends DriverEvent {
    public enum TransactionType {
        INSTALL,
        UPDATE,
        REMOVE,
        ROLLBACK
    }

    private final TransactionType transactionType;
    private final String transactionId;
    private final String errorMessage;

    public TransactionEvent(@NonNull Type type, @NonNull Transaction transaction) {
        super(Type.STARTED, "TransactionManager");
        this.transactionType = parseType(transaction.getType());
        this.transactionId = transaction.getId();
        this.errorMessage = null;
    }

    public TransactionEvent(@NonNull Type type, @NonNull Transaction transaction,
                           @Nullable String errorMessage) {
        super(type, "TransactionManager");
        this.transactionType = parseType(transaction.getType());
        this.transactionId = transaction.getId();
        this.errorMessage = errorMessage;
    }

    private TransactionType parseType(@NonNull String type) {
        switch (type) {
            case "INSTALL": return TransactionType.INSTALL;
            case "UPDATE": return TransactionType.UPDATE;
            case "REMOVE": return TransactionType.REMOVE;
            default: return TransactionType.INSTALL;
        }
    }

    public TransactionType getTransactionType() { return transactionType; }
    String getTransactionId() { return transactionId; }
    String getErrorMessage() { return errorMessage; }
}

/**
 * Package Event
 */
public class PackageEvent extends DriverEvent {
    public enum PackageEventType {
        INSTALLED,
        UPDATED,
        REMOVED,
        VERIFIED,
        FAILED
    }

    private final String packageName;
    private final String version;
    private final PackageEventType eventType;

    public PackageEvent(@NonNull String source, @NonNull String packageName,
                        @NonNull String version, @NonNull PackageEventType eventType) {
        super(PackageEvent.Type.valueOf(eventType.name()), source);
        this.packageName = packageName;
        this.version = version;
        this.eventType = eventType;
    }

    String getPackageName() { return packageName; }
    String getVersion() { return version; }
    PackageEventType getEventType() { return eventType; }
}

/**
 * Package Event Type enum for PackageEvent
 */
enum PackageEventType {
    INSTALLED, UPDATED, REMOVED, VERIFIED, FAILED
}

/**
 * Driver Event Listener interface
 */
public interface DriverEventListener<T extends DriverEvent> {
    void onEvent(@NonNull T event);

    @NonNull
    Class<?>[] getEventTypes();
}

/**
 * Typed Event Listener
 */
public interface TypedEventListener<T extends DriverEvent> extends DriverEventListener<T> {
    @Override
    void onEvent(@NonNull T event);

    @Override
    default Class<?>[] getEventTypes() {
        return new Class[]{getEventClass()};
    }

    @NonNull
    Class<T> getEventClass();
}

/**
 * Event Bus Builder
 */
public class EventBusBuilder {
    private final EventBus eventBus = new EventBus();

    public EventBusBuilder addListener(@NonNull DriverEventListener<?> listener) {
        eventBus.subscribe(listener);
        return this;
    }

    @NonNull
    public EventBus build() {
        return eventBus;
    }
}