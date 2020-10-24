use crate::{camera::Camera, gpu_primitives::InstanceRaw};
use std::collections::HashMap;

pub struct Scene {
    pub sprite_instances: HashMap<String, Vec<InstanceRaw>>,
    pub camera: Camera,
}
