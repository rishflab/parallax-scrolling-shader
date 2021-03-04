#![feature(in_band_lifetimes)]

use crate::{
    camera::{ActiveCamera, Camera, ParallaxCamera},
    gpu_primitives::{Instance, InstanceRaw},
    scene::Scene,
    time::Timer,
};
use cgmath::{Quaternion, Vector3};
use hecs::{DynamicBundle, Entity, World};
use std::{
    collections::HashMap,
    time::{Duration, Instant},
};
use winit::event::WindowEvent;

pub mod app;
pub mod asset;
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
pub struct KeyboardInput(pub Option<winit::event::KeyboardInput>);

pub struct Sprite {
    id: String,
    pub frame_id: u32,
}

impl Sprite {
    pub fn new(id: &str) -> Self {
        Self {
            id: id.to_string(),
            frame_id: 0,
        }
    }
}

pub struct Game<'a> {
    world: World,
    timer: Timer,
    systems: Vec<&'a dyn Fn(&World, Duration, Instant)>,
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
            system(&self.world, self.timer.elapsed(), self.timer.now())
        }
        self.build_scene()
    }
    pub fn spawn_entity(&mut self, components: impl DynamicBundle) -> Entity {
        self.world.spawn(components)
    }
    pub fn add_system(&mut self, system: &'a dyn Fn(&World, Duration, Instant)) {
        self.systems.push(system)
    }
    fn build_scene(&mut self) -> Scene {
        let mut sprites: HashMap<String, Vec<InstanceRaw>> = HashMap::default();

        for (_, (pos, rot, scale, sprite)) in &mut self
            .world
            .query::<(&Position, &Rotation, &Scale, &Sprite)>()
        {
            let instance_raw = InstanceRaw::from(Instance {
                position: pos.0,
                rotation: rot.0,
                scale: scale.0 as f32,
                frame_id: sprite.frame_id,
            });
            if let Some(instances) = sprites.get(&sprite.id) {
                // TODO: try and remove these clones
                let mut new = instances.clone();
                new.push(instance_raw);
                sprites.insert(sprite.id.clone(), new);
            } else {
                sprites.insert(sprite.id.clone(), vec![instance_raw]);
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
