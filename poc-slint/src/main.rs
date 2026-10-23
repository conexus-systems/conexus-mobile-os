//! Conexus Harmattan PoC — Slint Framework
//! 
//! HomeScreen com: Grid responsivo, 3 tiles (S/M/W), PivotTabs, Theme toggle
//! Compila e roda em Linux (Wayland/X11) + Android (NDK)

mod models;
mod tokens;
mod home_screen;
mod apps_grid;
mod pivot_tabs;
mod theme_provider;
mod platform;

use slint::{ComponentHandle, Model, ModelRc, VecModel};
use std::rc::Rc;
use std::cell::RefCell;
use std::sync::Arc;
use tracing_subscriber;

use crate::models::{Tile, TileSize, TileContent, ThemeMode, DesignTokens};
use crate::home_screen::HomeScreen;
use crate::platform::{Platform, LinuxPlatform, AndroidPlatform};

fn main() -> Result<(), slint::PlatformError> {
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

    // Create the main window
    let window = HomeScreen::new()?;

    // Set up theme provider
    let theme_provider = ThemeProvider::new(tokens.clone());
    window.global::<ThemeProvider>().set_theme_mode(theme_provider.current_theme());

    // Create sample tiles
    let tiles = create_sample_tiles();
    let tiles_model = Rc::new(VecModel::from(tiles));
    window.set_tiles(ModelRc::new(tiles_model.clone()));

    // Connect theme toggle
    let theme_provider_handle = theme_provider.clone();
    let window_handle = window.as_weak();
    window.global::<ThemeProvider>().on_toggle_theme(move || {
        let new_theme = theme_provider_handle.toggle();
        if let Some(w) = window_handle.upgrade() {
            w.global::<ThemeProvider>().set_theme_mode(new_theme);
        }
    });

    // Connect pivot tab change
    let tiles_model_clone = tiles_model.clone();
    window.on_pivot_changed(move |index| {
        tracing::info!("Pivot changed to index: {}", index);
        // Filter tiles based on pivot (placeholder)
        let _ = tiles_model_clone;
    });

    // Connect tile click
    let tiles_model_click = tiles_model.clone();
    window.on_tile_clicked(move |index| {
        tracing::info!("Tile clicked: index {}", index);
        if let Some(tile) = tiles_model_click.iter().nth(index as usize) {
            tracing::info!("Tile: {} ({:?})", tile.title, tile.size);
        }
    });

    // Platform-specific initialization
    platform.init(&window);

    // Run the event loop
    window.run()
}

fn create_sample_tiles() -> Vec<Tile> {
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
                align: crate::models::TextAlign::Center,
            }),
            badge: Some(3),
        },
        Tile {
            id: "tile-calendar".to_string(),
            size: TileSize::Medium,
            title: "Calendário".to_string(),
            front_content: TileContent::Text {
                text: "Hoje: 3 reuniões\n14:00 - Team sync\n16:00 - Review".to_string(),
                align: crate::models::TextAlign::Left,
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
                align: crate::models::TextAlign::Center,
            },
            back_content: Some(TileContent::Icon {
                name: "wb_sunny".to_string(),
                color: "#FA6800".to_string(),
            }),
            badge: None,
        },
    ]
}