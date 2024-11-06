use iced::border::Border;
use iced::theme::palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success};
use iced::theme::Palette;
use iced::widget::container;
use iced::widget::container::Style;
use iced_core::{Color, Element, Theme};
use iced_runtime::{Program, Task};
use iced_wgpu::Renderer;

const PALETTE: Palette = Palette {
    background: Color::from_rgb(0.2, 0.2, 0.2),
    text: Color::from_rgb(0.95, 0.95, 0.95),
    primary: Color::from_rgb(0.25, 0.25, 0.25),
    success: Color::from_rgb(58.0 / 255.0, 170.0 / 255.0, 153.0 / 255.0),
    danger: Color::from_rgb(204.0 / 255.0, 68.0 / 255.0, 153.0 / 255.0),
};

#[allow(dead_code)]
fn generate(_palette: Palette) -> Extended {
    Extended {
        background: Background {
            base: Pair::new(PALETTE.background, PALETTE.text),
            weak: Pair::new(PALETTE.background, PALETTE.text),
            strong: Pair::new(PALETTE.background, PALETTE.text),
        },
        primary: Primary {
            base: Pair::new(PALETTE.primary, PALETTE.text),
            weak: Pair::new(PALETTE.primary, PALETTE.text),
            strong: Pair::new(PALETTE.primary, PALETTE.text),
        },
        secondary: Secondary::generate(PALETTE.background, PALETTE.text),
        success: Success::generate(PALETTE.success, PALETTE.background, PALETTE.text),
        danger: Danger::generate(PALETTE.danger, PALETTE.background, PALETTE.text),
        is_dark: true,
    }
}

pub fn theme() -> iced::theme::Theme {
    //iced::theme::Theme::custom_with_fn(String::from("Naev"), PALETTE, generate )
    iced::theme::Theme::custom(String::from("Naev"), PALETTE)
}

#[allow(dead_code)]
pub fn window(theme: &Theme) -> Style {
    let palext = theme.extended_palette();
    let palette = theme.palette();

    Style {
        background: Some(palette.background.into()),
        border: Border {
            width: 1.0,
            radius: 10.0.into(),
            color: palext.background.strong.color,
        },
        ..Style::default()
    }
}

pub struct Toolkit {}

#[derive(Debug, Clone)]
pub enum Message {
    None,
}

impl Toolkit {
    pub fn new() -> Toolkit {
        Toolkit {}
    }
}

impl Program for Toolkit {
    type Theme = Theme;
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, _message: Message) -> Task<Message> {
        Task::none()
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        container("").into()
    }
}
