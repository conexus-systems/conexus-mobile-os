//! Pivot Tabs for Iced PoC

use iced::{
    Alignment, Background, Border, Color, Container, Element, Length, Padding, Row, Column,
    Text, button, container, horizontal_space,
    border::Radius,
    widget::button,
    alignment::Horizontal,
};

use crate::theme::ConexusTheme;
use crate::home_screen::Message;

pub struct PivotTabs {
    titles: Vec<String>,
    current: usize,
    theme: ConexusTheme,
}

impl PivotTabs {
    pub fn new(titles: Vec<String>, current: usize, theme: ConexusTheme) -> Self {
        Self {
            titles,
            current,
            theme,
        }
    }

    pub fn view(&self) -> Element<'_, Message> {
        let tabs = self.titles.iter().enumerate().map(|(i, title)| {
            let is_active = self.current == i;
            let text_color = if is_active { self.theme.primary() } else { self.theme.outline() };
            let border_color = if is_active { self.theme.primary() } else { Color::TRANSPARENT };
            
            Container::new(
                button(
                    Container::new(
                        Text::new(title)
                            .size(14)
                            .color(text_color)
                            .font(self.theme.font_family())
                    )
                    .width(Length::Fill)
                    .height(Length::Fill)
                    .center_x(Length::Fill)
                    .center_y(Length::Fill)
                )
                .on_press(Message::PivotChanged(i))
                .style(button::Style {
                    background: Some(Background::Color(Color::TRANSPARENT)),
                    border: Border {
                        color: border_color,
                        width: if is_active { 3.0 } else { 0.0 },
                        radius: Radius::new(0.0),
                    },
                    ..Default::default()
                })
                .padding(0)
            )
            .width(Length::Fill)
            .height(Length::Fixed(48.0))
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
            background: Some(Background::Color(self.theme.surface())),
            border: Border {
                color: self.theme.outline(),
                width: 1.0,
                radius: Radius::new(0.0),
            },
            ..Default::default()
        })
        .into()
    }
}