use std::collections::HashMap;

use wgpu::{Device, Face, ShaderModule, TextureFormat};

use crate::renderer::{GPUResourceManager, Texture};
use crate::renderer::mesh::InstanceTileRaw;
use crate::renderer::vertex::Vertex;

#[derive(Debug, Hash, Clone)]
struct PipelineDesc {
    // pub shader: String,
    pub primitive_topology: wgpu::PrimitiveTopology,
    pub depth_stencil: Option<wgpu::DepthStencilState>,
    pub use_instance: bool,

    pub sample_count: u32,
    pub sampler_mask: u64,
    pub alpha_to_coverage_enabled: bool,

    pub layouts: Vec<String>,
    pub front_face: wgpu::FrontFace,
    pub cull_mode: Option<Face>,
    // pub depth_bias: i32,
}

impl Default for PipelineDesc {
    fn default() -> Self {
        Self {
            // shader: "".to_string(),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            // color_states:vec![],
            use_instance: true,
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::LessEqual,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            sample_count: 1,
            sampler_mask: 0,
            alpha_to_coverage_enabled: true,
            layouts: vec!["camera_bind_group_layout".to_string(), "texture_bind_group_layout".to_string()],
            front_face: wgpu::FrontFace::Ccw,
            // cull_mode: Some(Face::Back),
            cull_mode: None,
            // depth_bias: 0,
        }
    }
}

impl PipelineDesc {
    fn build (
        &self ,
        shader: ShaderModule,
        device: &Device,
        default_format : TextureFormat,
        gpu_resource_manager : &GPUResourceManager
    ) -> wgpu::RenderPipeline {

        //여기 코드 이상하다...
        //이러면 gpu_resource_manager에 있는 데이터가 아니라,
        //unwrap 해서 새로 생성된 데이터들이 스코프에 묶여서 사용되는거 아닌가?
        //이거 어떻게 해야 예쁘게 되는거지?
        let bind_group_layouts = self
                .layouts
                .iter()
                .map(|group_name| {
                    gpu_resource_manager
                            .get_bind_group_layout(group_name)
                            .unwrap()
                })
                .collect::<Vec<_>>();

        let bind_group_layout_ref = bind_group_layouts
                .iter()
                .map(|l| {
                    l.as_ref()
                })
                .collect::<Vec<_>>();


        let vertex_buffer = if self.use_instance {
            vec![Vertex::desc(), InstanceTileRaw::desc()]
        } else {
            vec![Vertex::desc()]
        };


        let render_pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Render Pipeline Layout"),
            bind_group_layouts: &bind_group_layout_ref,
            push_constant_ranges: &[],
        });


        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Render Pipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &vertex_buffer,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: default_format,
                    blend: Some(wgpu::BlendState::ALPHA_BLENDING),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: wgpu::PrimitiveState {
                topology: self.primitive_topology,
                strip_index_format: None,
                front_face: self.front_face,
                cull_mode: self.cull_mode,
                // Setting this to anything other than Fill requires Features::NON_FILL_POLYGON_MODE
                polygon_mode: wgpu::PolygonMode::Fill,
                // Requires Features::DEPTH_CLIP_CONTROL
                unclipped_depth: false,
                // Requires Features::CONSERVATIVE_RASTERIZATION
                conservative: false,
            },
            depth_stencil: self.depth_stencil.clone(),
            multisample: wgpu::MultisampleState {
                count: self.sample_count, // 2.
                mask: !self.sampler_mask, // 3.
                alpha_to_coverage_enabled: self.alpha_to_coverage_enabled, // 4.
            },

            multiview: None,
        });

        render_pipeline
    }
}

pub struct PipelineManager{
    pipelines : HashMap<String ,  wgpu::RenderPipeline>
}

impl Default for PipelineManager {
    fn default() -> Self {
        let pipeline_manager = Self { pipelines: Default::default() };
        pipeline_manager
    }
}

impl PipelineManager {
    pub fn init_pipelines(
        &mut self,
        device: &Device,
        default_format: TextureFormat,
        gpu_resource_manager: &GPUResourceManager,
    ) {
        let shader = device.create_shader_module(wgpu::include_wgsl!("../../assets/image.wgsl"));
        let render_pipeline = PipelineDesc::default().build(shader, &device, default_format, &gpu_resource_manager);
        self.pipelines.insert("tile_pl".to_string(), render_pipeline);
    }

    pub fn get_pipeline(&self , name: &str) -> &wgpu::RenderPipeline{
        self.pipelines.get(name).unwrap()
    }
}