//! Theme system for egui PoC

use egui::{Color32, FontFamily, FontId, TextStyle, Visuals, Style};
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
    pub fn primary(&self) -> egui::Color32 {
        self.color_from_hex(self.tokens.color.primary.get(self.current_mode()))
    }

    pub fn surface(&self) -> egui::Color32 {
        self.color_from_hex(self.tokens.color.surface.get(self.current_mode()))
    }

    pub fn on_surface(&self) -> egui::Color32 {
        self.color_from_hex(self.tokens.color.on_surface.get(self.current_mode()))
    }

    pub fn outline(&self) -> egui::Color32 {
        self.color_from_hex(self.tokens.color.outline.get(self.current_mode()))
    }

    pub fn error(&self) -> egui::Color32 {
        self.color_from_hex(self.tokens.color.error.get(self.current_mode()))
    }

    fn color_from_hex(&self, hex: &str) -> egui::Color32 {
        let hex = hex.trim_start_matches('#');
        if hex.len() == 6 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            egui::Color32::from_rgb(r, g, b)
        } else if hex.len() == 8 {
            let r = u8::from_str_radix(&hex[0..2], 16).unwrap_or(0);
            let g = u8::from_str_radix(&hex[2..4], 16).unwrap_or(0);
            let b = u8::from_str_radix(&hex[4..6], 16).unwrap_or(0);
            let a = u8::from_str_radix(&hex[6..8], 16).unwrap_or(255);
            egui::Color32::from_rgba_unmultiplied(r, g, b, a)
        } else {
            egui::Color32::BLACK
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
    pub fn font_family(&self) -> FontFamily {
        FontFamily::Name("Inter Variable")
    }

    pub fn font_id_body(&self) -> FontId {
        FontId::new(self.tokens.typography.scale.body_large.size as f32, FontFamily::Name("Inter Variable"))
    }

    pub fn font_id_title(&self) -> FontId {
        FontId::new(self.tokens.typography.scale.title_large.size as f32, FontFamily::Name("Inter Variable"))
    }

    pub fn font_id_headline(&self) -> FontId {
        FontId::new(self.tokens.typography.scale.headline_medium.size as f32, FontFamily::Name("Inter Variable"))
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

    // Generate egui Visuals for current theme
    pub fn visuals(&self) -> Visuals {
        let mut visuals = match self.mode {
            ConexusThemeMode::Light => Visuals::light(),
            ConexusThemeMode::Dark => Visuals::dark(),
            ConexusThemeMode::HighContrast => Visuals {
                override_text_color: Some(self.on_surface()),
                widgets: egui::style::Widgets {
                    noninteractive: egui::style::WidgetVisuals {
                        bg_fill: self.surface(),
                        bg_stroke: egui::Stroke::new(1.0, self.outline()),
                        fg_stroke: egui::Stroke::new(1.0, self.on_surface()),
                        rounding: egui::Rounding::same(self.corner_radius("medium") as f32),
                        expansion: 0.0,
                    },
                    inactive: egui::style::WidgetVisuals {
                        bg_fill: self.surface(),
                        bg_stroke: egui::Stroke::new(1.0, self.outline()),
                        fg_stroke: egui::Stroke::new(1.0, self.on_surface()),
                        rounding: egui::Rounding::same(self.corner_radius("medium") as f32),
                        expansion: 0.0,
                    },
                    hovered: egui::style::WidgetVisuals {
                        bg_fill: self.primary().linear_multiply(0.1),
                        bg_stroke: egui::Stroke::new(1.0, self.primary()),
                        fg_stroke: egui::Stroke::new(1.0, self.on_surface()),
                        rounding: egui::Rounding::same(self.corner_radius("medium") as f32),
                        expansion: 1.0,
                    },
                    active: egui::style::WidgetVisuals {
                        bg_fill: self.primary().linear_multiply(0.2),
                        bg_stroke: egui::Stroke::new(1.0, self.primary()),
                        fg_stroke: egui::Stroke::new(1.0, egui::Color32::WHITE),
                        rounding: egui::Rounding::same(self.corner_radius("medium") as f32),
                        expansion: 2.0,
                    },
                    open: egui::style::WidgetVisuals {
                        bg_fill: self.primary().linear_multiply(0.1),
                        bg_stroke: egui::Stroke::new(1.0, self.primary()),
                        fg_stroke: egui::Stroke::new(1.0, self.on_surface()),
                        rounding: egui::Rounding::same(self.corner_radius("medium") as f32),
                        expansion: 0.0,
                    },
                },
                ..Visuals::dark()
            },
        };

        // Override colors
        visuals.override_text_color = Some(self.on_surface());
        visuals.panel_fill = self.surface();
        visuals.window_fill = self.surface();
        visuals.extreme_bg_color = self.surface();
        visuals.faint_bg_color = self.surface().linear_multiply(0.9);
        visuals.extreme_bg_color = self.surface();
        visuals.code_bg_color = self.surface().linear_multiply(0.8);
        visuals.warn_fg_color = self.error();
        visuals.error_fg_color = self.error();
        visuals.hyperlink_color = self.primary();
        
        visuals
    }

    // Helper color getters
    pub fn primary(&self) -> Color32 {
        self.color_from_hex(self.tokens.color.primary.get(self.current_mode()))
    }

    pub fn surface(&self) -> Color32 {
        self.color_from_hex(self.tokens.color.surface.get(self.current_mode()))
    }

    pub fn on_surface(&self) -> Color32 {
        self.color_from_hex(self.tokens.color.on_surface.get(self.current_mode()))
    }

    pub fn outline(&self) -> Color32 {
        self.color_from_hex(self.tokens.color.outline.get(self.current_mode()))
    }

    pub fn error(&self) -> Color32 {
        self.color_from_hex(self.tokens.color.error.get(self.current_mode()))
    }

    // Spacing
    pub fn spacing_base(&self) -> f32 {
        self.tokens.spacing.base as f32
    }

    pub fn spacing(&self, index: usize) -> f32 {
        self.tokens.spacing.scale.get(index).copied().unwrap_or(0) as f32
    }

    // Typography
    pub fn font_id_body(&self) -> FontId {
        FontId::new(self.tokens.typography.scale.body_large.size as f32, FontFamily::Name("Inter Variable"))
    }

    pub fn font_id_title(&self) -> FontId {
        FontId::new(self.tokens.typography.scale.title_large.size as f32, FontFamily::Name("Inter Variable"))
    }

    pub fn font_id_headline(&self) -> FontId {
        FontId::new(self.tokens.typography.scale.headline_medium.size as f32, FontFamily::Name("Inter Variable"))
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
}

use egui::{Color32, FontFamily, FontId, TextStyle, Visuals, Style};