use crate::{
    instance::{Instance, InstanceRaw},
    time::Timer,
};
use cgmath::{Deg, Quaternion, Rotation3, Vector3};
use std::ops::Range;

pub struct Scene {
    pub instanced_draws: Vec<Vec<InstanceRaw>>,
}
