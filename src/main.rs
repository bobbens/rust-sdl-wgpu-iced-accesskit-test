mod controls;
mod scene;
mod iced_sdl;

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
    state.queue_event( iced::Event::Window(iced::window::Event::RedrawRequested(std::time::Instant::now())) );

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
            if let Some(event) = iced_sdl::window_event(
                &event,
                1.0, //window.scale_factor(),
                sdl_context.keyboard().mod_state(),
            ) {
                state.queue_event(event);
            }
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

            let program = state.program();

            // We clear the frame
            let mut render_pass = Scene::clear(
                &view,
                &mut encoder,
                program.background_color(),
                //iced_core::Color::TRANSPARENT,
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
