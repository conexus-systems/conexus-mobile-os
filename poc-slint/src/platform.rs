//! Platform abstraction for cross-platform support

use slint::ComponentHandle;
use std::error::Error;

pub trait Platform: Send + Sync {
    fn init(&self, window: &dyn ComponentHandle);
    fn name(&self) -> &'static str;
}

pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl Platform for LinuxPlatform {
    fn init(&self, window: &dyn ComponentHandle) {
        println!("Linux platform initialized");
        // Linux-specific setup: Wayland/X11, DBus, etc.
    }
    
    fn name(&self) -> &'static str {
        "linux"
    }
}

#[cfg(target_os = "android")]
pub struct AndroidPlatform;

#[cfg(target_os = "android")]
impl AndroidPlatform {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(target_os = "android")]
impl Platform for AndroidPlatform {
    fn init(&self, window: &dyn ComponentHandle) {
        println!("Android platform initialized");
        // Android-specific setup: JNI, lifecycle, etc.
    }
    
    fn name(&self) -> &'static str {
        "android"
    }
}

#[cfg(not(target_os = "android"))]
pub struct AndroidPlatform;

#[cfg(not(target_os = "android"))]
impl AndroidPlatform {
    pub fn new() -> Self {
        Self
    }
}

#[cfg(not(target_os = "android"))]
impl Platform for AndroidPlatform {
    fn init(&self, window: &dyn ComponentHandle) {
        println!("Android platform stub (not on Android)");
    }
    
    fn name(&self) -> &'static str {
        "android-stub"
    }
}