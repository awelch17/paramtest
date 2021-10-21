use std::time::Instant;

use imgui::{Condition, Context};
use imgui_wgpu::{Renderer, RendererConfig};

use crate::params::WINDOW_DIMENSIONS;


struct State {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration
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
        
        let state = State {
            surface: surface,
            device: device,
            queue: queue,
            config: config
        };
        
        Self { state: state }
    }
    
    pub fn draw_frame(&mut self) {
        if let Ok(output) = self.state.surface.get_current_texture() {
            let view = output.texture.create_view(&wgpu::TextureViewDescriptor::default());
            
            let mut encoder = self.state.device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
            
            {
                let _render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                    label: Some("Render Pass"),
                    color_attachments: &[wgpu::RenderPassColorAttachment {
                        view: &view,
                        resolve_target: None,
                        ops: wgpu::Operations {
                            load: wgpu::LoadOp::Clear(wgpu::Color {
                                r: 1.0,
                                g: 0.2,
                                b: 0.3,
                                a: 1.0,
                            }),
                            store: true,
                        },
                    }],
                    depth_stencil_attachment: None,
                });
            }
        
            // submit will accept anything that implements IntoIter
            self.state.queue.submit(std::iter::once(encoder.finish()));
            output.present();
        
        }
        
    }
}
