pluginManagement {
    repositories {
        gradlePluginPortal()
        google()
        mavenCentral()
    }
    resolutionStrategy {
        eachPlugin {
            if (requested.id.id == "com.android.application" || requested.id.id == "com.android.library") {
                useModule("com.android.tools.build:gradle:8.2.0")
            }
            if (requested.id.id == "org.jetbrains.kotlin.android" || requested.id.id == "org.jetbrains.kotlin.multiplatform") {
                useModule("org.jetbrains.kotlin:kotlin-gradle-plugin:1.9.22")
            }
        }
    }
}

repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)

repositories {
    google()
    mavenCentral()
    maven { url = uri("https://jitpack.io") }
    maven { url = uri("https://maven.pkg.github.com/conexus-mobile-os/conexus-sdk") }
}

dependencyResolutionManagement {
    repositoriesMode.set(RepositoriesMode.FAIL_ON_PROJECT_REPOS)
    repositories {
        google()
        mavenCentral()
        maven { url = uri("https://jitpack.io") }
        maven { url = uri("https://maven.pkg.github.com/conexus-mobile-os/conexus-sdk") }
    }
}

rootProject.name = "conexus-mobile-os"

include(":conexus-sdk:kotlin")
include(":conexus-sdk:typescript")
include(":conexus-services:tile-engine")
include(":conexus-services:notification-broker")
include(":conexus-services:app-manager")
include(":conexus-services:search")
include(":conexus-services:update")
include(":conexus-services:security")
include(":conexus-services:bridge-android")
include(":platforms:android")
include(":site")
include(":conexus-launcher")