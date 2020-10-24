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

pub(crate) struct Instance {
    pub(crate) position: cgmath::Vector3<f32>,
    pub(crate) rotation: cgmath::Quaternion<f32>,
    pub scale: f32,
}

#[repr(C)]
#[derive(Copy, Clone)]
pub struct InstanceRaw {
    model: cgmath::Matrix4<f32>,
}

impl From<Instance> for InstanceRaw {
    fn from(from: Instance) -> Self {
        InstanceRaw {
            model: cgmath::Matrix4::from_translation(from.position)
                * cgmath::Matrix4::from(from.rotation)
                * cgmath::Matrix4::from_scale(from.scale),
        }
    }
}

unsafe impl bytemuck::Pod for InstanceRaw {}
unsafe impl bytemuck::Zeroable for InstanceRaw {}
