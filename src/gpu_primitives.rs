use bytemuck::{Pod, Zeroable};

pub type Index = u16;

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub _pos: [f32; 4],
    pub _tex_coord: [f32; 2],
}

unsafe impl Pod for Vertex {}
unsafe impl Zeroable for Vertex {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct CameraUniform {
    pub ortho: [f32; 16],
    pub persp: [f32; 16],
}

unsafe impl Pod for CameraUniform {}
unsafe impl Zeroable for CameraUniform {}

pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
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
