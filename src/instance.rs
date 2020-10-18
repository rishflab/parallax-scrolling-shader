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
