//! Theme system for Iced PoC

use iced::{Color, Theme as IcedTheme};
use crate::models::{ThemeMode, DesignTokens};
use crate::tokens;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConexusThemeMode {
    Light,
    Dark,
    HighContrast,
}

impl From<ThemeMode> for ConexusThemeMode {
    fn from(mode: ThemeMode) -> Self {
        match mode {
            ThemeMode::Light => ConexusThemeMode::Light,
            ThemeMode::Dark => ConexusThemeMode::Dark,
            ThemeMode::HighContrast => ConexusThemeMode::HighContrast,
        }
    }
}

impl From<ConexusThemeMode> for ThemeMode {
    fn from(mode: ConexusThemeMode) -> Self {
        match mode {
            ConexusThemeMode::Light => ThemeMode::Light,
            ConexusThemeMode::Dark => ThemeMode::Dark,
            ConexusThemeMode::HighContrast => ThemeMode::HighContrast,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ConexusTheme {
    pub mode: ConexusThemeMode,
    pub tokens: &'static DesignTokens,
}

impl ConexusTheme {
    pub fn new(mode: ThemeMode) -> Self {
        Self {
            mode: mode.into(),
            tokens: tokens(),
        }
    }

    pub fn toggle(&mut self) {
        self.mode = match self.mode {
            ConexusThemeMode::Light => ConexusThemeMode::Dark,
            ConexusThemeMode::Dark => ConexusThemeMode::HighContrast,
            ConexusThemeMode::HighContrast => ConexusThemeMode::Light,
        };
    }

    pub fn current_mode(&self) -> ThemeMode {
        self.mode.into()
    }

    // Color getters for current theme
    pub fn primary(&self) -> Color {
        self.color_from_hex(self.tokens.color.primary.get(self.current_mode()))
    }

    pub fn surface(&self) -> Color {
        self.color_from_hex(self.tokens.color.surface.get(self.current_mode()))
    }

    pub fn on_surface(&self) -> Color {
        self.color_from_hex(self.tokens.color.on_surface.get(self.current_mode()))
    }

    pub fn outline(&self) -> Color {
        self.color_from_hex(self.tokens.color.outline.get(self.current_mode()))
    }

    pub fn error(&self) -> Color {
        self.color_from_hex(self.tokens.color.error.get(self.current_mode()))
    }

    fn color_from_hex(&self, hex: &str) -> Color {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            Color::from_rgb8(r, g, b)
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            Color::from_rgba8(r, g, b, a)
        } else {
            Color::BLACK
        }
    }

    // Spacing
    pub fn spacing_base(&self) -> f32 {
        self.tokens.spacing.base as f32
    }

    pub fn spacing(&self, index: usize) -> f32 {
        self.tokens.spacing.scale.get(index).copied().unwrap_or(0) as f32
    }

    // Typography
    pub fn font_family(&self) -> iced::Font {
        iced::Font {
            family: iced::font::Family::Name("Inter Variable"),
            ..Default::default()
        }
    }

    pub fn font_size_body(&self) -> f32 {
        self.tokens.typography.scale.body_large.size as f32
    }

    pub fn font_size_title(&self) -> f32 {
        self.tokens.typography.scale.title_large.size as f32
    }

    pub fn font_size_headline(&self) -> f32 {
        self.tokens.typography.scale.headline_medium.size as f32
    }

    // Motion
    pub fn duration_short(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.tokens.motion.duration_short as u64)
    }

    pub fn duration_medium(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.tokens.motion.duration_medium as u64)
    }

    pub fn duration_long(&self) -> std::time::Duration {
        std::time::Duration::from_millis(self.tokens.motion.duration_long as u64)
    }

    // Shapes
    pub fn corner_radius(&self, size: &str) -> f32 {
        match size {
            "none" => self.tokens.shape.corner_none as f32,
            "small" => self.tokens.shape.corner_small as f32,
            "medium" => self.tokens.shape.corner_medium as f32,
            "large" => self.tokens.shape.corner_large as f32,
            "full" => self.tokens.shape.corner_full as f32,
            _ => 0.0,
        }
    }

    // Convert to Iced Theme for compatibility
    pub fn to_iced_theme(&self) -> IcedTheme {
        match self.mode {
            ConexusThemeMode::Light => IcedTheme::Light,
            ConexusThemeMode::Dark => IcedTheme::Dark,
            ConexusThemeMode::HighContrast => IcedTheme::Light, // Custom would need custom palette
        }
    }
}