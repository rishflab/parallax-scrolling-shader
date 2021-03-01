#![feature(in_band_lifetimes)]

use crate::{
    camera::{ActiveCamera, Camera, ParallaxCamera},
    gpu_primitives::{Instance, InstanceRaw},
    scene::Scene,
    time::Timer,
};
use cgmath::{Quaternion, Vector3};
use hecs::{DynamicBundle, Entity, World};
use std::{collections::HashMap, time::Duration};
use winit::event::WindowEvent;

mod app;
pub mod camera;
mod gpu_primitives;
mod renderer;
mod scene;
mod sprite;
mod texture;
mod time;

pub use app::App;

pub struct Position(pub Vector3<f32>);
pub struct Rotation(pub Quaternion<f32>);
pub struct Scale(pub u8);
pub struct Sprite(pub String);
pub struct KeyboardInput(pub Option<winit::event::KeyboardInput>);

pub struct Game<'a> {
    world: World,
    timer: Timer,
    systems: Vec<&'a dyn Fn(&World, Duration)>,
}

impl<'a> Game<'a> {
    pub fn new() -> Game<'a> {
        Game {
            world: Default::default(),
            timer: Default::default(),
            systems: vec![],
        }
    }
    fn run(&mut self) -> Scene {
        self.timer.tick();
        for system in self.systems.iter() {
            system(&self.world, self.timer.elapsed())
        }
        self.build_scene()
    }
    pub fn spawn_entity(&mut self, components: impl DynamicBundle) -> Entity {
        self.world.spawn(components)
    }
    pub fn add_system(&mut self, system: &'a dyn Fn(&World, Duration)) {
        self.systems.push(system)
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

        let mut q = self.world.query::<(&ActiveCamera, &ParallaxCamera)>();

        let (_, (_, cam)) = q.iter().next().expect("No camera defined");

        Scene {
            sprite_instances: sprites,
            camera_uniform: cam.generate_matrix(),
        }
    }
    fn capture_input(&self, event: winit::event::WindowEvent) {
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
}

impl<'a> Default for Game<'a> {
    fn default() -> Self {
        Self::new()
    }
}
