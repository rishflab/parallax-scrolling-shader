use crate::renderer::{Instance, InstanceRaw};
use cgmath::{Deg, Quaternion, Rotation3, Vector3};

pub struct World {
    cubes: Vec<Cube>,
}

impl World {
    pub fn two_cubes() -> Self {
        let cube1 = Cube {
            position: cgmath::Vector3::new(1.0, 0.0, 0.0),
            rotation: Quaternion::from_axis_angle(Vector3::new(1.0, 0.0, 0.0), Deg(90.0)),
            angular_velocity: Quaternion::from_axis_angle(Vector3::new(0.0, 1.0, 0.0), Deg(1.0)),
        };

        let cube2 = Cube {
            position: cgmath::Vector3::new(-1.0, 0.0, 0.0),
            rotation: Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(0.0)),
            angular_velocity: Quaternion::from_axis_angle(Vector3::new(0.0, 0.0, 1.0), Deg(1.0)),
        };
        World {
            cubes: vec![cube1, cube2],
        }
    }

    pub fn update(&mut self) {
        for cube in self.cubes.iter_mut() {
            cube.update()
        }
    }

    pub fn cube_instance_data(&self) -> Vec<InstanceRaw> {
        self.cubes
            .iter()
            .map(|cube| cube.to_instance_raw())
            .collect::<Vec<InstanceRaw>>()
    }
    pub fn cube_instance_data_len(&self) -> usize {
        self.cube_instance_data().len() * std::mem::size_of::<InstanceRaw>()
    }
}

#[derive(Copy, Clone)]
pub struct Cube {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
    pub angular_velocity: cgmath::Quaternion<f32>,
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
