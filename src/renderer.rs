use crate::{
    camera::ParallaxCamera,
    gpu_primitives::{CameraUniform, Vertex},
    scene::Scene,
    sprite::{DrawSprite, Sprite},
    texture::Texture,
};
use std::{mem, path::Path};
use wgpu::{util::DeviceExt, BlendFactor, BlendOperation, CullMode, FrontFace, VertexBufferLayout};

pub(crate) struct Renderer {
    sprites: Vec<Sprite>,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
    depth_texture: Texture,
}

impl Renderer {
    pub(crate) fn init(
        sc_desc: &wgpu::SwapChainDescriptor,
        device: &mut wgpu::Device,
        queue: &wgpu::Queue,
    ) -> Self {
        let camera_matrix = ParallaxCamera::default().camera_uniform();
        let uniform_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Uniform Buffer"),
            contents: bytemuck::bytes_of(&camera_matrix),
            usage: wgpu::BufferUsage::UNIFORM | wgpu::BufferUsage::COPY_DST,
        });

        let uniform_buffer_binding_resource = uniform_buffer.as_entire_binding();

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: None,
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        min_binding_size: wgpu::BufferSize::new(
                            mem::size_of::<CameraUniform>() as _
                        ),
                        has_dynamic_offset: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStage::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Storage { read_only: false },
                        min_binding_size: None,
                        has_dynamic_offset: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 3,
                    visibility: wgpu::ShaderStage::FRAGMENT,
                    ty: wgpu::BindingType::Sampler {
                        filtering: false,
                        comparison: false,
                    },
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
                Path::new(&"assets/square.png"),
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
            device.create_shader_module(&wgpu::include_spirv!("../shaders/shader.vert.spv"));
        let fs_module =
            device.create_shader_module(&wgpu::include_spirv!("../shaders/shader.frag.spv"));

        let depth_texture = Texture::create_depth_texture(&device, &sc_desc);

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &vs_module,
                entry_point: "main",
                buffers: &[VertexBufferLayout {
                    array_stride: mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::InputStepMode::Vertex,
                    attributes: &[
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float4,
                            offset: 0,
                            shader_location: 0,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float2,
                            offset: wgpu::VertexFormat::Float4.size(),
                            shader_location: 1,
                        },
                        wgpu::VertexAttribute {
                            format: wgpu::VertexFormat::Float4,
                            offset: wgpu::VertexFormat::Float4.size()
                                + wgpu::VertexFormat::Float2.size(),
                            shader_location: 2,
                        },
                    ],
                }],
            },
            fragment: Some(wgpu::FragmentState {
                module: &fs_module,
                entry_point: "main",
                targets: &[wgpu::ColorTargetState {
                    format: sc_desc.format,
                    color_blend: wgpu::BlendState {
                        src_factor: BlendFactor::SrcAlpha,
                        dst_factor: BlendFactor::OneMinusSrcAlpha,
                        operation: BlendOperation::Add,
                    },
                    alpha_blend: wgpu::BlendState {
                        src_factor: BlendFactor::One,
                        dst_factor: BlendFactor::One,
                        operation: BlendOperation::Min,
                    },
                    write_mask: wgpu::ColorWrite::ALL,
                }],
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Cw,
                cull_mode: CullMode::Back,
                polygon_mode: Default::default(),
            },
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: Default::default(),
                clamp_depth: false,
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
        });

        // Done
        Renderer {
            uniform_buffer,
            pipeline,
            sprites,
            depth_texture,
        }
    }

    pub(crate) fn resize(
        &mut self,
        _sc_desc: &wgpu::SwapChainDescriptor,
        _device: &wgpu::Device,
        _queue: &wgpu::Queue,
    ) {
    }

    pub(crate) fn render(
        &mut self,
        frame: &wgpu::SwapChainTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _sc_desc: &wgpu::SwapChainDescriptor,
        scene: Scene,
    ) {
        let mx_total = scene.camera.camera_uniform();

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::bytes_of(&mx_total));

        for sprite in self.sprites.iter_mut() {
            if let Some(instances) = scene.sprite_instances.get(&sprite.id) {
                sprite.update_instance_buffer(instances.clone(), queue);
            }
        }

        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: None,
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
                        load: wgpu::LoadOp::Clear(1.0),
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
