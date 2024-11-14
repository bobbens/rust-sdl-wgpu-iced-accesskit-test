use crate::toolkit_lua::{MessageLua, ToolkitWindowLua};
use iced::border::Border;
use iced::theme::palette::{Background, Danger, Extended, Pair, Primary, Secondary, Success};
use iced::theme::Palette;
use iced::widget::container::Style;
use iced_core::{Color, Element, Theme};
use iced_runtime::Task;
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

pub trait Window {
    fn update(&mut self, message: Message) -> Message;
    fn view(&self) -> Element<'_, Message, Theme, Renderer>;
}

#[derive(Debug, Clone)]
pub enum MessageDialogue {
    Accept,
    Cancel,
    ContentChanged(String),
}

#[derive(PartialEq, Eq)]
pub struct DlgInput {
    msg: String,
    input: String,
}
impl DlgInput {
    pub fn new(msg: String) -> DlgInput {
        DlgInput {
            msg,
            input: String::new(),
        }
    }
}
impl Window for DlgInput {
    fn update(&mut self, message: Message) -> Message {
        if let Message::Dialogue(m) = message {
            match m {
                MessageDialogue::Accept => Message::None, //Message::OpenLua,
                MessageDialogue::Cancel => Message::CloseWindow,
                MessageDialogue::ContentChanged(content) => {
                    self.input = content;
                    Message::None
                } //_ => Message::None,
            }
        } else {
            Message::None
        }
    }

    fn view(&self) -> Element<Message, Theme, Renderer> {
        use iced::{color, Center, Fill};
        use iced_widget::{button, column, container, row, text, text_input};

        container(
            container(
                column![
                    text(self.msg.as_str()).color(color!(0xffffff)),
                    text_input("", &self.input).on_input(|str| {
                        Message::Dialogue(MessageDialogue::ContentChanged(str))
                    }),
                    row![
                        button("OK").on_press(Message::Dialogue(MessageDialogue::Accept)),
                        button("Cancel").on_press(Message::Dialogue(MessageDialogue::Cancel)),
                    ]
                    .spacing(20),
                ]
                .spacing(10)
                .padding(20)
                .align_x(Center),
            )
            .style(crate::toolkit::window)
            .align_x(Center)
            .width(400),
        )
        .style(container::transparent)
        .center(Fill)
        .into()
    }
}

pub enum ToolkitWindow {
    Lua(ToolkitWindowLua),
    MenuMain(crate::menu_main::MenuMain),
    DlgInput(DlgInput),
}

#[derive(Debug, Clone)]
pub enum Message {
    None,
    CloseWindow,
    OpenMenuMain,
    OpenLua(ToolkitWindowLua),
    OpenDialogueInput(String),
    Lua(MessageLua),
    MenuMain(crate::menu_main::Message),
    Dialogue(MessageDialogue),
}

impl Window for ToolkitWindow {
    fn update(&mut self, message: Message) -> Message {
        match self {
            ToolkitWindow::Lua(state) => state.update(message),
            ToolkitWindow::MenuMain(state) => state.update(message),
            ToolkitWindow::DlgInput(state) => state.update(message),
            //_ => iced_runtime::Task::none(),
        }
    }

    fn view(&self) -> iced_core::Element<Message, Theme, Renderer> {
        match self {
            ToolkitWindow::Lua(state) => state.view(),
            ToolkitWindow::MenuMain(state) => state.view(),
            ToolkitWindow::DlgInput(state) => state.view(),
            //_ => iced_widget::text("").into(),
        }
    }
}

pub struct ToolkitProgram {
    pub open: bool,
    pub windows: Vec<ToolkitWindow>,
}

impl ToolkitProgram {
    pub fn new() -> ToolkitProgram {
        ToolkitProgram {
            open: false,
            windows: Vec::new(),
        }
    }

    pub fn window_update(&mut self, message: Message) {
        window_message(&mut self.windows, message, true);
        self.open = !self.windows.is_empty();
    }
}

fn window_message(windows: &mut Vec<ToolkitWindow>, message: Message, recurse: bool) {
    match message {
        Message::CloseWindow => {
            windows.pop();
        }
        Message::OpenMenuMain => {
            windows.push(ToolkitWindow::MenuMain(crate::menu_main::MenuMain::new()));
        }
        Message::OpenLua(tk) => windows.push(ToolkitWindow::Lua(tk)),
        Message::OpenDialogueInput(msg) => {
            windows.push(ToolkitWindow::DlgInput(DlgInput::new(msg)));
        }
        _ => {
            if recurse {
                if let Some(wdw) = windows.last_mut() {
                    match wdw.update(message) {
                        Message::None => (),
                        msg => window_message(windows, msg, false),
                    }
                }
            }
        }
    }
}

impl iced_runtime::Program for ToolkitProgram {
    type Theme = Theme;
    type Message = Message;
    type Renderer = Renderer;

    fn update(&mut self, message: Message) -> Task<Message> {
        self.window_update(message);
        iced_runtime::Task::none()
    }

    fn view(&self) -> Element<'_, Message, Theme, Renderer> {
        let ele: Vec<Element<'_, Message, Theme, Renderer>> =
            self.windows.iter().map(|w| w.view()).collect();
        iced_widget::Stack::with_children(ele).into()
    }
}

pub static MESSAGE_QUEUE: std::sync::Mutex<Vec<Message>> = std::sync::Mutex::new(Vec::new());

pub struct Toolkit<'a> {
    theme: Theme,
    device: &'a wgpu::Device,
    queue: &'a wgpu::Queue,
    renderer: iced_wgpu::Renderer,
    viewport: iced_wgpu::graphics::Viewport,
    debug: iced_runtime::Debug,
    cursor_position: iced_core::mouse::Cursor,
    state: iced_runtime::program::State<ToolkitProgram>,
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
            ToolkitProgram::new(),
            viewport.logical_size(),
            &mut renderer,
            &mut debug,
        );
        state.queue_event(iced::Event::Window(iced::window::Event::RedrawRequested(
            std::time::Instant::now(),
        )));
        Toolkit {
            theme: iced::theme::Theme::custom(String::from("Naev"), PALETTE),
            device,
            queue,
            renderer,
            viewport,
            debug,
            cursor_position: iced_core::mouse::Cursor::Unavailable,
            state,
        }
    }

    pub fn update_cursor_position(&mut self, x: f32, y: f32) {
        let s = 1.0 / self.viewport.scale_factor() as f32;
        self.cursor_position =
            iced_core::mouse::Cursor::Available(iced_core::Point::new(x * s, y * s));
    }

    pub fn queue_event(&mut self, event: iced_core::Event) {
        if !self.state.program().open {
            return;
        }
        self.state.queue_event(event)
    }

    pub fn queue_message(&mut self, message: Message) {
        self.state.queue_message(message)
    }

    pub fn update(
        &mut self,
        clipboard: &mut impl iced_core::Clipboard,
        lua_th: &mut Option<mlua::Thread>,
    ) {
        let mut mq = MESSAGE_QUEUE.lock().unwrap();
        while let Some(m) = mq.pop() {
            self.queue_message(m);
        }

        let nw = self.state.program().windows.len();

        // We update iced
        let _ = self.state.update(
            self.viewport.logical_size(),
            self.cursor_position,
            &mut self.renderer,
            &self.theme,
            &iced_core::renderer::Style::default(),
            clipboard,
            &mut self.debug,
        );

        // Run Lua if window was closed. TODO check if window was closed and another was opened
        if let Some(th) = lua_th {
            if self.state.program().windows.len() < nw {
                dbg!(nw, self.state.program().windows.len());
                dbg!("resume");
                th.resume::<()>(()).unwrap();
                if th.status() != mlua::ThreadStatus::Resumable {
                    *lua_th = None;
                }
            }
        };
    }

    pub fn draw(
        &mut self,
        engine: &mut iced_wgpu::Engine,
        view: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        frame: &wgpu::SurfaceTexture,
    ) {
        if !self.state.program().open {
            return;
        }

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
}
