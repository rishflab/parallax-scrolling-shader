use crate::{
    gpu_primitives::{Index, InstanceRaw, Vertex},
    texture::Texture,
};
use image::GenericImageView;
use std::{ops::Range, path::Path};
use wgpu::util::DeviceExt;

pub const MAX_INSTANCES: u64 = 1024;
pub const PIXELS_PER_METRE: u32 = 32;

pub struct Sprite {
    pub id: String,
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    pub instance_buffer: wgpu::Buffer,
    pub bind_group: wgpu::BindGroup,
    num_indices: u32,
}

impl Sprite {
    pub fn new(
        device: &mut wgpu::Device,
        queue: &wgpu::Queue,
        bind_group_layout: &wgpu::BindGroupLayout,
        uniform_buffer_binding_resource: &wgpu::BindingResource,
        path: impl AsRef<Path>,
        id: String,
    ) -> Self {
        let image = image::open(path).unwrap();
        let (tex_width, tex_height) = image.dimensions();

        let (vertex_data, index_data) = create_vertices(tex_width, tex_height, PIXELS_PER_METRE);

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertex_data),
            usage: wgpu::BufferUsage::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&index_data),
            usage: wgpu::BufferUsage::INDEX,
        });

        let instance_buf_size = MAX_INSTANCES * std::mem::size_of::<InstanceRaw>() as u64;
        let instance_buffer = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniform Buffer"),
            usage: wgpu::BufferUsage::STORAGE | wgpu::BufferUsage::COPY_DST,
            size: instance_buf_size,
            mapped_at_creation: false,
        });

        let texture = Texture::create_sprite_texture(&device, &queue, image.into_rgba());

        // Create bind group
        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform_buffer_binding_resource.clone(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: instance_buffer.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&texture.sampler),
                },
            ],
            label: None,
        });

        Self {
            vertex_buffer,
            index_buffer,
            instance_buffer,
            bind_group,
            num_indices: index_data.len() as u32,
            id,
        }
    }

    pub fn update_instance_buffer(&mut self, instances: Vec<InstanceRaw>, queue: &wgpu::Queue) {
        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(instances.as_slice()),
        );
    }
}

pub trait DrawSprite<'a, 'b>
where
    'b: 'a,
{
    fn draw_sprite(
        &mut self,
        model: &'b Sprite,
        instances: Range<u32>,
        bind_group: &'b wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawSprite<'a, 'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_sprite(
        &mut self,
        model: &'b Sprite,
        instances: Range<u32>,
        bind_group: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, model.vertex_buffer.slice(..));
        self.set_index_buffer(model.index_buffer.slice(..), wgpu::IndexFormat::Uint16);
        self.set_bind_group(0, bind_group, &[]);
        self.draw_indexed(0..model.num_indices, 0, instances);
    }
}
fn create_vertices(width: u32, height: u32, pixel_per_metre: u32) -> (Vec<Vertex>, Vec<Index>) {
    let w = (width as f32 / pixel_per_metre as f32) / 2.0;
    let h = (height as f32 / pixel_per_metre as f32) / 2.0;
    let vertex_data = [
        Vertex {
            _pos: [-w, -h, 0.0, 1.0],
            _tex_coord: [0.0, 1.0],
        },
        Vertex {
            _pos: [w, -h, 0.0, 1.0],
            _tex_coord: [1.0, 1.0],
        },
        Vertex {
            _pos: [w, h, 0.0, 1.0],
            _tex_coord: [1.0, 0.0],
        },
        Vertex {
            _pos: [-w, h, 0.0, 1.0],
            _tex_coord: [0.0, 0.0],
        },
    ];

    let index_data: &[u16] = &[0, 1, 2, 2, 3, 0];

    (vertex_data.to_vec(), index_data.to_vec())
}
