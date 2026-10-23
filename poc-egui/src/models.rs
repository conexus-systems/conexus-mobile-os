//! Data models shared across PoCs

use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TileSize {
    Small,   // 1x1
    Medium,  // 2x2
    Wide,    // 4x2
    Large,   // 4x4
}

impl TileSize {
    pub fn grid_cols(&self) -> u32 {
        match self {
            TileSize::Small => 1,
            TileSize::Medium => 2,
            TileSize::Wide => 4,
            TileSize::Large => 4,
        }
    }

    pub fn grid_rows(&self) -> u32 {
        match self {
            TileSize::Small => 1,
            TileSize::Medium => 2,
            TileSize::Wide => 2,
            TileSize::Large => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TextAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TileContent {
    Icon { name: String, color: String },
    Text { text: String, align: TextAlign },
    Image { url: String, fit: ImageFit },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ImageFit {
    Cover,
    Contain,
    Fill,
    ScaleDown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tile {
    pub id: String,
    pub size: TileSize,
    pub title: String,
    pub front_content: TileContent,
    pub back_content: Option<TileContent>,
    pub badge: Option<u32>, // 1-99, None = no badge
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ThemeMode {
    Light,
    Dark,
    HighContrast,
}

impl ThemeMode {
    pub fn next(&self) -> Self {
        match self {
            ThemeMode::Light => ThemeMode::Dark,
            ThemeMode::Dark => ThemeMode::HighContrast,
            ThemeMode::HighContrast => ThemeMode::Light,
        }
    }

    pub fn as_str(&self) -> &'static str {
        match self {
            ThemeMode::Light => "light",
            ThemeMode::Dark => "dark",
            ThemeMode::HighContrast => "hc",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DesignTokens {
    pub color: ColorTokens,
    pub typography: TypographyTokens,
    pub spacing: SpacingTokens,
    pub shape: ShapeTokens,
    pub motion: MotionTokens,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ColorTokens {
    pub primary: ThemeColor,
    pub surface: ThemeColor,
    pub on_surface: ThemeColor,
    pub outline: ThemeColor,
    pub error: ThemeColor,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ThemeColor {
    pub light: String,
    pub dark: String,
    pub hc: String,
}

impl ThemeColor {
    pub fn get(&self, mode: ThemeMode) -> &str {
        match mode {
            ThemeMode::Light => &self.light,
            ThemeMode::Dark => &self.dark,
            ThemeMode::HighContrast => &self.hc,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyTokens {
    pub font_family: String,
    pub scale: TypographyScale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypographyScale {
    pub display_large: TypeScale,
    pub headline_medium: TypeScale,
    pub title_large: TypeScale,
    pub body_large: TypeScale,
    pub label_large: TypeScale,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TypeScale {
    pub size: u32,
    pub weight: u32,
    pub line_height: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpacingTokens {
    pub base: u32,
    pub scale: Vec<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ShapeTokens {
    pub corner_none: u32,
    pub corner_small: u32,
    pub corner_medium: u32,
    pub corner_large: u32,
    pub corner_full: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MotionTokens {
    pub duration_short: u32,
    pub duration_medium: u32,
    pub duration_long: u32,
    pub easing_standard: String,
}

impl DesignTokens {
    pub fn load_from_file(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let content = std::fs::read_to_string(path)?;
        let tokens: DesignTokens = serde_json::from_str(&content)?;
        Ok(tokens)
    }
}