//! Tile Widget for Iced PoC

use iced::{
    Alignment, Background, Border, Color, Container, Element, Length, Padding, Row, Column,
    Text, TextAlignment, mouse, Event, Touch,
    widget::{button, container, text},
    border::Radius,
    widget::button,
    alignment::{Horizontal, Vertical},
    Border as IcedBorder,
};

use crate::models::{Tile, TileSize, TileContent, TextAlign};
use crate::theme::ConexusTheme;

#[derive(Debug, Clone)]
pub struct TileWidget {
    tile: Tile,
    theme: ConexusTheme,
    on_click: Option<Box<dyn Fn() + Send + Sync>>,
    is_flipping: bool,
    flip_progress: f32,
}

impl TileWidget {
    pub fn new(tile: Tile, theme: ConexusTheme) -> Self {
        Self {
            tile,
            theme,
            on_click: None,
            is_flipping: false,
            flip_progress: 0.0,
        }
    }

    pub fn on_click<F>(mut self, f: F) -> Self
    where
        F: Fn() + Send + Sync + 'static,
    {
        self.on_click = Some(Box::new(f));
        self
    }

    pub fn view(&self) -> Element<'_, crate::home_screen::Message> {
        let theme = &self.theme;
        let tile = &self.tile;
        
        let size_class = match tile.size {
            TileSize::Small => "small",
            TileSize::Medium => "medium",
            TileSize::Wide => "wide",
            TileSize::Large => "large",
        };

        let content = Column::new()
            .spacing(8)
            .padding(16)
            .align_x(Alignment::Center)
            .push(self.render_front_content(theme))
            .push(
                Text::new(&tile.title)
                    .size(16)
                    .color(theme.on_surface())
                    .font(self.theme.font_family())
                    .align_x(Horizontal::Center)
            )
            .push(self.render_badge(theme));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(theme.surface())),
                border: IcedBorder {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(theme.corner_radius("medium")),
                },
                ..Default::default()
            })
            .into()
    }

    fn render_front_content(&self, theme: &ConexusTheme) -> Element<'_, crate::home_screen::Message> {
        let tile = &self.tile;
        
        match &tile.front_content {
            TileContent::Icon { name, color } => {
                Container::new(
                    Text::new(&format!("[{}]", name)) // Placeholder for icon
                        .size(32)
                        .color(Color::from_str(color).unwrap_or(theme.primary()))
                        .font(self.theme.font_family())
                )
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
            }
            TileContent::Text { text, align } => {
                let alignment = match align {
                    TextAlign::Left => Horizontal::Left,
                    TextAlign::Center => Horizontal::Center,
                    TextAlign::Right => Horizontal::Right,
                };
                
                Container::new(
                    Text::new(text)
                        .size(14)
                        .color(self.theme.on_surface())
                        .font(self.theme.font_family())
                        .align_x(align)
                        .wrapping(iced::widget::text::Wrapping::Word)
                )
                .center_x(Length::Fill)
                .into()
            }
            TileContent::Image { url, fit } => {
                // Placeholder for image
                Container::new(
                    Text::new(&format!("[Image: {}]", url))
                        .size(14)
                        .color(self.theme.outline())
                        .font(self.theme.font_family())
                )
                .center_x(Length::Fill)
                .center_y(Length::Fill)
                .into()
            }
        }
    }

    fn render_badge(&self, theme: &ConexusTheme) -> Element<'_, crate::home_screen::Message> {
        if let Some(badge) = self.tile.badge {
            let badge_text = if badge > 99 { "99+".to_string() } else { badge.to_string() };
            
            Container::new(
                Text::new(badge_text)
                    .size(12)
                    .color(Color::WHITE)
                    .font(self.theme.font_family())
            )
            .padding(Padding::new(2.0, 6.0))
            .style(move |_| container::Style {
                background: Some(Background::Color(self.theme.error())),
                border: Border {
                    color: Color::TRANSPARENT,
                    width: 0.0,
                    radius: Radius::new(12.0),
                },
                ..Default::default()
            })
            .into()
        } else {
            vertical_space(Length::Fixed(0.0)).into()
        }
    }
}

// Helper to parse hex color
fn color_from_hex(hex: &str) -> Color {
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