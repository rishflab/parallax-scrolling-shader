use crate::asset::Vertex;
use std::ops::Range;
use wgpu::util::DeviceExt;

pub struct Model {
    pub(crate) vertex_buffer: wgpu::Buffer,
    pub(crate) index_buffer: wgpu::Buffer,
    num_indices: u32,
}

impl Model {
    pub fn new(device: &mut wgpu::Device, vertex_data: Vec<Vertex>, index_data: Vec<u16>) -> Self {
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

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: index_data.len() as u32,
        }
    }
}

pub trait DrawModel<'a, 'b>
where
    'b: 'a,
{
    fn draw_model(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
        bind_group: &'b wgpu::BindGroup,
    );
}

impl<'a, 'b> DrawModel<'a, 'b> for wgpu::RenderPass<'a>
where
    'b: 'a,
{
    fn draw_model(
        &mut self,
        model: &'b Model,
        instances: Range<u32>,
        bind_group: &'b wgpu::BindGroup,
    ) {
        self.set_vertex_buffer(0, model.vertex_buffer.slice(..));
        self.set_index_buffer(model.index_buffer.slice(..));
        self.set_bind_group(0, bind_group, &[]);
        self.draw_indexed(0..model.num_indices, 0, instances);
    }
}
