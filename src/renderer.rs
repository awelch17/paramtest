use std::time::Instant;

use imgui::{Condition, Context};
use imgui_wgpu::{Renderer, RendererConfig};

use crate::params::WINDOW_DIMENSIONS;


struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    imgui: Context,
    renderer: Renderer,
    last_frame: Instant
}


pub struct ParamRenderer {
    state: State
}

impl ParamRenderer {
    pub async fn new<W: raw_window_handle::HasRawWindowHandle>(handle: W) -> Self {
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(&handle) };
        
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    features: wgpu::Features::empty(),
                    limits: wgpu::Limits::default(),
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();
        
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: WINDOW_DIMENSIONS.0 as u32,
            height: WINDOW_DIMENSIONS.1 as u32,
            present_mode: wgpu::PresentMode::Fifo,
        };
        surface.configure(&device, &config);
        
        let mut imgui = imgui::Context::create();
        
        let renderer_config = RendererConfig {
            ..Default::default()
        };
    
        let mut renderer = Renderer::new(&mut imgui, &device, &queue, renderer_config);
        
        
        let state = State {
            surface: surface,
            device: device,
            queue: queue,
            config: config,
            imgui: imgui,
            renderer: renderer,
            last_frame: Instant::now()
        };
        
        Self { state: state }
    }
    
    pub fn draw_frame(&mut self) {
        
        
        let delta_s = self.state.last_frame.elapsed();
        let now = Instant::now();
        self.state.imgui.io_mut().update_delta_time(now - self.state.last_frame);
        self.state.last_frame = now;
        
        let frame = match self.state.surface.get_current_texture() {
            Ok(frame) => frame,
            Err(e) => {
                eprintln!("dropped frame: {:?}", e);
                return;
            }
        };
        
        let ui = self.state.imgui.frame();
        
        {
            let window = imgui::Window::new("Hello world");
            window
                .size([WINDOW_DIMENSIONS.0 as f32, WINDOW_DIMENSIONS.1 as f32], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text("Hello world!");
                    ui.text("This...is...imgui-rs on WGPU!");
                    ui.separator();
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1},{:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                });

            let window = imgui::Window::new("Hello too");
            window
                .size([400.0, 200.0], Condition::FirstUseEver)
                .position([400.0, 200.0], Condition::FirstUseEver)
                .build(&ui, || {
                    ui.text(format!("Frametime: {:?}", delta_s));
                });
                
            ui.show_demo_window(&mut true);
            
            
        }
        
        let mut encoder: wgpu::CommandEncoder = self.state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        
        let clear_color = wgpu::Color {
            r: 0.1,
            g: 0.2,
            b: 0.3,
            a: 1.0,
        };
        
        let view = frame
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());
        let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[wgpu::RenderPassColorAttachment {
                view: &view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(clear_color),
                    store: true,
                },
            }],
            depth_stencil_attachment: None,
        });
        self.state.renderer
            .render(ui.render(), &self.state.queue, &self.state.device, &mut rpass)
            .expect("Rendering failed");
        drop(rpass);
        self.state.queue.submit(Some(encoder.finish()));
        frame.present();
    }
}
