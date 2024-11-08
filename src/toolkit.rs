use crate::toolkit_lua::{MessageLua, ToolkitProgramLua};
use iced::border::Border;
use iced::theme::palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success};
use iced::theme::Palette;
use iced::widget::container::Style;
use iced_core::{Color, Theme};
use iced_wgpu::{wgpu, Renderer};

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

pub enum ToolkitProgram {
    Lua(ToolkitProgramLua),
    MenuMain(crate::menu_main::MenuMain),
}

#[derive(Debug, Clone)]
pub enum Message {
    Lua(MessageLua),
    MenuMain(crate::menu_main::Message),
}

impl iced_runtime::Program for ToolkitProgram {
    type Theme = Theme;
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> iced_runtime::Task<Message> {
        match self {
            ToolkitProgram::Lua(state) => state.update(message),
            ToolkitProgram::MenuMain(state) => state.update(message),
            //_ => iced_runtime::Task::none(),
        }
    }

    fn view(&self) -> iced_core::Element<Message, Theme, Renderer> {
        match self {
            ToolkitProgram::Lua(state) => state.view(),
            ToolkitProgram::MenuMain(state) => state.view(),
            //_ => iced_widget::text("").into(),
        }
    }
}

pub struct Toolkit<'a> {
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    renderer: iced_wgpu::Renderer,
    viewport: iced_wgpu::graphics::Viewport,
    debug: iced_runtime::Debug,
    cursor_position: iced_core::mouse::Cursor,
    //pub state: Vec<Box<iced_runtime::program::State<dyn iced_runtime::Program>>>,
    pub state: Vec<iced_runtime::program::State<ToolkitProgram>>,
}

impl<'a> Toolkit<'a> {
    pub fn new(
        engine: &mut iced_wgpu::Engine,
        device: &'a wgpu::Device,
        queue: &'a wgpu::Queue,
        scale_factor: f64,
        width: u32,
        height: u32,
    ) -> Toolkit<'a> {
        let mut renderer = iced_wgpu::Renderer::new(
            device,
            engine,
            iced::Font::default(),
            iced::Pixels::from(16),
        );
        let viewport = iced_wgpu::graphics::Viewport::with_physical_size(
            iced::Size::new(width, height),
            scale_factor,
        );
        let mut debug = iced_runtime::Debug::new();
        let mut state = iced_runtime::program::State::new(
            ToolkitProgramLua::new().unwrap_or_else(|err| {
                panic!("{}", err);
            }),
            viewport.logical_size(),
            &mut renderer,
            &mut debug,
        );
        state.queue_event(iced::Event::Window(iced::window::Event::RedrawRequested(
            std::time::Instant::now(),
        )));
        Toolkit {
            device,
            queue,
            renderer,
            viewport,
            debug,
            cursor_position: iced_core::mouse::Cursor::Unavailable,
            state: Vec::new(),
        }
    }

    pub fn update_cursor_position(&mut self, x: f32, y: f32) -> () {
        let s = 1.0 / self.viewport.scale_factor() as f32;
        self.cursor_position =
            iced_core::mouse::Cursor::Available(iced_core::Point::new(x * s, y * s));
    }

    pub fn queue_event(&mut self, event: iced_core::Event) -> () {
        match self.state.last_mut() {
            Some(state) => {
                state.queue_event(event);
            }
            _ => {
                return;
            }
        };
    }

    pub fn update(&mut self) -> () {
        let state = match self.state.last_mut() {
            Some(state) => state,
            _ => {
                return;
            }
        };

        if state.is_queue_empty() {
            return;
        }
        let theme = crate::toolkit::theme();

        // We update iced
        let _ = state.update(
            self.viewport.logical_size(),
            self.cursor_position,
            &mut self.renderer,
            &theme,
            &iced_core::renderer::Style::default(),
            &mut iced_core::clipboard::Null,
            &mut self.debug,
        );

        // Handle events from the app
        //let program = state.program();
        // match program.state {
        //     menu_main::Message::ExitGame => {
        //         break 'running;
        //     }
        //     _ => (),
        // };

        // and request a redraw
        //window.request_redraw();
    }

    pub fn draw(
        &mut self,
        engine: &mut iced_wgpu::Engine,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::SurfaceTexture,
    ) -> () {
        self.renderer.present(
            engine,
            self.device,
            self.queue,
            encoder,
            None,
            frame.texture.format(),
            view,
            &self.viewport,
            &self.debug.overlay(),
        );
    }

    //pub fn open ( &mut self, program: impl iced_runtime::Program<Renderer = iced_wgpu::Renderer> + 'static ) -> () {
    pub fn open(&mut self, program: ToolkitProgram) -> &ToolkitProgram {
        let state = iced_runtime::program::State::new(
            program,
            self.viewport.logical_size(),
            &mut self.renderer,
            &mut self.debug,
        );
        self.state.push(state);
        match self.state.last() {
            Some(state) => state.program(),
            None => unreachable!(),
        }
    }
}
