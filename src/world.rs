use crate::{
    instance::{Instance, InstanceRaw},
    time::Timer,
};
use cgmath::{Deg, Quaternion, Rotation3, Vector3};
use std::ops::Range;

pub struct World {
    pub cubes: Vec<Cube>,
    pub icospheres: Vec<Icosphere>,
    pub timer: Timer,
}

impl World {
    pub fn test() -> Self {
        let cube1 = Cube {
            position: cgmath::Vector3::new(0.0, 0.0, 0.0),
            rotation: Quaternion::from_axis_angle(Vector3::new(1.0, 0.0, 0.0), Deg(90.0)),
            angular_velocity: Quaternion::from_axis_angle(Vector3::new(0.0, 1.0, 0.0), Deg(1.0)),
            static_mesh_id: 0,
        };

        let cube2 = Cube {
            position: cgmath::Vector3::new(0.0, 8.0, 0.0),
            rotation: Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(0.0)),
            angular_velocity: Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(0.0)),
            static_mesh_id: 0,
        };

        let icosphere1 = Icosphere {
            position: cgmath::Vector3::new(-3.0, -1.0, 1.0),
            rotation: Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(0.0)),
            angular_velocity: Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(0.0)),
            static_mesh_id: 1,
        };

        let icosphere2 = Icosphere {
            position: cgmath::Vector3::new(3.0, 1.0, -1.0),
            rotation: Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(0.0)),
            angular_velocity: Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(-1.0)),
            static_mesh_id: 1,
        };

        World {
            cubes: vec![cube1, cube2],
            icospheres: vec![icosphere1, icosphere2],
            timer: Timer::new(),
        }
    }

    pub fn update(&mut self) {
        self.timer.tick();
        println!("fps: {:?}", self.timer.fps());
        for cube in self.cubes.iter_mut() {
            cube.update()
        }
        for icosphere in self.icospheres.iter_mut() {
            icosphere.update()
        }
    }

    pub fn cube_instance_data(&self) -> Vec<InstanceRaw> {
        self.cubes
            .iter()
            .map(|cube| cube.to_instance_raw())
            .collect::<Vec<InstanceRaw>>()
    }

    pub fn icosphere_instance_data(&self) -> Vec<InstanceRaw> {
        self.icospheres
            .iter()
            .map(|icosphere| icosphere.to_instance_raw())
            .collect::<Vec<InstanceRaw>>()
    }
    pub fn instance_data(&self) -> (Vec<InstanceRaw>, Vec<Range<u32>>) {
        let handles = [0, self.cubes.len(), self.icospheres.len()]
            .windows(2)
            .map(|w| Range {
                start: w[0] as u32,
                end: w[0] as u32 + w[1] as u32,
            })
            .collect();
        let mut vec = self.cube_instance_data();
        vec.append(&mut self.icosphere_instance_data());
        (vec, handles)
    }
}

#[derive(Copy, Clone)]
pub struct Cube {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub angular_velocity: cgmath::Quaternion<f32>,
    pub static_mesh_id: usize,
}

impl Cube {
    pub fn update(&mut self) {
        self.rotation = self.rotation * self.angular_velocity;
    }
    fn to_instance_raw(self) -> InstanceRaw {
        InstanceRaw::from(Instance {
            position: self.position,
            rotation: self.rotation,
        })
    }
}

#[derive(Copy, Clone)]
pub struct Icosphere {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub angular_velocity: cgmath::Quaternion<f32>,
    pub static_mesh_id: usize,
}

impl Icosphere {
    pub fn update(&mut self) {
        self.rotation = self.rotation * self.angular_velocity;
    }
    fn to_instance_raw(self) -> InstanceRaw {
        InstanceRaw::from(Instance {
            position: self.position,
            rotation: self.rotation,
        })
    }
}
