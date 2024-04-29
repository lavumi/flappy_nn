use std::collections::HashMap;
use std::iter;
use wgpu::RequestAdapterOptions;

use winit::window::Window;

use crate::renderer::gpu_resource_manager::GPUResourceManager;
use crate::renderer::pipeline_manager::PipelineManager;
use crate::renderer::font_manager::FontManager;
use crate::renderer::render_input_data::*;
use crate::renderer::texture;


pub struct RenderState {
    pub device: wgpu::Device,
    surface: wgpu::Surface,

    pub queue: wgpu::Queue,
    pub config: wgpu::SurfaceConfiguration,

    pub gpu_resource_manager: GPUResourceManager,
    pub pipeline_manager: PipelineManager,

    font_manager: FontManager,

    color: wgpu::Color,
    depth_texture: texture::TextureViewAndSampler,

    aspect_ratio: f32,
    viewport_data: [f32; 6],
}
impl RenderState {
    pub async fn new(window: &Window) -> Self {
        let size = window.inner_size();

        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });


        // # Safety
        // The surface needs to live as long as the window that created it.
        // State owns the window so this should be safe.
        let surface = unsafe { instance.create_surface(&window) }.unwrap();
        let adapter = instance
                .request_adapter(&wgpu::RequestAdapterOptions {
                    power_preference: wgpu::PowerPreference::default(),
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
                        // WebGL doesn't support all of wgpu`s features, so if
                        // we're building for the web we'll have to disable some.
                        limits: if cfg!(target_arch = "wasm32") {
                            wgpu::Limits::downlevel_webgl2_defaults()
                        } else {
                            wgpu::Limits::default()
                        },
                    },
                    // Some(&std::path::Path::new("trace")), // Trace path
                    None,
                )
                .await
                .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);
        let surface_format = surface_caps.formats.iter()
                .copied()
                // .filter(|f| f.describe().srgb)
                .next()
                .unwrap_or(surface_caps.formats[0]);
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };
        surface.configure(&device, &config);


        let depth_texture = texture::TextureViewAndSampler::create_depth_texture(&device, &config, "depth_texture");
        let color = wgpu::Color { r: 0.0, g: 0.0, b: 0.0, a: 1.0 };

        let aspect_ratio = size.width as f32 / size.height as f32;
        let viewport_data = [0., 0., size.width as f32, size.height as f32, 0., 1.];

        let mut gpu_resource_manager = GPUResourceManager::default();
        gpu_resource_manager.initialize(&device);
        let mut pipeline_manager = PipelineManager::default();
        pipeline_manager.init_pipelines(&device, config.format, &gpu_resource_manager);


        let font_manager = FontManager::default();


        Self {
            device,
            surface,
            queue,
            config,
            gpu_resource_manager,
            pipeline_manager,
            color,
            depth_texture,
            aspect_ratio,
            viewport_data,
            font_manager,
        }
    }

    pub async fn init_resources(&mut self) {
        self.gpu_resource_manager.init_atlas(&self.device, &self.queue);
        self.gpu_resource_manager.init_meshes(&self.device);



        self.gpu_resource_manager.init_ui_meshes(&self.device);

        let font_texture = self.font_manager.make_font_atlas_rgba(&self.device, &self.queue).await.unwrap();
        self.gpu_resource_manager.init_ui_atlas(&self.device,font_texture);
    }

    #[allow(dead_code)]
    pub fn set_clear_color(&mut self, color: wgpu::Color) {
        self.color = color;
    }
    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.depth_texture = texture::TextureViewAndSampler::create_depth_texture(&self.device, &self.config, "depth_texture");
            self.surface.configure(&self.device, &self.config);

            let aspect_ratio = new_size.width as f32 / new_size.height as f32;

            if (self.aspect_ratio - aspect_ratio).abs() > 0.02 {
                if self.aspect_ratio < aspect_ratio { //width is bigger
                    let adjust_width = new_size.height as f32 * self.aspect_ratio;
                    let x_offset = (new_size.width as f32 - adjust_width) * 0.5;

                    self.viewport_data = [x_offset, 0., adjust_width, new_size.height as f32, 0., 1.];
                } else {
                    let adjust_height = new_size.width as f32 / self.aspect_ratio;
                    self.viewport_data = [0., 0., new_size.width as f32, adjust_height, 0., 1.];
                }
            } else {
                self.viewport_data = [0., 0., new_size.width as f32, new_size.height as f32, 0., 1.];
            }
        }
    }
    pub fn update_camera_buffer(&self, camera_uniform: [[f32; 4]; 4]) {
        let camera_buffer = self.gpu_resource_manager.get_buffer("camera_matrix");
        self.queue.write_buffer(&camera_buffer, 0, bytemuck::cast_slice(&[camera_uniform]));
    }

    pub fn update_mesh_instance(&mut self, tile_render_data: HashMap<String, Vec<TileRenderData>>) {

        for pair in tile_render_data {

            let instance_data = (pair.1)
                    .iter()
                    .map(|data|{
                data.get_instance_matrix()
            }).collect::<Vec<_>>();


            self.gpu_resource_manager.update_mesh_instance(pair.0, &self.device, &self.queue, instance_data);
        }
    }

    pub fn update_text_instance(&mut self, texts: Vec<TextRenderData>) {
        let tile_instance =  texts
                .iter()
                .flat_map(|text|{
                    self.font_manager.make_instance_buffer( text )
                }).collect::<Vec<_>>();


        self.gpu_resource_manager.update_color_mesh_instance("font", &self.device, &self.queue, tile_instance);

    }

    pub fn render(&self) -> Result<(), wgpu::SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
                .texture
                .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = self
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(self.color),
                        store: true,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            render_pass.set_viewport(self.viewport_data[0],
                self.viewport_data[1],
                self.viewport_data[2],
                self.viewport_data[3],
                self.viewport_data[4],
                self.viewport_data[5]);

            let render_pipeline = self.pipeline_manager.get_pipeline("tile_pl");
            render_pass.set_pipeline(render_pipeline);
            self.gpu_resource_manager.render(&mut render_pass);


            let render_pipeline = self.pipeline_manager.get_pipeline("font_pl");
            render_pass.set_pipeline(render_pipeline);
            self.gpu_resource_manager.render_ui(&mut render_pass);
        }


        self.queue.submit(iter::once(encoder.finish()));
        output.present();
        Ok(())
    }


    pub async fn make_font_atlas() {
        // The instance is a handle to our GPU
        // BackendBit::PRIMARY => Vulkan + Metal + DX12 + Browser WebGPU
        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            backends: wgpu::Backends::all(),
            dx12_shader_compiler: Default::default(),
        });

        let adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                force_fallback_adapter: false,
                compatible_surface: None,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(&Default::default(), None)
            .await
            .unwrap();

        let mut font_manager = FontManager::default();

        let rasterized_font = font_manager.font_rasterize(24.0);
        let u8_size = std::mem::size_of::<u8>() as u32;
        let output_buffer_size = (u8_size * 256 * 256 * 4) as wgpu::BufferAddress;
        let output_buffer_desc = wgpu::BufferDescriptor {
            size: output_buffer_size,
            usage: wgpu::BufferUsages::MAP_READ | wgpu::BufferUsages::COPY_DST,
            label: Some("font atlas buffer"),
            mapped_at_creation: false,
        };
        let output_buffer = device.create_buffer(&output_buffer_desc);
        let output_buffer = font_manager.make_font_butter(rasterized_font,output_buffer, &device, &queue).unwrap();

        {
            let buffer_slice = output_buffer.slice(..);

            // NOTE: We have to create the mapping THEN device.poll() before await
            // the future. Otherwise, the application will freeze.
            let (tx, rx) = futures_intrusive::channel::shared::oneshot_channel();
            buffer_slice.map_async(wgpu::MapMode::Read, move |result| {
                tx.send(result).unwrap();
            });
            device.poll(wgpu::Maintain::Wait);
            rx.receive().await.unwrap().unwrap();

            let data = buffer_slice.get_mapped_range();

            use image::{ImageBuffer, Rgba};
            let buffer = ImageBuffer::<Rgba<u8>, _>::from_raw(256, 256, data).unwrap();
            buffer.save("assets/img/font.png").unwrap();
        }
        output_buffer.unmap();

    }
}