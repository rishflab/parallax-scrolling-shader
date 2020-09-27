use crate::{
    instance::{Instance, InstanceRaw},
    scene::Scene,
    time::Timer,
};
use cgmath::{Deg, Quaternion, Rotation3, Vector3};
use hecs::*;
use std::ops::Range;

pub struct Position(Vector3<f32>);
pub struct Rotation(Quaternion<f32>);
pub struct AngularVelocity(Quaternion<f32>);
pub struct StaticMesh(usize);

pub struct Game {
    world: World,
    timer: Timer,
}

impl Game {
    pub(crate) fn new() -> Game {
        let mut world = World::new();
        let cube1 = (
            Position(cgmath::Vector3::new(0.0, 0.0, 0.0)),
            Rotation(Quaternion::from_axis_angle(
                Vector3::new(1.0, 0.0, 0.0),
                Deg(90.0),
            )),
            AngularVelocity(Quaternion::from_axis_angle(
                Vector3::new(0.0, 1.0, 0.0),
                Deg(1.0),
            )),
            StaticMesh(0),
        );

        let cube2 = (
            Position(cgmath::Vector3::new(0.0, 0.0, 0.0)),
            Rotation(Quaternion::from_axis_angle(
                Vector3::new(0.0, 0.0, 0.0),
                Deg(0.0),
            )),
            AngularVelocity(Quaternion::from_axis_angle(
                Vector3::new(0.0, 1.0, 1.0),
                Deg(0.0),
            )),
            StaticMesh(0),
        );

        let icosphere1 = (
            Position(cgmath::Vector3::new(-3.0, -1.0, 0.0)),
            Rotation(Quaternion::from_axis_angle(
                Vector3::new(0.0, 0.0, 1.0),
                Deg(0.0),
            )),
            AngularVelocity(Quaternion::from_axis_angle(
                Vector3::new(0.0, 1.0, 1.0),
                Deg(0.0),
            )),
            StaticMesh(1),
        );

        let icosphere2 = (
            Position(cgmath::Vector3::new(3.0, 1.0, -1.0)),
            Rotation(Quaternion::from_axis_angle(
                Vector3::new(0.0, 0.0, 1.0),
                Deg(0.0),
            )),
            AngularVelocity(Quaternion::from_axis_angle(
                Vector3::new(0.0, 1.0, 1.0),
                Deg(-1.0),
            )),
            StaticMesh(1),
        );

        world.spawn(cube1);
        world.spawn(cube2);
        world.spawn(icosphere1);
        world.spawn(icosphere2);

        Game {
            world,
            timer: Timer::new(),
        }
    }

    fn rotate_objects(&self) {
        for (_, (rot, ang_vel)) in &mut self.world.query::<(&mut Rotation, &AngularVelocity)>() {
            rot.0 = rot.0 * ang_vel.0;
        }
    }

    fn build_scene(&self) -> Scene {
        let mut instanced_draws: Vec<Vec<InstanceRaw>> = vec![];

        for (_, (pos, rot, mesh_id)) in
            &mut self.world.query::<(&Position, &Rotation, &StaticMesh)>()
        {
            let instance_raw = InstanceRaw::from(Instance {
                position: pos.0,
                rotation: rot.0,
            });
            if let Some(_) = instanced_draws.get(mesh_id.0) {
                instanced_draws[mesh_id.0].push(instance_raw)
            } else {
                instanced_draws.insert(mesh_id.0, vec![instance_raw])
            }
        }
        Scene { instanced_draws }
    }

    pub fn run(&mut self) -> Scene {
        self.timer.tick();
        println!("fps: {}", self.timer.fps());
        self.rotate_objects();
        self.build_scene()
    }
}
