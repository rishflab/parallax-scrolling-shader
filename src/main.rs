#![allow(clippy::single_match)]
#![feature(or_patterns)]
extern crate parallax_scrolling_shader_demo;

use glam::{Quat, Vec3};
use hecs::World;
use parallax_scrolling_shader_demo::{
    asset::SpriteAsset,
    camera::{ActiveCamera, ParallaxCamera},
    App, Game, KeyboardInput, Position, Rotation, Scale, Sprite,
};
use std::time::{Duration, Instant};
use winit::{
    event::{ElementState, VirtualKeyCode},
    event_loop::EventLoop,
};

#[derive(Clone, Copy)]
struct MoveSpeed(f32);

fn main() {
    let event_loop = EventLoop::new();
    let app = futures::executor::block_on(App::new("parallax-scrolling-shader-demo", &event_loop));
    let mut parallax_demo = Game::new();

    let sprite_assets = vec![
        SpriteAsset::new("player", vec!["assets/player.png"]),
        SpriteAsset::new("apple", vec!["assets/apple.png"]),
        SpriteAsset::new("ashberry", vec!["assets/ashberry.png"]),
        SpriteAsset::new("baobab", vec!["assets/baobab.png"]),
        SpriteAsset::new("beech", vec!["assets/beech.png"]),
    ];

    let movespeed = MoveSpeed(10.0);

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

    let player = (
        Position(Vec3::new(0.0, 0.0, 20.0)),
        Rotation(Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 0.0)),
        Scale(1),
        KeyboardInput(None),
        Sprite::new("player"),
        movespeed,
    );

    let apple = (
        Position(Vec3::new(-2.0, 0.0, 30.0)),
        Rotation(Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 0.0)),
        Scale(1),
        Sprite::new("apple"),
    );

    let ashberry = (
        Position(Vec3::new(2.0, 0.0, 30.0)),
        Rotation(Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 0.0)),
        Scale(1),
        Sprite::new("ashberry"),
    );

    let baobab = (
        Position(Vec3::new(3.0, 0.0, 55.0)),
        Rotation(Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 0.0)),
        Scale(1),
        Sprite::new("baobab"),
    );

    let beech = (
        Position(Vec3::new(-3.5, 0.0, 95.0)),
        Rotation(Quat::from_axis_angle(Vec3::new(0.0, 1.0, 0.0), 0.0)),
        Scale(1),
        Sprite::new("beech"),
    );

    parallax_demo.spawn_entity(player);
    parallax_demo.spawn_entity(apple);
    parallax_demo.spawn_entity(ashberry);
    parallax_demo.spawn_entity(baobab);
    parallax_demo.spawn_entity(beech);
    parallax_demo.spawn_entity(camera);

    parallax_demo.add_system(&move_player);
    parallax_demo.add_system(&move_camera);

    app.run(event_loop, parallax_demo, sprite_assets);
}

fn move_player(world: &World, dt: Duration, _instant: Instant) {
    let mut q = world.query::<(&KeyboardInput, &mut Position, &MoveSpeed)>();

    for (_, (key, pos, speed)) in q.iter() {
        if let Some(input) = key.0 {
            let dx = Vec3::new(speed.0 * dt.as_secs_f32(), 0.0, 0.0);
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
}

fn move_camera(world: &World, dt: Duration, _instant: Instant) {
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
