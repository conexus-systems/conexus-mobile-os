//! Tile Widget for egui PoC

use egui::{Color32, FontId, FontFamily, RichText, Sense, Stroke, Rounding, Vec2, Response, Ui, Widget, Layout, Align2, Color32};
use crate::models::{Tile, TileSize, TileContent, TextAlign, ImageFit};
use crate::theme::ConexusTheme;

pub struct TileWidget {
    tile: Tile,
    theme: &'static crate::theme::ConexusTheme,
    is_flipping: bool,
    flip_progress: f32,
}

impl TileWidget {
    pub fn new(tile: Tile) -> Self {
        // We'll pass theme reference when rendering
        Self {
            tile,
            theme: &crate::theme::tokens().into(), // This won't work - need to pass theme reference
            is_flipping: false,
            flip_progress: 0.0,
        }
    }

    pub fn ui(&mut self, ui: &mut egui::Ui, theme: &crate::theme::ConexusTheme) -> Response {
        let tile = &self.tile;
        let theme_ref = theme;
        
        let size_class = match tile.size {
            TileSize::Small => "small",
            TileSize::Medium => "medium",
            TileSize::Wide => "wide",
            TileSize::Large => "large",
        };

        let (width, height) = self.tile_size(tile.size, ui.available_width());
        
        let desired_size = egui::vec2(width, height);
        let (rect, response) = ui.allocate_exact_size(desired_size, Sense::click());
        
        if response.clicked() {
            println!("Tile clicked: {}", tile.id);
        }

        // Render tile content
        if ui.is_rect_visible(rect) {
            let painter = ui.painter();
            
            // Tile background
            painter.rect_filled(
                rect,
                egui::Rounding::same(theme.corner_radius("medium")),
                theme.surface(),
            );
            
            // Border
            painter.rect_stroke(
                rect,
                egui::Rounding::same(theme.corner_radius("medium")),
                egui::Stroke::new(1.0, theme.outline()),
            );
            
            // Shadow
            painter.rect_filled(
                rect.expand(4.0),
                egui::Rounding::same(theme.corner_radius("medium") + 4.0),
                Color32::from_black_alpha(40),
            );
            
            // Tile content
            let content_rect = rect.shrink(16.0);
            let mut y = content_rect.min.y;
            
            // Badge (top right)
            if let Some(badge) = self.tile.badge {
                let badge_text = if badge > 99 { "99+".to_string() } else { badge.to_string() };
                let badge_rect = Rect::from_min_max(
                    Pos2::new(content_rect.max.x - 60.0, content_rect.min.y + 8.0),
                    Pos2::new(content_rect.max.x - 8.0, content_rect.min.y + 28.0),
                );
                
                ui.painter().rect_filled(
                    badge_rect,
                    Rounding::same(12.0),
                    theme.error(),
                );
                
                ui.painter().text(
                    badge_rect.center(),
                    Align2::CENTER_CENTER,
                    if badge > 99 { "99+" } else { &badge.to_string() },
                    FontId::new(12.0, FontFamily::Name("Inter Variable")),
                    Color32::WHITE,
                );
            }
            
            // Front content
            let content_y = self.render_front_content(ui, theme, &content_rect);
            
            // Title
            let title_rect = Rect::from_min_max(
                Pos2::new(content_rect.min.x, y),
                Pos2::new(content_rect.max.x, y + 24.0),
            );
            
            ui.painter().text(
                title_rect.center(),
                Align2::CENTER_CENTER,
                &self.tile.title,
                FontId::new(16.0, FontFamily::Name("Inter Variable")),
                theme.on_surface(),
            );
            
            // Badge count
            if let Some(badge) = self.tile.badge {
                let badge_text = if badge > 99 { "99+" } else { badge.to_string() };
                ui.painter().text(
                    Pos2::new(content_rect.center().x, y + 36.0),
                    Align2::CENTER_CENTER,
                    if badge > 99 { "99+" } else { &badge.to_string() },
                    FontId::new(12.0, FontFamily::Name("Inter Variable")),
                    theme.error(),
                );
            }
        }
        
        response
    }
    
    fn tile_size(&self, size: TileSize, available_width: f32) -> (f32, f32) {
        let base_width = available_width / 4.0; // Assuming 4 columns base
        let tile_width = base_width * size.grid_cols() as f32 - 16.0; // spacing
        let tile_height = tile_width * size.grid_rows() as f32 / size.grid_cols() as f32;
        (tile_width.max(80.0), tile_height.max(80.0))
    }
    
    fn render_front_content(&self, ui: &mut egui::Ui, theme: &crate::theme::ConexusTheme, rect: &egui::Rect) -> f32 {
        let tile = &self.tile;
        let mut y = rect.min.y + 16.0;
        
        match &tile.front_content {
            TileContent::Icon { name, color } => {
                // Placeholder for icon
                let text = format!("[{}]", name);
                let color = color_from_hex(color).unwrap_or_else(|| self.primary());
                
                let text_galley = ui.painter().layout(
                    text,
                    FontId::new(32.0, FontFamily::Name("Inter Variable")),
                    color_from_hex(color).unwrap_or_else(|| self.primary()),
                );
                
                let pos = Pos2::new(rect.center().x, rect.min.y + 16.0 + text_galley.size().y / 2.0);
                ui.painter().galley(pos, text_galley, color);
                
                32.0 + 16.0
            }
            TileContent::Text { text, align } => {
                let halign = match align {
                    TextAlign::Left => Align2::LEFT_TOP,
                    TextAlign::Center => Align2::CENTER_TOP,
                    TextAlign::Right => Align2::RIGHT_TOP,
                };
                
                let text_galley = ui.painter().layout(
                    text,
                    FontId::new(14.0, FontFamily::Name("Inter Variable")),
                    self.on_surface(),
                );
                
                let pos = match align {
                    TextAlign::Left => Pos2::new(rect.min.x + 16.0, rect.min.y + 16.0),
                    TextAlign::Center => Pos2::new(rect.center().x, rect.min.y + 16.0 + text_galley.size().y / 2.0),
                    TextAlign::Right => Pos2::new(rect.max.x - 16.0, rect.min.y + 16.0),
                };
                
                ui.painter().galley(pos, text_galley, self.on_surface());
                
                text_galley.size().y + 16.0
            }
            TileContent::Image { url, fit } => {
                // Placeholder for image
                let text = format!("[Image: {}]", url);
                let text_galley = ui.painter().layout(
                    text,
                    FontId::new(14.0, FontFamily::Name("Inter Variable")),
                    self.outline(),
                );
                
                let pos = Pos2::new(rect.center().x, rect.min.y + 16.0 + text_galley.size().y / 2.0);
                ui.painter().galley(pos, text_galley, self.outline());
                
                text_galley.size().y + 16.0
            }
        }
    }
    
    // Theme color helpers
    fn primary(&self) -> Color32 {
        Color32::from_hex("#0066CC").unwrap_or(Color32::BLUE)
    }
    
    fn surface(&self) -> Color32 {
        Color32::from_hex("#121212").unwrap_or(Color32::DARK_GRAY)
    }
    
    fn on_surface(&self) -> Color32 {
        Color32::from_hex("#E6E1E5").unwrap_or(Color32::WHITE)
    }
    
    fn outline(&self) -> Color32 {
        Color32::from_hex("#938F99").unwrap_or(Color32::GRAY)
    }
    
    fn error(&self) -> Color32 {
        Color32::from_hex("#F2B8B5").unwrap_or(Color32::RED)
    }
}