//! Home Screen for Iced PoC

use iced::{
    Alignment, Background, Border, Color, Container, Element, Length, Padding, Row, Column,
    Scrollable, Text, TextAlignment, Padding as IcedPadding,
    widget::{button, checkbox, container, horizontal_space, vertical_space, text},
    alignment::{Horizontal, Vertical},
    border::Radius,
    widget::{grid, scrollable},
    keyboard, mouse, touch,
    event, Event, Subscription, Task,
    keyboard::key::Named,
    widget::grid::{Grid, Item},
};
use std::sync::Arc;

use crate::models::{Tile, TileSize, TileContent, ThemeMode, DesignTokens};
use crate::theme::{ConexusTheme, ConexusThemeMode};
use crate::tile::TileWidget;
use crate::pivot_tabs::PivotTabs;
use crate::platform::Platform;

#[derive(Debug, Clone)]
pub enum Message {
    ThemeToggle,
    PivotChanged(usize),
    TileClicked(usize),
    WindowResized(iced::Size),
    ThemeModeChanged(ThemeMode),
}

pub struct HomeScreen {
    theme: ConexusTheme,
    tiles: Vec<Tile>,
    current_pivot: usize,
    pivot_titles: Vec<String>,
    window_size: iced::Size,
    platform: Box<dyn Platform>,
}

impl HomeScreen {
    pub fn new(tokens: Arc<DesignTokens>) -> (Self, Task<Message>) {
        let theme = ConexusTheme::new(ThemeMode::Dark);
        let tiles = create_sample_tiles();
        
        let platform: Box<dyn Platform> = if cfg!(target_os = "android") {
            Box::new(crate::platform::AndroidPlatform::new())
        } else {
            Box::new(crate::platform::LinuxPlatform::new())
        };

        let mut app = Self {
            theme: ConexusTheme {
                mode: ConexusThemeMode::Dark,
                tokens: tokens.as_ref(),
            },
            tiles,
            current_pivot: 1,
            pivot_titles: vec!["Eventos".to_string(), "Apps".to_string(), "Configurações".to_string()],
            window_size: iced::Size::new(360.0, 640.0),
            platform,
        };

        let init_task = app.platform.init();
        
        (app, init_task)
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::ThemeToggle => {
                self.theme.toggle();
                Task::none()
            }
            Message::PivotChanged(index) => {
                self.current_pivot = index;
                Task::none()
            }
            Message::TileClicked(index) => {
                println!("Tile clicked: {}", index);
                Task::none()
            }
            Message::WindowResized(size) => {
                self.window_size = size;
                Task::none()
            }
            Message::ThemeModeChanged(mode) => {
                self.theme.mode = mode.into();
                Task::none()
            }
        }
    }

    pub fn view(&self) -> Element<Message> {
        let theme = &self.theme;
        
        // Main container
        let content = Column::new()
            .width(Length::Fill)
            .height(Length::Fill)
            .spacing(0)
            .push(self.app_bar(theme))
            .push(self.pivot_tabs(theme))
            .push(self.content_area(theme))
            .push(self.status_bar(theme));

        Container::new(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(theme.surface())),
                ..Default::default()
            })
            .into()
    }

    fn app_bar(&self, theme: &ConexusTheme) -> Element<Message> {
        Container::new(
            Row::new()
                .spacing(16)
                .padding(16)
                .align_y(Alignment::Center)
                .push(
                    Text::new("Conexus")
                        .size(theme.font_size_headline() as u16)
                        .color(theme.on_surface())
                        .font(theme.font_family())
                )
                .push(horizontal_space(Length::Fill))
                .push(
                    button(
                        Text::new("🔍")
                            .size(20)
                            .color(theme.on_surface())
                    )
                    .on_press(Message::TileClicked(999)) // placeholder for search
                    .style(button::Style {
                        background: Some(Background::Color(Color::TRANSPARENT)),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: Radius::new(20.0),
                        },
                        ..Default::default()
                    })
                    .padding(8)
                )
                .push(
                    button(
                        Text::new(match theme.mode {
                            ConexusThemeMode::Dark => "☀️",
                            ConexusThemeMode::Light => "🌙",
                            ConexusThemeMode::HighContrast => "⚡",
                        })
                            .size(20)
                            .color(theme.on_surface())
                    )
                    .on_press(Message::ThemeToggle)
                    .style(button::Style {
                        background: Some(Background::Color(Color::TRANSPARENT)),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: Radius::new(20.0),
                        },
                        ..Default::default()
                    })
                    .padding(8)
                )
        )
        .width(Length::Fill)
        .height(Length::Fixed(56.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(theme.surface())),
            border: Border {
                color: theme.outline(),
                width: 1.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        })
        .into()
    }

    fn pivot_tabs(&self, theme: &ConexusTheme) -> Element<Message> {
        let tabs = self.pivot_titles.iter().enumerate().map(|(i, title)| {
            let is_active = self.current_pivot == i;
            let text_color = if is_active { theme.primary() } else { theme.outline() };
            
            Container::new(
                Text::new(title)
                    .size(14)
                    .color(text_color)
                    .font(theme.font_family())
            )
            .width(Length::Fill)
            .height(Length::Fixed(48.0))
            .center_x(Length::Fill)
            .center_y(Length::Fill)
            .style(move |_| container::Style {
                background: Some(Background::Color(Color::TRANSPARENT)),
                border: Border {
                    color: if is_active { theme.primary() } else { Color::TRANSPARENT },
                    width: 0.0,
                    radius: Radius::new(0.0),
                },
                ..Default::default()
            })
            .into()
        }).collect::<Vec<_>>();

        Container::new(
            Row::new()
                .spacing(0)
                .push(tabs)
        )
        .width(Length::Fill)
        .height(Length::Fixed(48.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(theme.surface())),
            border: Border {
                color: theme.outline(),
                width: 1.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        })
        .into()
    }

    fn content_area(&self, theme: &ConexusTheme) -> Element<Message> {
        match self.current_pivot {
            0 => self.eventos_view(theme),
            1 => self.apps_grid_view(theme),
            2 => self.settings_view(theme),
            _ => self.apps_grid_view(theme),
        }
    }

    fn eventos_view(&self, theme: &ConexusTheme) -> Element<Message> {
        Container::new(
            Column::new()
                .spacing(16)
                .align_x(Alignment::Center)
                .push(
                    Text::new("Eventos")
                        .size(24)
                        .color(theme.on_surface())
                        .font(theme.font_family())
                )
                .push(
                    Text::new("Próximos eventos aparecerão aqui")
                        .size(16)
                        .color(theme.outline())
                        .font(theme.font_family())
                )
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(theme.surface())),
            ..Default::default()
        })
        .into()
    }

    fn apps_grid_view(&self, theme: &ConexusTheme) -> Element<Message> {
        let responsive_cols = if self.window_size.width < 600.0 {
            4
        } else if self.window_size.width < 900.0 {
            5
        } else if self.window_size.width < 1200.0 {
            6
        } else {
            8
        };

        let grid = Grid::new()
            .spacing(16)
            .push(self.tiles.iter().enumerate().map(|(i, tile)| {
                let col_span = match tile.size {
                    TileSize::Small => 1,
                    TileSize::Medium => 2,
                    TileSize::Wide => 4,
                    TileSize::Large => 4,
                };
                let row_span = match tile.size {
                    TileSize::Small => 1,
                    TileSize::Medium => 2,
                    TileSize::Wide => 2,
                    TileSize::Large => 4,
                };

                Item::new(
                    TileWidget::new(tile.clone(), theme.clone())
                        .on_click(move || Message::TileClicked(i))
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
                .col_span(col_span)
                .row_span(row_span)
            }).collect::<Vec<_>>()
            .width(Length::Fill)
            .height(Length::Fill);

        Container::new(
            Column::new()
                .spacing(16)
                .padding(16)
                .push(
                    Row::new()
                        .spacing(16)
                        .align_y(Alignment::Center)
                        .push(
                            Text::new("Aplicativos")
                                .size(18)
                                .color(theme.on_surface())
                                .font(theme.font_family())
                        )
                        .push(horizontal_space(Length::Fill))
                        .push(
                            Text::new(format!("{} apps", self.tiles.len()))
                                .size(14)
                                .color(theme.outline())
                                .font(theme.font_family())
                        )
                )
                .push(
                    Scrollable::new(grid)
                        .width(Length::Fill)
                        .height(Length::Fill)
                )
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(theme.surface())),
            ..Default::default()
        })
        .into()
    }

    fn settings_view(&self, theme: &ConexusTheme) -> Element<Message> {
        Container::new(
            Column::new()
                .spacing(16)
                .align_x(Alignment::Center)
                .push(
                    Text::new("Configurações")
                        .size(24)
                        .color(theme.on_surface())
                        .font(theme.font_family())
                )
                .push(
                    Text::new("Configurações do Conexus aparecerão aqui")
                        .size(16)
                        .color(theme.outline())
                        .font(theme.font_family())
                )
                .push(
                    button(
                        Text::new(format!("Alternar Tema ({})", theme.mode.as_str()))
                            .size(16)
                            .color(Color::WHITE)
                            .font(theme.font_family())
                    )
                    .on_press(Message::ThemeToggle)
                    .padding(12)
                    .style(button::Style {
                        background: Some(Background::Color(theme.primary())),
                        border: Border {
                            color: Color::TRANSPARENT,
                            width: 0.0,
                            radius: Radius::new(8.0),
                        },
                        ..Default::default()
                    })
                )
        )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_x(Length::Fill)
        .center_y(Length::Fill)
        .style(move |_| container::Style {
            background: Some(Background::Color(theme.surface())),
            ..Default::default()
        })
        .into()
    }

    fn status_bar(&self, theme: &ConexusTheme) -> Element<Message> {
        Container::new(
            Row::new()
                .spacing(16)
                .padding(IcedPadding::new(0.0, 8.0))
                .align_y(Alignment::Center)
                .push(
                    Text::new("🔋 85%")
                        .size(12)
                        .color(theme.on_surface())
                        .font(theme.font_family())
                )
                .push(
                    Text::new("14:30")
                        .size(12)
                        .color(theme.on_surface())
                        .font(theme.font_family())
                )
                .push(
                    Text::new("📶 📡")
                        .size(12)
                        .color(theme.on_surface())
                        .font(theme.font_family())
                )
        )
        .width(Length::Fill)
        .height(Length::Fixed(24.0))
        .style(move |_| container::Style {
            background: Some(Background::Color(theme.surface())),
            border: Border {
                color: theme.outline(),
                width: 1.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        })
        .into()
    }

    pub fn subscription(&self) -> Subscription<Message> {
        Subscription::batch(vec![
            iced::window::resize_events().map(Message::WindowResized),
            iced::keyboard::on_key_press(|key, _modifiers| {
                match key {
                    Named::Escape => Some(Message::TileClicked(999)),
                    _ => None,
                }
            }),
        ])
    }

    pub fn theme(&self) -> IcedTheme {
        self.theme.to_iced_theme()
    }

    pub fn scale_factor(&self) -> f64 {
        1.0
    }
}

fn create_sample_tiles() -> Vec<Tile> {
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