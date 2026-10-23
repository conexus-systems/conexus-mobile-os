//! Home Screen for egui PoC

use eframe::egui::{self, Color32, FontId, FontFamily, FontId, TextStyle, Visuals, Style, Vec2, Rect, Pos2};
use std::sync::Arc;

use crate::models::{Tile, TileSize, TileContent, ThemeMode, DesignTokens, TextAlign, ImageFit};
use crate::theme::{ConexusTheme, ConexusThemeMode};
use crate::tile::TileWidget;
use crate::pivot_tabs::PivotTabs;
use crate::platform::Platform;

pub struct HomeScreenApp {
    theme: ConexusTheme,
    tiles: Vec<Tile>,
    current_pivot: usize,
    pivot_titles: Vec<String>,
    window_size: egui::Vec2,
    platform: Box<dyn Platform>,
    tile_widgets: Vec<TileWidget>,
}

impl HomeScreenApp {
    pub fn new(tokens: Arc<DesignTokens>, platform: Box<dyn Platform>) -> Self {
        let theme = ConexusTheme::new(ThemeMode::Dark);
        let tiles = create_sample_tiles();
        
        let tile_widgets = tiles.iter().map(|t| TileWidget::new(t.clone())).collect();

        Self {
            theme: ConexusTheme {
                mode: ConexusThemeMode::Dark,
                tokens: tokens.as_ref(),
            },
            tiles,
            current_pivot: 1,
            pivot_titles: vec!["Eventos".to_string(), "Apps".to_string(), "Configurações".to_string()],
            window_size: egui::Vec2::new(360.0, 640.0),
            platform,
            tile_widgets: tiles.iter().map(|t| TileWidget::new(t.clone())).collect(),
        }
    }

    pub fn update(&mut self, ctx: &egui::Context) {
        // Handle theme toggle via keyboard shortcut
        if ctx.input(|i| i.key_pressed(egui::Key::T) && i.modifiers.ctrl) {
            self.theme.toggle();
        }

        // Handle escape key
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            // Close any open modals/menus
        }
    }

    pub fn ui(&mut self, ctx: &egui::Context) {
        // Apply theme visuals
        let visuals = self.theme.visuals();
        ctx.set_visuals(self.theme.visuals());

        // Central panel
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.with_layout(egui::Layout::top_down(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                
                // App Bar
                self.app_bar(ui);
                
                ui.add_space(8.0);
                
                // Pivot Tabs
                self.pivot_tabs(ui);
                
                ui.add_space(8.0);
                
                // Content Area
                match self.current_pivot {
                    0 => self.eventos_view(ui),
                    1 => self.apps_grid_view(ui),
                    2 => self.settings_view(ui),
                    _ => self.apps_grid_view(ui),
                }
                
                ui.add_space(8.0);
                
                // Status Bar
                self.status_bar(ui);
            });
        });
    }

    fn app_bar(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.add_space(16.0);
            
            // Title
            ui.label(egui::RichText::new("Conexus")
                .size(self.theme.tokens.typography.scale.headline_medium.size as f32)
                .color(self.theme.on_surface())
                .font(egui::FontId::new(
                    self.theme.tokens.typography.scale.headline_medium.size as f32,
                    FontFamily::Name("Inter Variable")
                ))
                .strong()
            );
            
            ui.add_space(16.0);
            
            // Spacer
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                // Theme toggle
                if ui.button(egui::RichText::new(match self.theme.mode {
                    ConexusThemeMode::Dark => "☀️",
                    ConexusThemeMode::Light => "🌙",
                    ConexusThemeMode::HighContrast => "⚡",
                }).size(20.0)).on_hover_text("Alternar tema").clicked() {
                    self.theme.toggle();
                }
                
                ui.add_space(8.0);
                
                // Search button
                if ui.button("🔍").on_hover_text("Buscar").clicked() {
                    // TODO: Open search modal
                }
            });
        });
        
        // Divider
        ui.separator();
    }

    fn pivot_tabs(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            for (i, title) in self.pivot_titles.iter().enumerate() {
                let is_active = self.current_pivot == i;
                
                let button = egui::Button::new(
                    egui::RichText::new(title)
                        .size(14.0)
                        .color(if self.current_pivot == i { self.primary() } else { self.outline() })
                        .strong()
                )
                .fill(Color32::TRANSPARENT)
                .stroke(egui::Stroke::NONE)
                .corner_radius(0.0);
                
                let response = ui.add(button);
                
                if response.clicked() {
                    self.current_pivot = i;
                }
                
                // Active indicator
                if is_active {
                    let rect = response.rect;
                    ui.painter().rect_stroke(
                        Rect::from_min_max(
                            Pos2::new(rect.min.x, rect.max.y - 3.0),
                            Pos2::new(rect.max.x, rect.max.y)
                        ),
                        0.0,
                        egui::Stroke::new(3.0, self.primary()),
                    );
                }
            });
        });
        
        ui.separator();
    }

    fn content_area(&mut self, ui: &mut egui::Ui) {
        match self.current_pivot {
            0 => self.eventos_view(ui),
            1 => self.apps_grid_view(ui),
            2 => self.settings_view(ui),
            _ => self.apps_grid_view(ui),
        }
    }

    fn eventos_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(32.0);
            ui.label(egui::RichText::new("Eventos")
                .size(24.0)
                .color(self.on_surface())
                .strong()
            );
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Próximos eventos aparecerão aqui")
                .size(16.0)
                .color(self.outline())
            );
        });
    }

    fn apps_grid_view(&mut self, ui: &mut egui::Ui) {
        let responsive_cols = if self.window_size.x < 600.0 {
            4
        } else if self.window_size.x < 900.0 {
            5
        } else if self.window_size.x < 1200.0 {
            6
        } else {
            8
        };

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(egui::RichText::new("Aplicativos")
                    .size(18.0)
                    .color(self.on_surface())
                    .strong()
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    ui.label(egui::RichText::new(format!("{} apps", self.tiles.len()))
                        .size(14.0)
                        .color(self.outline())
                    );
                });
            });
            
            ui.add_space(8.0);
            
            // Grid using Grid layout
            egui::Grid::new("apps_grid")
                .num_columns(self.responsive_cols as usize)
                .spacing([16.0, 16.0])
                .striped(false)
                .show(ui, |ui| {
                    for (i, tile) in self.tiles.iter().enumerate() {
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
                        
                        // Render tile
                        let tile_widget = TileWidget::new(tile.clone());
                        let response = tile_widget.ui(ui, &self.theme);
                        
                        if response.clicked() {
                            println!("Tile clicked: {}", tile.id);
                        }
                        
                        // Handle grid spanning (egui Grid doesn't support col_span/row_span directly)
                        // This is a simplified implementation
                        if i % self.responsive_cols as usize == self.responsive_cols as usize - 1 {
                            ui.end_row();
                        }
                    }
                });
        });
    }

    fn settings_view(&mut self, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.add_space(32.0);
            ui.label(egui::RichText::new("Configurações")
                .size(24.0)
                .color(self.on_surface())
                .strong()
            );
            ui.add_space(8.0);
            ui.label(egui::RichText::new("Configurações do Conexus aparecerão aqui")
                .size(16.0)
                .color(self.outline())
            );
            
            ui.add_space(16.0);
            
            if ui.button(egui::RichText::new(format!("Alternar Tema ({})", self.theme.current_mode().as_str()))
                .size(16.0)
                .color(Color32::WHITE)
            ).fill(self.primary()).clicked() {
                self.theme.toggle();
            }
        });
    }

    fn status_bar(&mut self, ui: &mut egui::Ui) {
        ui.add_space(8.0);
        ui.separator();
        ui.horizontal(|ui| {
            ui.add_space(8.0);
            ui.label(egui::RichText::new("🔋 85%").size(12.0).color(self.on_surface()));
            ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::LeftToRight), |ui| {
                ui.label(egui::RichText::new("14:30").size(12.0).color(self.on_surface()).strong());
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                ui.add_space(8.0);
                ui.label(egui::RichText::new("📶 📡").size(12.0).color(self.on_surface()));
            });
        });
    }

    // Theme color getters
    fn primary(&self) -> Color32 {
        self.theme.primary()
    }

    fn surface(&self) -> Color32 {
        self.theme.surface()
    }

    fn on_surface(&self) -> Color32 {
        self.theme.on_surface()
    }

    fn outline(&self) -> Color32 {
        self.theme.outline()
    }

    fn error(&self) -> Color32 {
        self.theme.error()
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