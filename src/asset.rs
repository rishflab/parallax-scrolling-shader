use bytemuck::{Pod, Zeroable};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub(crate) _pos: [f32; 4],
    pub(crate) _tex_coord: [f32; 2],
}

pub type Index = u16;

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}
