#![allow(clippy::single_match)]
#![feature(or_patterns)]
extern crate erlking;

use cgmath::{Deg, Quaternion, Rotation3, Vector3};
use erlking::{
    asset::SpriteAsset,
    camera::{ActiveCamera, ParallaxCamera},
    App, Game, KeyboardInput, Position, Rotation, Scale, Sprite,
};
use glam::Vec3;
use hecs::World;
use std::time::{Duration, Instant};
use winit::{
    event::{ElementState, VirtualKeyCode},
    event_loop::EventLoop,
};

#[derive(Clone, Copy)]
struct MoveSpeed(f32);

enum PlayerState {
    Idle,
    Walk(Instant),
}

impl PlayerState {
    pub fn animation_state(&self, now: Instant) -> u32 {
        match self {
            Self::Idle => 0,
            Self::Walk(start) => {
                let animation = vec![(2, 0.2), (1, 0.0)];
                let dt = now - *start;
                let dt = dt.as_secs_f32() % 0.4;
                let mut frame = 0;
                for (f, time) in animation {
                    if dt > time {
                        frame = f;
                        break;
                    }
                }
                frame
            }
        }
    }
}

fn main() {
    let event_loop = EventLoop::new();
    let app = futures::executor::block_on(App::new("parallax-demo", &event_loop));
    let mut parallax_demo = Game::new();

    let sprite_assets = vec![
        SpriteAsset::new("player", vec![
            "assets/adventurer_idle.png",
            "assets/adventurer_walk1.png",
            "assets/adventurer_walk2.png",
        ]),
        SpriteAsset::new("apple", vec!["assets/apple.png"]),
        SpriteAsset::new("ashberry", vec!["assets/ashberry.png"]),
        SpriteAsset::new("baobab", vec!["assets/baobab.png"]),
        SpriteAsset::new("beech", vec!["assets/beech.png"]),
    ];

    let movespeed = MoveSpeed(10.0);

    let player = (
        Position(cgmath::Vector3::new(0.0, 0.0, 20.0)),
        Rotation(Quaternion::from_axis_angle(
            Vector3::new(0.0, 1.0, 0.0),
            Deg(0.0),
        )),
        Scale(1),
        KeyboardInput(None),
        Sprite {
            id: "player".to_string(),
            frame_id: 0,
        },
        PlayerState::Idle,
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
        Sprite {
            id: "apple".to_string(),
            frame_id: 0,
        },
    );

    let ashberry = (
        Position(cgmath::Vector3::new(2.0, 0.0, 30.0)),
        Rotation(Quaternion::from_axis_angle(
            Vector3::new(0.0, 1.0, 0.0),
            Deg(0.0),
        )),
        Scale(1),
        Sprite {
            id: "ashberry".to_string(),
            frame_id: 0,
        },
    );

    let baobab = (
        Position(cgmath::Vector3::new(3.0, 0.0, 55.0)),
        Rotation(Quaternion::from_axis_angle(
            Vector3::new(0.0, 1.0, 0.0),
            Deg(0.0),
        )),
        Scale(1),
        Sprite {
            id: "baobab".to_string(),
            frame_id: 0,
        },
    );

    let beech = (
        Position(cgmath::Vector3::new(-3.5, 0.0, 95.0)),
        Rotation(Quaternion::from_axis_angle(
            Vector3::new(0.0, 1.0, 0.0),
            Deg(0.0),
        )),
        Scale(1),
        Sprite {
            id: "beech".to_string(),
            frame_id: 0,
        },
    );

    parallax_demo.spawn_entity(player);
    parallax_demo.spawn_entity(apple);
    parallax_demo.spawn_entity(ashberry);
    parallax_demo.spawn_entity(baobab);
    parallax_demo.spawn_entity(beech);
    parallax_demo.spawn_entity(camera);

    parallax_demo.add_system(&move_player);
    parallax_demo.add_system(&move_camera);
    parallax_demo.add_system(&update_animation_state);

    app.run(event_loop, parallax_demo, sprite_assets);
}

fn move_player(world: &World, dt: Duration, instant: Instant) {
    let mut q = world.query::<(&KeyboardInput, &mut Position, &MoveSpeed, &mut PlayerState)>();

    for (_, (key, pos, speed, state)) in q.iter() {
        if let Some(input) = key.0 {
            let dx = Vector3::new(speed.0 * dt.as_secs_f32(), 0.0, 0.0);
            match input {
                winit::event::KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Left),
                    ..
                } => {
                    match state {
                        PlayerState::Idle => *state = PlayerState::Walk(instant),
                        _ => (),
                    }
                    pos.0 -= dx;
                }
                winit::event::KeyboardInput {
                    state: ElementState::Pressed,
                    virtual_keycode: Some(VirtualKeyCode::Right),
                    ..
                } => {
                    match state {
                        PlayerState::Idle => *state = PlayerState::Walk(instant),
                        _ => (),
                    }
                    pos.0 += dx;
                }
                _ => *state = PlayerState::Idle,
            }
        } else {
            *state = PlayerState::Idle;
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

fn update_animation_state(world: &World, _dt: Duration, instant: Instant) {
    let mut q = world.query::<(&PlayerState, &mut Sprite)>();

    for (_, (state, sprite)) in q.iter() {
        sprite.frame_id = state.animation_state(instant);
    }
}
