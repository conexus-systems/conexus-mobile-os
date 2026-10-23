//! Conexus Harmattan PoC — Iced Framework
//! 
//! HomeScreen com: Grid responsivo, 3 tiles (S/M/W), PivotTabs, Theme toggle
//! Compila e roda em Linux (Wayland/X11) + Android (NDK via wgpu)

mod models;
mod tokens;
mod theme;
mod home_screen;
mod apps_grid;
mod pivot_tabs;
mod tile;
mod platform;

use iced::{Application, Command, Element, Settings, Theme, window};
use tracing_subscriber;

use crate::models::{Tile, TileSize, TileContent, ThemeMode, DesignTokens};
use crate::home_screen::HomeScreen;
use crate::platform::{Platform, LinuxPlatform, AndroidPlatform};

fn main() -> iced::Result {
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
    HomeScreen::run(Settings {
        window: window::Settings {
            size: (360, 640),
            min_size: Some((320, 480)),
            max_size: None,
            visible: true,
            resizable: true,
            decorations: true,
            transparent: false,
            always_on_top: false,
            icon: None,
            platform_specific: Default::default(),
        },
        default_font: iced::Font::DEFAULT,
        fonts: vec![],
        antialiasing: true,
        ..Default::default()
    })
}

pub fn create_sample_tiles() -> Vec<Tile> {
    use crate::models::{Tile, TileSize, TileContent, TextAlign};
    
    vec![
        Tile {
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
        Tile {
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
        Tile {
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