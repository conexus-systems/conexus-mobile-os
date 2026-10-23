//! Pivot Tabs for egui PoC

use egui::{Color32, RichText, Ui, Sense, Response, Rect, Pos2, Stroke, Rounding, Painter, Align2, FontId, FontFamily, Layout, Align, Widget};
use crate::theme::ConexusTheme;
use crate::home_screen::Message;

pub struct PivotTabs {
    titles: Vec<String>,
    current: usize,
}

impl PivotTabs {
    pub fn new(titles: Vec<String>, current: usize) -> Self {
        Self {
            titles,
            current,
        }
    }

    pub fn ui(&mut self, ui: &mut Ui, theme: &crate::theme::ConexusTheme) -> Vec<Response> {
        let responses = self.titles.iter().enumerate().map(|(i, title)| {
            let is_active = self.current == i;
            
            let text_color = if is_active { theme.primary() } else { theme.outline() };
            let border_color = if is_active { theme.primary() } else { Color32::TRANSPARENT };
            
            let button = egui::Button::new(
                RichText::new(title)
                    .size(14.0)
                    .color(text_color)
                    .strong()
            )
            .fill(Color32::TRANSPARENT)
            .stroke(egui::Stroke::NONE)
            .corner_radius(0.0);
            
            let response = ui.add_sized([ui.available_width() / self.titles.len() as f32, 48.0], button);
            
            if response.clicked() {
                // Message sent via parent
            }
            
            // Active indicator
            if is_active {
                let rect = response.rect;
                ui.painter().rect_stroke(
                    Rect::from_min_max(
                        Pos2::new(rect.min.x, rect.max.y - 3.0),
                        Pos2::new(rect.max.x, rect.max.y)
                    ),
                    Rounding::ZERO,
                    Stroke::new(3.0, theme.primary()),
                );
            }
            
            response
        }).collect()
    }
}