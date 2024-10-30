mod controls;
mod scene;

use controls::Controls;
use scene::Scene;

/*
use iced_winit::conversion;
use iced_winit::core::{mouse,renderer};
use iced_winit::core::{Color, Font, Pixels, Size, Theme};
use iced_winit::{winit, futures};
use iced_winit::runtime::{program,Debug};
use iced_winit::Clipboard;
*/
use iced::{Font, Pixels, Size};
use iced_runtime::{program,Debug};
use iced_wgpu::graphics::Viewport;
use iced_wgpu::{wgpu, Engine, Renderer};

//use std::borrow::Cow;
//use std::collections::HashMap;
//use wgpu::SurfaceError;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

/*
use winit::{
    //event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    keyboard::ModifiersState,
};
*/

//use std::sync::Arc;

//pub fn main() -> Result<(), winit::error::EventLoopError> {
pub fn main() -> Result<(), String> {
    // Show logs from wgpu
    env_logger::init();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Raw Window Handle Example", 800, 600)
        .position_centered()
        .resizable()
        .metal_view()
        .allow_highdpi()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;
    let (width, height) = window.size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        //backends: wgpu::Backends::GL,
        backends: wgpu::Backends::PRIMARY,
        ..Default::default()
    });
    let surface = unsafe {
        match instance
            .create_surface_unsafe( wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap() )
        {
            Ok(s) => s,
            Err(e) => return Err(e.to_string()),
        }
    };
    let adapter_opt = pollster::block_on(instance.request_adapter(&wgpu::RequestAdapterOptions {
        power_preference: wgpu::PowerPreference::HighPerformance,
        force_fallback_adapter: false,
        compatible_surface: Some(&surface),
    }));
    let adapter = match adapter_opt {
        Some(a) => a,
        None => return Err(String::from("No adapter found")),
    };

    let (device, queue) = match pollster::block_on(adapter.request_device(
        &wgpu::DeviceDescriptor {
            required_limits: wgpu::Limits::default(),
            label: Some("device"),
            required_features: wgpu::Features::empty(),
            memory_hints: Default::default(),
        },
        None,
    )) {
        Ok(a) => a,
        Err(e) => return Err(e.to_string()),
    };

    let format = wgpu::TextureFormat::Bgra8UnormSrgb;

    let mut config = wgpu::SurfaceConfiguration {
        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        format: format,
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: Vec::default(),
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    let mut engine =
        Engine::new(&adapter, &device, &queue, format, None);
    let mut renderer = Renderer::new(
        &device,
        &engine,
        Font::default(),
        Pixels::from(16),
    );
    let viewport = Viewport::with_physical_size(
        Size::new(width, height),
        1.0,
        //window.scale_factor(),
    );
    let mut debug = Debug::new();
    let controls = Controls::new();
    let scene = Scene::new(&device, format);
    let mut state = program::State::new(
        controls,
        viewport.logical_size(),
        &mut renderer,
        &mut debug,
    );

    let mut cursor_position = iced_core::mouse::Cursor::Unavailable;
    let mut event_pump = sdl_context.event_pump()?;
    'running: loop {
        for event in event_pump.poll_iter() {

            match &event {
                Event::Window {
                    window_id,
                    win_event: WindowEvent::SizeChanged(width, height),
                    ..
                } if *window_id == window.id() => {
                    config.width = *width as u32;
                    config.height = *height as u32;
                    surface.configure(&device, &config);
                }
                Event::MouseMotion {
                    x,
                    y,
                    ..
                } | Event::MouseButtonDown {
                    x,
                    y,
                    ..
                } | Event::MouseButtonUp {
                    x,
                    y,
                    ..
                } => {
                    cursor_position = iced_core::mouse::Cursor::Available( iced_core::Point::new(*x as f32, *y as f32 ));
                }
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    break 'running;
                }
                _e => {
                    //dbg!(e);
                }
            }

            // Map window event to iced event
            if let Some(event) = sdl_to_iced_window_event(
                &event,
                1.0, //window.scale_factor(),
                sdl_context.keyboard().mod_state(),
            ) {
                state.queue_event(event);
            }

            // If there are events pending
            if !state.is_queue_empty() {
                // We update iced
                let _ = state.update(
                    viewport.logical_size(),
                    cursor_position,
                    &mut renderer,
                    &iced_core::Theme::Dark,
                    &iced_core::renderer::Style {
                        text_color: iced_core::Color::WHITE,
                    },
                    &mut iced_core::clipboard::Null,
                    &mut debug,
                );

                // and request a redraw
                //window.request_redraw();
            }
        }



        let frame = match surface.get_current_texture() {
            Ok(frame) => frame,
            Err(err) => {
                let reason = match err {
                    wgpu::SurfaceError::Timeout => "Timeout",
                    wgpu::SurfaceError::Outdated => "Outdated",
                    wgpu::SurfaceError::Lost => "Lost",
                    wgpu::SurfaceError::OutOfMemory => "OutOfMemory",
                };
                println!("Failed to get current surface texture! Reason: {}", reason);
                continue 'running;
            }
        };

        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("command_encoder"),
        });

        {
            /*
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::GREEN),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                label: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });
            rpass.set_pipeline(&render_pipeline);
            rpass.set_bind_group(0, &bind_group, &[]);
            rpass.draw(0..3, 0..1);
            */

            // We clear the frame
            let mut render_pass = Scene::clear(
                &view,
                &mut encoder,
                iced_core::Color::TRANSPARENT,
            );

            // Draw the scene
            scene.draw(&mut render_pass);
        }
        renderer.present( &mut engine, &device, &queue, &mut encoder, None, frame.texture.format(), &view, &viewport, &debug.overlay(), );
        //queue.submit([encoder.finish()]);
        engine.submit(&queue, encoder);
        frame.present();
    }

    Ok(())
}

/*
    // Initialize winit
    let event_loop = EventLoop::new()?;

    #[allow(clippy::large_enum_variant)]
    enum Runner {
        Loading,
        Ready {
            window: Arc<winit::window::Window>,
            device: wgpu::Device,
            queue: wgpu::Queue,
            surface: wgpu::Surface<'static>,
            format: wgpu::TextureFormat,
            engine: Engine,
            renderer: Renderer,
            scene: Scene,
            state: program::State<Controls>,
            cursor_position: Option<winit::dpi::PhysicalPosition<f64>>,
            clipboard: Clipboard,
            viewport: Viewport,
            modifiers: ModifiersState,
            resized: bool,
            debug: Debug,
        },
    }

    impl winit::application::ApplicationHandler for Runner {
        fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
            if let Self::Loading = self {
                let window = Arc::new(
                    event_loop
                        .create_window(
                            winit::window::WindowAttributes::default(),
                        )
                        .expect("Create window"),
                );

                let physical_size = window.inner_size();
                let viewport = Viewport::with_physical_size(
                    Size::new(physical_size.width, physical_size.height),
                    window.scale_factor(),
                );
                let clipboard = Clipboard::connect(window.clone());

                let backend =
                    wgpu::util::backend_bits_from_env().unwrap_or_default();

                let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                    backends: backend,
                    ..Default::default()
                });
                let surface = instance
                    .create_surface(window.clone())
                    .expect("Create window surface");

                let (format, adapter, device, queue) =
                    futures::futures::executor::block_on(async {
                        let adapter =
                            wgpu::util::initialize_adapter_from_env_or_default(
                                &instance,
                                Some(&surface),
                            )
                            .await
                            .expect("Create adapter");

                        let adapter_features = adapter.features();

                        let capabilities = surface.get_capabilities(&adapter);

                        let (device, queue) = adapter
                            .request_device(
                                &wgpu::DeviceDescriptor {
                                    label: None,
                                    required_features: adapter_features
                                        & wgpu::Features::default(),
                                    required_limits: wgpu::Limits::default(),
                                    //memory_hints:wgpu::MemoryHints::MemoryUsage,
                                },
                                None,
                            )
                            .await
                            .expect("Request device");

                        (
                            capabilities
                                .formats
                                .iter()
                                .copied()
                                .find(wgpu::TextureFormat::is_srgb)
                                .or_else(|| {
                                    capabilities.formats.first().copied()
                                })
                                .expect("Get preferred format"),
                            adapter,
                            device,
                            queue,
                        )
                    });

                surface.configure(
                    &device,
                    &wgpu::SurfaceConfiguration {
                        usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                        format,
                        width: physical_size.width,
                        height: physical_size.height,
                        present_mode: wgpu::PresentMode::AutoVsync,
                        alpha_mode: wgpu::CompositeAlphaMode::Auto,
                        view_formats: vec![],
                        desired_maximum_frame_latency: 2,
                    },
                );

                // Initialize scene and GUI controls
                let scene = Scene::new(&device, format);
                let controls = Controls::new();

                // Initialize iced
                let mut debug = Debug::new();
                let engine =
                    Engine::new(&adapter, &device, &queue, format, None);
                let mut renderer = Renderer::new(
                    &device,
                    &engine,
                    Font::default(),
                    Pixels::from(16),
                );

                let state = program::State::new(
                    controls,
                    viewport.logical_size(),
                    &mut renderer,
                    &mut debug,
                );

                // You should change this if you want to render continuously
                event_loop.set_control_flow(ControlFlow::Wait);

                *self = Self::Ready {
                    window,
                    device,
                    queue,
                    surface,
                    format,
                    engine,
                    renderer,
                    scene,
                    state,
                    cursor_position: None,
                    modifiers: ModifiersState::default(),
                    clipboard,
                    viewport,
                    resized: false,
                    debug,
                };
            }
        }

        fn window_event(
            &mut self,
            event_loop: &winit::event_loop::ActiveEventLoop,
            _window_id: winit::window::WindowId,
            event: WindowEvent,
        ) {
            let Self::Ready {
                window,
                device,
                queue,
                surface,
                format,
                engine,
                renderer,
                scene,
                state,
                viewport,
                cursor_position,
                modifiers,
                clipboard,
                resized,
                debug,
            } = self
            else {
                return;
            };

            match event {
                WindowEvent::RedrawRequested => {
                    if *resized {
                        let size = window.inner_size();

                        *viewport = Viewport::with_physical_size(
                            Size::new(size.width, size.height),
                            window.scale_factor(),
                        );

                        surface.configure(
                            device,
                            &wgpu::SurfaceConfiguration {
                                format: *format,
                                usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                                width: size.width,
                                height: size.height,
                                present_mode: wgpu::PresentMode::AutoVsync,
                                alpha_mode: wgpu::CompositeAlphaMode::Auto,
                                view_formats: vec![],
                                desired_maximum_frame_latency: 2,
                            },
                        );

                        *resized = false;
                    }

                    match surface.get_current_texture() {
                        Ok(frame) => {
                            let mut encoder = device.create_command_encoder(
                                &wgpu::CommandEncoderDescriptor { label: None },
                            );

                            let program = state.program();

                            let view = frame.texture.create_view(
                                &wgpu::TextureViewDescriptor::default(),
                            );

                            {
                                // We clear the frame
                                let mut render_pass = Scene::clear(
                                    &view,
                                    &mut encoder,
                                    program.background_color(),
                                );

                                // Draw the scene
                                scene.draw(&mut render_pass);
                            }

                            // And then iced on top
                            renderer.present(
                                engine,
                                device,
                                queue,
                                &mut encoder,
                                None,
                                frame.texture.format(),
                                &view,
                                viewport,
                                &debug.overlay(),
                            );

                            // Then we submit the work
                            engine.submit(queue, encoder);
                            frame.present();

                            // Update the mouse cursor
                            window.set_cursor(
                                iced_winit::conversion::mouse_interaction(
                                    state.mouse_interaction(),
                                ),
                            );
                        }
                        Err(error) => match error {
                            wgpu::SurfaceError::OutOfMemory => {
                                panic!(
                                    "Swapchain error: {error}. \
                                Rendering cannot continue."
                                )
                            }
                            _ => {
                                // Try rendering again next frame.
                                window.request_redraw();
                            }
                        },
                    }
                }
                WindowEvent::CursorMoved { position, .. } => {
                    *cursor_position = Some(position);
                }
                WindowEvent::ModifiersChanged(new_modifiers) => {
                    *modifiers = new_modifiers.state();
                }
                WindowEvent::Resized(_) => {
                    *resized = true;
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                _ => {}
            }

            // Map window event to iced event
            if let Some(event) = iced_winit::conversion::window_event(
                event,
                window.scale_factor(),
                *modifiers,
            ) {
                state.queue_event(event);
            }

            // If there are events pending
            if !state.is_queue_empty() {
                // We update iced
                let _ = state.update(
                    viewport.logical_size(),
                    cursor_position
                        .map(|p| {
                            conversion::cursor_position(
                                p,
                                viewport.scale_factor(),
                            )
                        })
                        .map(mouse::Cursor::Available)
                        .unwrap_or(mouse::Cursor::Unavailable),
                    renderer,
                    &Theme::Dark,
                    &renderer::Style {
                        text_color: Color::WHITE,
                    },
                    clipboard,
                    debug,
                );

                // and request a redraw
                window.request_redraw();
            }
        }
    }

    let mut runner = Runner::Loading;
    event_loop.run_app(&mut runner)
}
*/

pub fn sdl_to_iced_mouse_button(
    mouse_btn: &sdl2::mouse::MouseButton,
) -> iced::mouse::Button {
    match mouse_btn {
        sdl2::mouse::MouseButton::Left => iced::mouse::Button::Left,
        sdl2::mouse::MouseButton::Right => iced::mouse::Button::Right,
        sdl2::mouse::MouseButton::Middle => iced::mouse::Button::Middle,
        sdl2::mouse::MouseButton::X1 => iced::mouse::Button::Back,
        sdl2::mouse::MouseButton::X2 => iced::mouse::Button::Forward,
        sdl2::mouse::MouseButton::Unknown => iced::mouse::Button::Other(0),
    }
}

pub fn sdl_to_iced_window_event(
    event: &Event,
    _scale_factor: f64,
    _modifiers: sdl2::keyboard::Mod,
) -> Option<iced_core::Event> {
    match event {
        Event::Window {
            //window_id,
            win_event: WindowEvent::SizeChanged(width, height),
            ..
        } => {
            Some(iced_core::Event::Window(iced_core::window::Event::Resized(Size {
                width: *width as f32,
                height: *height as f32,
            })))
        }
        Event::Window {
            win_event: WindowEvent::Enter,
            ..
        } => {
            Some(iced_core::Event::Mouse(iced_core::mouse::Event::CursorEntered))
        }
        Event::Window {
            win_event: WindowEvent::Leave,
            ..
        } => {
            Some(iced_core::Event::Mouse(iced_core::mouse::Event::CursorLeft))
        }
        Event::MouseMotion {
            x,
            y,
            ..
        } => {
            Some(iced_core::Event::Mouse(iced_core::mouse::Event::CursorMoved {
                position: iced_core::Point::new(*x as f32, *y as f32),
            }))
        }
        Event::MouseButtonDown {
            mouse_btn,
            ..
        } => {
            let btn = sdl_to_iced_mouse_button( mouse_btn );
            Some(iced_core::Event::Mouse(iced_core::mouse::Event::ButtonPressed(btn)))
        }
        Event::MouseButtonUp {
            mouse_btn,
            ..
        } => {
            let btn = sdl_to_iced_mouse_button( mouse_btn );
            Some(iced_core::Event::Mouse(iced_core::mouse::Event::ButtonReleased(btn)))
        }
        Event::Quit { .. } => {
            Some(iced_core::Event::Window(iced_core::window::Event::CloseRequested))
        }
        _ => None,
    }
}
