use crate::{
    asset::{Index, Vertex},
    instance::InstanceRaw,
};
use image::GenericImageView;
use std::{ops::Range, path::Path};
use wgpu::util::DeviceExt;

pub const MAX_INSTANCES: u64 = 1024;

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
        path: &Path,
        id: String,
    ) -> Self {
        let image = image::open(path).unwrap();
        let (tex_width, tex_height) = image.dimensions();
        let aspect_ratio = tex_width as f32 / tex_height as f32;

        let (vertex_data, index_data) = create_vertices(aspect_ratio);

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

        let texels = image::open(path).unwrap().to_rgba().to_vec();

        let texture_extent = wgpu::Extent3d {
            width: tex_width,
            height: tex_height,
            depth: 1,
        };
        let texture = device.create_texture(&wgpu::TextureDescriptor {
            label: None,
            size: texture_extent,
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Rgba8UnormSrgb,
            usage: wgpu::TextureUsage::SAMPLED | wgpu::TextureUsage::COPY_DST,
        });
        let texture_view = texture.create_view(&wgpu::TextureViewDescriptor::default());
        queue.write_texture(
            wgpu::TextureCopyView {
                texture: &texture,
                mip_level: 0,
                origin: wgpu::Origin3d::ZERO,
            },
            &texels,
            wgpu::TextureDataLayout {
                offset: 0,
                bytes_per_row: 4 * tex_width,
                rows_per_image: 0,
            },
            texture_extent,
        );

        // Create other resources
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            address_mode_u: wgpu::AddressMode::ClampToEdge,
            address_mode_v: wgpu::AddressMode::ClampToEdge,
            address_mode_w: wgpu::AddressMode::ClampToEdge,
            mag_filter: wgpu::FilterMode::Nearest,
            min_filter: wgpu::FilterMode::Linear,
            mipmap_filter: wgpu::FilterMode::Nearest,
            anisotropy_clamp: core::num::NonZeroU8::new(16),
            ..Default::default()
        });

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
                    resource: wgpu::BindingResource::Buffer(instance_buffer.slice(..)),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: wgpu::BindingResource::TextureView(&texture_view),
                },
                wgpu::BindGroupEntry {
                    binding: 3,
                    resource: wgpu::BindingResource::Sampler(&sampler),
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
        self.set_index_buffer(model.index_buffer.slice(..));
        self.set_bind_group(0, bind_group, &[]);
        self.draw_indexed(0..model.num_indices, 0, instances);
    }
}

fn create_vertices(_aspect_ratio: f32) -> (Vec<Vertex>, Vec<Index>) {
    let vertex_data = [
        Vertex {
            _pos: [-1.0, -1.0, 0.0, 1.0],
            _tex_coord: [0.0, 1.0],
        },
        Vertex {
            _pos: [1.0, -1.0, 0.0, 1.0],
            _tex_coord: [1.0, 1.0],
        },
        Vertex {
            _pos: [1.0, 1.0, 0.0, 1.0],
            _tex_coord: [1.0, 0.0],
        },
        Vertex {
            _pos: [-1.0, 1.0, 0.0, 1.0],
            _tex_coord: [0.0, 0.0],
        },
    ];

    let index_data: &[u16] = &[0, 1, 2, 2, 3, 0];

    (vertex_data.to_vec(), index_data.to_vec())
}
