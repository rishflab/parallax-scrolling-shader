use crate::instance::InstanceRaw;
use std::collections::HashMap;

pub struct Scene {
    pub sprite_instances: HashMap<String, Vec<InstanceRaw>>,
}
