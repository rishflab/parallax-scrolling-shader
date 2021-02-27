extern crate erlking;

use cgmath::{Deg, Quaternion, Rotation3, Vector3};
use erlking::{
    camera::{ActiveCamera, ParallaxCamera},
    App, Game, KeyboardInput, Position, Rotation, Scale, Sprite,
};
use glam::Vec3;
use hecs::World;
use std::time::Duration;
use winit::{
    event::{ElementState, VirtualKeyCode},
    event_loop::EventLoop,
};

#[derive(Clone, Copy)]
struct MoveSpeed(f32);

fn main() {
    let event_loop = EventLoop::new();
    let app = futures::executor::block_on(App::new("parallax-demo", &event_loop));
    let mut parallax_demo = Game::new();

    let movespeed = MoveSpeed(10.0);

    let player = (
        Position(cgmath::Vector3::new(0.0, 0.0, 20.0)),
        Rotation(Quaternion::from_axis_angle(
            Vector3::new(0.0, 1.0, 0.0),
            Deg(0.0),
        )),
        Scale(1),
        KeyboardInput(None),
        Sprite("player".to_string()),
        movespeed,
    );

    let camera = (
        ParallaxCamera::new(
            Vec3::new(0.0, 3.0, 0.0),
            Vec3::new(0.0, 0.0, 1.0),
            1.0,
            0.1,
            500.0,
        ),
        KeyboardInput(None),
        ActiveCamera,
        movespeed,
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
    parallax_demo.spawn_entity(player);
    parallax_demo.spawn_entity(apple);
    parallax_demo.spawn_entity(ashberry);
    parallax_demo.spawn_entity(baobab);
    parallax_demo.spawn_entity(beech);
    parallax_demo.spawn_entity(camera);

    parallax_demo.add_system(&move_player);

    app.run(event_loop, parallax_demo);
}

fn move_player(world: &World, dt: Duration) {
    let mut q = world.query::<(&KeyboardInput, &mut Position, &MoveSpeed)>();

    for (_, (key, pos, speed)) in q.iter() {
        if let Some(input) = key.0 {
            let dx = Vector3::new(speed.0 * dt.as_secs_f32(), 0.0, 0.0);
            match input {
                winit::event::KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Left),
                    ..
                } => {
                    pos.0 -= dx;
                }
                winit::event::KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Right),
                    ..
                } => {
                    pos.0 += dx;
                }
                _ => (),
            }
        }
    }
    let mut q = world.query::<(
        &ActiveCamera,
        &mut ParallaxCamera,
        &KeyboardInput,
        &MoveSpeed,
    )>();

    let (_, (_, cam, key, speed)) = q.iter().next().expect("active camera is preset");
    if let Some(input) = key.0 {
        let dx = Vec3::new(speed.0 * dt.as_secs_f32(), 0.0, 0.0);
        match input {
            winit::event::KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Left),
                ..
            } => {
                cam.eye -= dx;
            }
            winit::event::KeyboardInput {
                state: ElementState::Pressed,
                virtual_keycode: Some(VirtualKeyCode::Right),
                ..
            } => {
                cam.eye += dx;
            }
            _ => (),
        }
    }
}
