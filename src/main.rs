//mod controls;
mod iced_sdl;
//mod menu_main;
mod scene;
mod toolkit;
mod toolkit_lua;

//use controls::Controls;
//use menu_main::MenuMain;
use scene::Scene;

use iced_wgpu::wgpu;

use sdl2::event::{Event, WindowEvent};
use sdl2::keyboard::Keycode;

pub fn main() -> Result<(), String> {
    // Show logs from wgpu
    env_logger::init();

    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let window = video_subsystem
        .window("Raw Window Handle Example", 800, 600)
        .position_centered()
        .resizable()
        .allow_highdpi()
        .build()
        .map_err(|e| e.to_string())?;
    let (width, height) = window.size();

    let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
        backends: wgpu::Backends::PRIMARY, //backends: wgpu::Backends::GL,
        ..Default::default()
    });
    let surface = unsafe {
        match instance
            .create_surface_unsafe(wgpu::SurfaceTargetUnsafe::from_window(&window).unwrap())
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
        format,
        width,
        height,
        present_mode: wgpu::PresentMode::Fifo,
        alpha_mode: wgpu::CompositeAlphaMode::Auto,
        view_formats: Vec::default(),
        desired_maximum_frame_latency: 2,
    };
    surface.configure(&device, &config);

    let scale_factor = 1.2; // TODO hook with SDL or something
    let mut scene = Scene::new(&device, &queue, format);
    let mut engine = iced_wgpu::Engine::new(&adapter, &device, &queue, format, None);
    let mut program =
        toolkit_lua::Toolkit::new(&mut engine, &device, &queue, scale_factor, width, height);

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
                Event::MouseMotion { x, y, .. }
                | Event::MouseButtonDown { x, y, .. }
                | Event::MouseButtonUp { x, y, .. } => {
                    let s = 1.0 / scale_factor as f32;
                    let fx = (*x as f32) * s;
                    let fy = (*y as f32) * s;
                    program.update_cursor_position(iced_core::mouse::Cursor::Available(
                        iced_core::Point::new(fx, fy),
                    ));
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
            if let Some(evt) = iced_sdl::window_event(&event, scale_factor) {
                program.state.queue_event(evt);
            }
        }

        program.update();

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
                if let wgpu::SurfaceError::Outdated | wgpu::SurfaceError::Lost = err {
                    let (w, h) = window.size();
                    config.width = w;
                    config.height = h;
                    surface.configure(&device, &config);
                }
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
            // We clear the frame
            let mut render_pass = scene.clear(&view, &mut encoder, iced_core::Color::BLACK);

            // Draw the scene
            scene.draw(&mut render_pass);
        }
        program.draw(&mut engine, &view, &mut encoder, &frame);
        engine.submit(&queue, encoder);
        frame.present();

        scene.update(0.01);
    }

    Ok(())
}
