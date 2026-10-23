//! Platform abstraction for Iced PoC

use iced::Task;

pub trait Platform: Send + Sync {
    fn init(&self) -> Task<crate::home_screen::Message>;
    fn name(&self) -> &'static str;
}

pub struct LinuxPlatform;

impl LinuxPlatform {
    pub fn new() -> Self {
        Self
    }
}

impl Platform for LinuxPlatform {
    fn init(&self) -> Task<crate::home_screen::Message> {
        println!("Linux platform initialized (Iced)");
        Task::none()
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
    fn init(&self) -> Task<crate::home_screen::Message> {
        println!("Android platform initialized (Iced)");
        Task::none()
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
    fn init(&self) -> Task<crate::home_screen::Message> {
        println!("Android platform stub (not on Android)");
        Task::none()
    }
    
    fn name(&self) -> &'static str {
        "android-stub"
    }
}