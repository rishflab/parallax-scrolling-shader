use crate::{renderer::Renderer, Game};
use winit::{
    dpi::LogicalSize,
    event::{self, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
};

pub const WINDOW_SIZE: LogicalSize<u32> = LogicalSize::new(1280, 720);

pub struct App {
    window: winit::window::Window,
    instance: wgpu::Instance,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface,
    adapter: wgpu::Adapter,
    device: wgpu::Device,
    queue: wgpu::Queue,
}

impl App {
    pub async fn new(title: &str, event_loop: &EventLoop<()>) -> App {
        let mut builder = winit::window::WindowBuilder::new();
        builder = builder
            .with_title(title)
            .with_inner_size(WINDOW_SIZE)
            .with_resizable(true);

        let window = builder.build(&event_loop).unwrap();

        log::info!("Initializing the surface...");

        let instance = wgpu::Instance::new(wgpu::BackendBit::PRIMARY);
        let (size, surface) = unsafe {
            let size = window.inner_size();
            let surface = instance.create_surface(&window);
            (size, surface)
        };

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::HighPerformance,
                compatible_surface: Some(&surface),
            })
            .await
            .unwrap();

        let optional_features = wgpu::Features::empty();
        let required_features = wgpu::Features::empty();
        let adapter_features = adapter.features();
        assert!(
            adapter_features.contains(required_features),
            "Adapter does not support required features for this example: {:?}",
            required_features - adapter_features
        );

        let needed_limits = wgpu::Limits::default();

        let trace_dir = std::env::var("WGPU_TRACE");
        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: wgpu::Label::None,
                    features: (optional_features & adapter_features) | required_features,
                    limits: needed_limits,
                },
                trace_dir.ok().as_ref().map(std::path::Path::new),
            )
            .await
            .unwrap();

        App {
            window,
            instance,
            size,
            surface,
            adapter,
            device,
            queue,
        }
    }

    pub fn run(mut self, event_loop: EventLoop<()>, mut game: Game<'static>) {
        let sc_desc = wgpu::SwapChainDescriptor {
            usage: wgpu::TextureUsage::RENDER_ATTACHMENT,
            width: self.size.width,
            height: self.size.height,
            present_mode: wgpu::PresentMode::Mailbox,
            // TODO: Allow srgb unconditionally
            format: wgpu::TextureFormat::Bgra8UnormSrgb,
        };
        let mut swap_chain = self.device.create_swap_chain(&self.surface, &sc_desc);

        let mut renderer = Renderer::init(&sc_desc, &mut self.device, &self.queue);

        log::info!("Entering render loop...");
        event_loop.run(move |event, _, control_flow| {
            let _ = (&self.instance, &self.adapter); // force ownership by the closure

            match event {
                event::Event::MainEventsCleared => {
                    self.window.request_redraw();
                }
                event::Event::WindowEvent { event, .. } => match event {
                    WindowEvent::KeyboardInput {
                        input:
                            event::KeyboardInput {
                                virtual_keycode: Some(event::VirtualKeyCode::Escape),
                                state: event::ElementState::Pressed,
                                ..
                            },
                        ..
                    }
                    | WindowEvent::CloseRequested => {
                        *control_flow = ControlFlow::Exit;
                    }
                    _ => game.capture_input(event),
                },
                event::Event::RedrawRequested(_) => {
                    let frame = match swap_chain.get_current_frame() {
                        Ok(frame) => frame,
                        Err(_) => {
                            swap_chain = self.device.create_swap_chain(&self.surface, &sc_desc);
                            swap_chain
                                .get_current_frame()
                                .expect("Failed to acquire next swap chain texture!")
                        }
                    };

                    let scene = game.run();

                    renderer.render(&frame.output, &self.device, &self.queue, &sc_desc, scene);
                }
                _ => (),
            }
        });
    }
}
