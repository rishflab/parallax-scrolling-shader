use crate::{
    asset::Vertex,
    camera::Camera,
    scene::Scene,
    sprite::{DrawSprite, Sprite},
    texture::Texture,
};
use std::path::Path;
use wgpu::{
    util::DeviceExt, BlendFactor, BlendOperation, CullMode, FrontFace, RasterizationStateDescriptor,
};

pub(crate) struct Renderer {
    sprites: Vec<Sprite>,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    depth_texture: Texture,
    camera: Camera,
}

impl Renderer {
    pub(crate) fn init(
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &mut wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        use std::mem;

        let camera = Camera::new(
            glam::Vec3::new(0.0, 10.0, 0.0),
            glam::Vec3::new(0.0, 0.0, 0.0),
            6.0,
            sc_desc.width as f32 / sc_desc.height as f32,
        );

        let mx_total = camera.generate_matrix();
        let mx_ref: &[f32; 16] = mx_total.as_ref();

        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::cast_slice(mx_ref),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let uniform_buffer_binding_resource =
            wgpu::BindingResource::Buffer(uniform_buffer.slice(..));

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::UniformBuffer {
                        dynamic: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::StorageBuffer {
                        dynamic: false,
                        min_binding_size: None,
                        readonly: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::SampledTexture {
                        multisampled: true,
                        component_type: wgpu::TextureComponentType::Float,
                        dimension: wgpu::TextureViewDimension::D2,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler { comparison: false },
                    count: None,
                },
            ],
        });

        // Load sprites onto GPU
        let sprites = vec![
            Sprite::new(
                device,
                queue,
                &bind_group_layout,
                &uniform_buffer_binding_resource,
                Path::new(&"assets/leaves.png"),
                "leaves".to_string(),
            ),
            Sprite::new(
                device,
                queue,
                &bind_group_layout,
                &uniform_buffer_binding_resource,
                Path::new(&"assets/pepe.png"),
                "pepe".to_string(),
            ),
        ];

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        // Create the render pipeline
        let vs_module =
            device.create_shader_module(wgpu::include_spirv!("../assets/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(wgpu::include_spirv!("../assets/shader.frag.spv"));

        let depth_texture = Texture::create_depth_texture(&device, &sc_desc, "depth_texture");

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex_stage: wgpu::ProgrammableStageDescriptor {
                module: &vs_module,
                entry_point: "main",
            },
            fragment_stage: Some(wgpu::ProgrammableStageDescriptor {
                module: &fs_module,
                entry_point: "main",
            }),
            rasterization_state: Some(RasterizationStateDescriptor {
                front_face: FrontFace::Ccw,
                cull_mode: CullMode::None,
                depth_bias: 0,
                depth_bias_slope_scale: 0.0,
                depth_bias_clamp: 0.0,
                clamp_depth: false,
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor {
                    src_factor: BlendFactor::SrcAlpha,
                    dst_factor: BlendFactor::OneMinusSrcAlpha,
                    operation: BlendOperation::Add,
                },
                alpha_blend: wgpu::BlendDescriptor {
                    src_factor: BlendFactor::One,
                    dst_factor: BlendFactor::One,
                    operation: BlendOperation::Min,
                },
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: Some(wgpu::DepthStencilStateDescriptor {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilStateDescriptor::default(),
            }),
            vertex_state: wgpu::VertexStateDescriptor {
                index_format: wgpu::IndexFormat::Uint16,
                vertex_buffers: &[wgpu::VertexBufferDescriptor {
                    stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float4,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttributeDescriptor {
                            format: wgpu::VertexFormat::Float2,
                            offset: 4 * 4,
                            shader_location: 1,
                        },
                    ],
                }],
            },
            sample_count: 1,
            sample_mask: !0,
            alpha_to_coverage_enabled: false,
        });

        // Done
        Renderer {
            uniform_buffer,
            pipeline,
            camera,
            sprites,
            depth_texture,
        }
    }

    pub(crate) fn resize(
        &mut self,
        sc_desc: &wgpu::SwapChainDescriptor,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        self.camera.aspect_ratio = sc_desc.width as f32 / sc_desc.height as f32;
        let mx_total = self.camera.generate_matrix();
        let mx_ref: &[f32; 16] = mx_total.as_ref();
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mx_ref));
    }

    pub(crate) fn render(
        &mut self,
        frame: &wgpu::SwapChainTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _sc_desc: &wgpu::SwapChainDescriptor,
        scene: Scene,
    ) {
        let mx_total = self.camera.generate_matrix();
        let mx_ref: &[f32; 16] = mx_total.as_ref();
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mx_ref));

        for sprite in self.sprites.iter_mut() {
            if let Some(instances) = scene.sprite_instances.get(&sprite.id) {
                sprite.update_instance_buffer(instances.clone(), queue);
            }
        }

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 0.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachmentDescriptor {
                    attachment: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(0.5),
                        store: true,
                    }),
                    stencil_ops: None,
                }),
            });

            rpass.set_pipeline(&self.pipeline);

            for sprite in self.sprites.iter_mut() {
                if let Some(instances) = scene.sprite_instances.get(&sprite.id) {
                    rpass.draw_sprite(sprite, 0..instances.len() as u32, &sprite.bind_group);
                }
            }
        }

        queue.submit(Some(encoder.finish()));
    }
}
