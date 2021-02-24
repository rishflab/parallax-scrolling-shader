use crate::{
    camera::ParallaxCamera,
    gpu_primitives::{Instance, InstanceRaw},
    scene::Scene,
    time::Timer,
};
use cgmath::{Deg, Quaternion, Rotation3, Vector3};
use glam::Vec3;
use hecs::*;
use std::collections::HashMap;
use winit::event::{ElementState, VirtualKeyCode, WindowEvent};

pub struct Position(Vector3<f32>);
pub struct Rotation(Quaternion<f32>);
pub struct Scale(u8);
pub struct Sprite(String);
pub struct KeyboardInput(pub Option<winit::event::KeyboardInput>);

pub struct Game {
    world: World,
    timer: Timer,
    pub camera: ParallaxCamera,
}

impl Game {
    pub(crate) fn new() -> Game {
        let mut world = World::new();

        let player = (
            Position(cgmath::Vector3::new(0.0, 0.0, 20.0)),
            Rotation(Quaternion::from_axis_angle(
                Vector3::new(0.0, 1.0, 0.0),
                Deg(0.0),
            )),
            Scale(1),
            KeyboardInput(None),
            Sprite("player".to_string()),
        );

        let apple = (
            Position(cgmath::Vector3::new(-2.0, 0.0, 30.0)),
            Rotation(Quaternion::from_axis_angle(
                Vector3::new(0.0, 1.0, 0.0),
                Deg(0.0),
            )),
            Scale(1),
            Sprite("apple".to_string()),
        );

        let ashberry = (
            Position(cgmath::Vector3::new(2.0, 0.0, 30.0)),
            Rotation(Quaternion::from_axis_angle(
                Vector3::new(0.0, 1.0, 0.0),
                Deg(0.0),
            )),
            Scale(1),
            Sprite("ashberry".to_string()),
        );

        let baobab = (
            Position(cgmath::Vector3::new(3.0, 0.0, 55.0)),
            Rotation(Quaternion::from_axis_angle(
                Vector3::new(0.0, 1.0, 0.0),
                Deg(0.0),
            )),
            Scale(1),
            Sprite("baobab".to_string()),
        );

        let beech = (
            Position(cgmath::Vector3::new(-3.5, 0.0, 95.0)),
            Rotation(Quaternion::from_axis_angle(
                Vector3::new(0.0, 1.0, 0.0),
                Deg(0.0),
            )),
            Scale(1),
            Sprite("beech".to_string()),
        );

        world.spawn(player);
        world.spawn(apple);
        world.spawn(ashberry);
        world.spawn(baobab);
        world.spawn(beech);

        let camera = ParallaxCamera::new(
            glam::Vec3::new(0.0, 3.0, 0.0),
            glam::Vec3::new(0.0, 0.0, 1.0),
            1.0,
            1.0,
            500.0,
        );

        Game {
            world,
            timer: Timer::new(),
            camera,
        }
    }

    fn move_player(&mut self) {
        let mut q = self.world.query::<(&KeyboardInput, &mut Position)>();
        let move_speed = 0.1;
        for (_, (key, pos)) in q.iter() {
            // ignore non keyboard input
            if let Some(input) = key.0 {
                match input {
                    winit::event::KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Left),
                        ..
                    } => {
                        pos.0 -= Vector3::new(move_speed, 0.0, 0.0);
                        self.camera.eye -= Vec3::new(move_speed, 0.0, 0.0);
                    }
                    winit::event::KeyboardInput {
                        state: ElementState::Pressed,
                        virtual_keycode: Some(VirtualKeyCode::Right),
                        ..
                    } => {
                        pos.0 += Vector3::new(move_speed, 0.0, 0.0);
                        self.camera.eye += Vec3::new(move_speed, 0.0, 0.0);
                    }
                    _ => (),
                }
            }
        }
    }

    fn build_scene(&mut self) -> Scene {
        let mut sprites: HashMap<String, Vec<InstanceRaw>> = HashMap::default();

        for (_, (pos, rot, scale, sprite_id)) in
            &mut self
                .world
                .query::<(&Position, &Rotation, &Scale, &Sprite)>()
        {
            let instance_raw = InstanceRaw::from(Instance {
                position: pos.0,
                rotation: rot.0,
                scale: scale.0 as f32,
            });
            if let Some(instances) = sprites.get(&sprite_id.0) {
                let mut new = instances.clone();
                new.push(instance_raw);
                sprites.insert(sprite_id.0.clone(), new);
            } else {
                sprites.insert(sprite_id.0.clone(), vec![instance_raw]);
            }
        }

        Scene {
            sprite_instances: sprites,
            camera: self.camera,
        }
    }

    pub fn capture_input(&self, event: winit::event::WindowEvent) {
        let mut q = self.world.query::<&mut KeyboardInput>();
        for (_, mut key) in q.iter() {
            // ignore non keyboard input
            if let WindowEvent::KeyboardInput { input, .. } = event {
                key.0 = Some(input);
            } else {
                key.0 = None;
            }
        }
    }

    pub fn run(&mut self) -> Scene {
        self.timer.tick();
        self.move_player();
        self.build_scene()
    }
}
