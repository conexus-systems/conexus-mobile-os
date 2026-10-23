//! Platform abstraction for egui PoC

use eframe::egui;

pub trait Platform: Send + Sync {
    fn init(&self);
    fn name(&self) -> &'static str;
}

pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl Platform for LinuxPlatform {
    fn init(&self) {
        println!("Linux platform initialized (egui)");
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
    fn init(&self) {
        println!("Android platform initialized (egui)");
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
    fn init(&self) {
        println!("Android platform stub (not on Android)");
    }
    
    fn name(&self) -> &'static str {
        "android-stub"
    }
}