use crate::{
    asset::{MeshData, StaticMesh, StaticMeshHandle, Vertex},
    camera::Camera,
    instance::InstanceRaw,
    model::{DrawModel, Model, MAX_INSTANCES},
    scene::Scene,
};
use wgpu::{util::DeviceExt, Buffer, BufferAddress, BufferDescriptor, MapMode};
use winit::event::{ElementState, KeyboardInput, VirtualKeyCode, WindowEvent};

pub(crate) struct Renderer {
    models: Vec<Model>,
    uniform_buffer: wgpu::Buffer,
    pipeline: wgpu::RenderPipeline,
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
            cgmath::Point3::new(0.0, -20.0, 3.0),
            cgmath::Point3::new(0f32, 0.0, 0.0),
        );

        let mx_total = camera.generate_matrix(sc_desc.width as f32 / sc_desc.height as f32);
        let mx_ref: &[f32; 16] = mx_total.as_ref();

        let uniform_buf_size = (bytemuck::cast_slice(mx_ref) as &[u8]).len() as u64;
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
                        multisampled: false,
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

        let icosphere_path = std::path::Path::new(&"assets/icosphere.gltf");
        let icosphere_mesh = StaticMesh::new(icosphere_path);

        let cube_path = std::path::Path::new(&"assets/cube.gltf");
        let cube_mesh = StaticMesh::new(cube_path);

        let cube_model = Model::new(
            device,
            queue,
            &bind_group_layout,
            &uniform_buffer_binding_resource,
            cube_mesh.vertex_data,
            cube_mesh.index_data,
        );

        let icosphere_model = Model::new(
            device,
            queue,
            &bind_group_layout,
            &uniform_buffer_binding_resource,
            icosphere_mesh.vertex_data,
            icosphere_mesh.index_data,
        );

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
            rasterization_state: Some(wgpu::RasterizationStateDescriptor {
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: wgpu::CullMode::Back,
                ..Default::default()
            }),
            primitive_topology: wgpu::PrimitiveTopology::TriangleList,
            color_states: &[wgpu::ColorStateDescriptor {
                format: sc_desc.format,
                color_blend: wgpu::BlendDescriptor::REPLACE,
                alpha_blend: wgpu::BlendDescriptor::REPLACE,
                write_mask: wgpu::ColorWrite::ALL,
            }],
            depth_stencil_state: None,
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
            models: vec![cube_model, icosphere_model],
        }
    }

    pub(crate) fn input(&mut self, event: winit::event::WindowEvent) {
        if let WindowEvent::KeyboardInput { input, .. } = event {
            match input {
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Left),
                    ..
                } => {
                    self.camera.eye += cgmath::vec3(0.1, 0.0, 0.0);
                    self.camera.look_at += cgmath::vec3(0.1, 0.0, 0.0);
                }
                KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Right),
                    ..
                } => {
                    self.camera.eye += cgmath::vec3(-0.1, 0.0, 0.0);
                    self.camera.look_at += cgmath::vec3(-0.1, 0.0, 0.0);
                }
                _ => (),
            }
        }
    }

    pub(crate) fn resize(
        &mut self,
        sc_desc: &wgpu::SwapChainDescriptor,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
    ) {
        let mx_total = self
            .camera
            .generate_matrix(sc_desc.width as f32 / sc_desc.height as f32);
        let mx_ref: &[f32; 16] = mx_total.as_ref();
        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mx_ref));
    }

    pub(crate) fn update(
        &mut self,
        queue: &wgpu::Queue,
        sc_desc: &wgpu::SwapChainDescriptor,
        scene: Scene,
    ) {
        let mx_total = self
            .camera
            .generate_matrix(sc_desc.width as f32 / sc_desc.height as f32);
        let mx_ref: &[f32; 16] = mx_total.as_ref();

        queue.write_buffer(&self.uniform_buffer, 0, bytemuck::cast_slice(mx_ref));

        for i in 0..2 {
            self.models[i].update_instance_buffer(scene.instanced_draws[i].clone(), queue);
        }
        // queue.write_buffer(
        //     &self.instance_buffer,
        //     0,
        //     bytemuck::cast_slice(&instance_data),
        // );
    }

    pub(crate) fn render(
        &mut self,
        frame: &wgpu::SwapChainTexture,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _spawner: &impl futures::task::LocalSpawn,
        _sc_desc: &wgpu::SwapChainDescriptor,
    ) {
        let mut encoder =
            device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        {
            let mut rpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                color_attachments: &[wgpu::RenderPassColorAttachmentDescriptor {
                    attachment: &frame.view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.1,
                            g: 0.2,
                            b: 0.3,
                            a: 1.0,
                        }),
                        store: true,
                    },
                }],
                depth_stencil_attachment: None,
            });

            rpass.set_pipeline(&self.pipeline);

            rpass.draw_model(&self.models[0], 0..2, &self.models[0].bind_group);
            rpass.draw_model(&self.models[1], 0..2, &self.models[1].bind_group);
        }

        queue.submit(Some(encoder.finish()));
    }
}
