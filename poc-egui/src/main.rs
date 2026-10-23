//! Conexus Harmattan PoC — egui Framework
//! 
//! HomeScreen com: Grid responsivo, 3 tiles (S/M/W), PivotTabs, Theme toggle
//! Compila e roda em Linux (Wayland/X11) + Android (NDK via winit)

mod models;
mod tokens;
mod theme;
mod home_screen;
mod tile;
mod pivot_tabs;
mod platform;

use eframe::egui;
use tracing_subscriber;

use crate::models::{Tile, TileSize, TileContent, ThemeMode, DesignTokens};
use crate::home_screen::HomeScreenApp;
use crate::platform::{Platform, LinuxPlatform, AndroidPlatform};

fn main() -> eframe::Result<()> {
    // Initialize tracing
    tracing_subscriber::fmt::init();

    // Load design tokens
    let tokens = DesignTokens::load_from_file("design-tokens.json")
        .expect("Failed to load design-tokens.json");

    // Create platform-specific implementation
    let platform: Box<dyn Platform> = if cfg!(target_os = "android") {
        Box::new(AndroidPlatform::new())
    } else {
        Box::new(LinuxPlatform::new())
    };

    // Run the application
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size([360.0, 640.0])
            .with_min_inner_size([320.0, 480.0])
            .with_title("Conexus Harmattan - egui PoC")
            .with_resizable(true)
            .with_decorations(true)
            .with_transparent(false),
        renderer: eframe::Renderer::Wgpu, // Use wgpu for better performance
        ..Default::default()
    };

    eframe::run_native(
        "Conexus Harmattan - egui PoC",
        options,
        Box::new(|cc| {
            // Load design tokens
            let tokens = DesignTokens::load_from_file("design-tokens.json")
                .expect("Failed to load design-tokens.json");
            
            // Create platform-specific implementation
            let platform: Box<dyn Platform> = if cfg!(target_os = "android") {
                Box::new(AndroidPlatform::new())
            } else {
                Box::new(LinuxPlatform::new())
            };

            // Initialize platform
            platform.init();
            
            // Create app
            Box::new(HomeScreenApp::new(tokens, platform))
        })
    }
}

fn create_sample_tiles() -> Vec<crate::models::Tile> {
    use crate::models::{Tile, TileSize, TileContent, TextAlign};
    
    vec![
        crate::models::Tile {
            id: "tile-email".to_string(),
            size: TileSize::Small,
            title: "Email".to_string(),
            front_content: TileContent::Icon {
                name: "mail".to_string(),
                color: "#0066CC".to_string(),
            },
            back_content: Some(TileContent::Text {
                text: "3 unread messages".to_string(),
                align: TextAlign::Center,
            }),
            badge: Some(3),
        },
        crate::models::Tile {
            id: "tile-calendar".to_string(),
            size: TileSize::Medium,
            title: "Calendário".to_string(),
            front_content: TileContent::Text {
                text: "Hoje: 3 reuniões\n14:00 - Team sync\n16:00 - Review".to_string(),
                align: TextAlign::Left,
            },
            back_content: Some(TileContent::Icon {
                name: "event".to_string(),
                color: "#00A300".to_string(),
            }),
            badge: None,
        },
        crate::models::Tile {
            id: "tile-weather".to_string(),
            size: TileSize::Wide,
            title: "Clima".to_string(),
            front_content: TileContent::Text {
                text: "São Paulo\n24°C · Parcialmente nublado\nMáx: 28°C Min: 19°C".to_string(),
                align: TextAlign::Center,
            },
            back_content: Some(TileContent::Icon {
                name: "wb_sunny".to_string(),
                color: "#FA6800".to_string(),
            }),
            badge: None,
        },
    ]
}