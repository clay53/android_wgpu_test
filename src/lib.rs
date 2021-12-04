use winit::{
    window::{
        Window,
        WindowBuilder,
    },
    event::{
        Event,
        WindowEvent,
    },
    event_loop::ControlFlow,
};

pub struct Renderer {
    surface: wgpu::Surface,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    pipeline: wgpu::RenderPipeline,
}

impl Renderer {
    pub async fn new(window: &winit::window::Window) -> Self {
        let size = window.inner_size();
        let instance = wgpu::Instance::new(wgpu::Backends::all());
        let surface = unsafe { instance.create_surface(window) };
        let adapter = instance
            .enumerate_adapters(wgpu::Backends::all())
            .filter(|adapter| {
                surface.get_preferred_format(&adapter).is_some()
            })
            .next()
            .unwrap();
        let (device, queue) = adapter.request_device(
            &wgpu::DeviceDescriptor {
                features: wgpu::Features::empty(),
                limits: wgpu::Limits::downlevel_defaults(),
                label: None,
            },
            None,
        ).await.unwrap();
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface.get_preferred_format(&adapter).unwrap(),
            width: size.width,
            height: size.height,
            present_mode: wgpu::PresentMode::Immediate,
        };
        surface.configure(&device, &config);

        let shader = device.create_shader_module(&wgpu::include_wgsl!("shader.wgsl"));
        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Main Pipeline"),
            layout: None,
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[]
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[wgpu::ColorTargetState {
                    format: config.format,
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                }]
            }),
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Ccw,
                cull_mode: Some(wgpu::Face::Back),
                polygon_mode: wgpu::PolygonMode::Fill,
                clamp_depth: false,
                conservative: false,
            },
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            }
        });

        Self {
            surface,
            device,
            queue,
            config,
            pipeline,
        }
    }

    pub fn device(&self) -> &wgpu::Device { &self.device }
    pub fn surface(&self) -> &wgpu::Surface { &self.surface }
    pub fn queue(&self) -> &wgpu::Queue { &self.queue }
    pub fn config(&self) ->&wgpu::SurfaceConfiguration { &self.config }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0  {
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.reconfigure();
        }
    }

    pub fn reconfigure(&mut self) {
        self.surface.configure(&self.device, &self.config);
    }
}

fn resume(renderer: &mut Option<Renderer>, window: &Window) {
    *renderer = Some(futures::executor::block_on(Renderer::new(window)));
}

#[cfg(target_os = "android")]
fn pre_init(_renderer: &mut Option<Renderer>, _window: &Window) {}

#[cfg(not(target_os = "android"))]
fn pre_init(renderer: &mut Option<Renderer>, window: &Window) {
    resume(renderer, window);
}

#[cfg_attr(target_os = "android", ndk_glue::main(backtrace = "on"))]
pub fn main() {
    println!("Starting...");
    let event_loop = winit::event_loop::EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Sound Galaxy")
        .build(&event_loop).unwrap();

    let mut renderer = None;
    
    pre_init(&mut renderer, &window);

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                println!("Exit Requested. Exiting...");
                *control_flow = ControlFlow::Exit
            },
            Event::Resumed => {
                println!("Resuming...");
                resume(&mut renderer, &window);
                println!("Resumed!");
            },
            Event::Suspended => {
                println!("Suspending...");
            },
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window.id() => {
                let renderer = renderer.as_mut().unwrap();
                match event {
                    WindowEvent::Resized(physical_size) => {
                        renderer.resize(*physical_size);
                    },
                    WindowEvent::ScaleFactorChanged {
                        new_inner_size,
                        ..
                    } => {
                        renderer.resize(**new_inner_size);
                    },
                    _ => {}
                }
            },
            Event::RedrawRequested(_) => {
                let renderer = renderer.as_mut().unwrap();
                match renderer.surface().get_current_texture() {
                    Ok(surface_texture) => {
                        let view = surface_texture.texture.create_view(&wgpu::TextureViewDescriptor::default());
                        let mut encoder = renderer.device().create_command_encoder(&wgpu::CommandEncoderDescriptor {
                            label: Some("Render encoder"),
                        });
                        let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                            label: Some("Menu Render Pass"),
                            color_attachments: &[
                                wgpu::RenderPassColorAttachment {
                                    view: &view,
                                    resolve_target: None,
                                    ops: wgpu::Operations {
                                        load: wgpu::LoadOp::Clear(wgpu::Color {
                                            r: 0.0,
                                            g: 0.0,
                                            b: 0.0,
                                            a: 1.0
                                        }),
                                        store: true,
                                    }
                                }
                            ],
                            depth_stencil_attachment: None,
                        });
                        render_pass.set_pipeline(&renderer.pipeline);
                        render_pass.draw(0..6, 0..1);
                        drop(render_pass);
                        renderer.queue().submit(std::iter::once(encoder.finish()));
                        surface_texture.present();
                    },
                    Err(wgpu::SurfaceError::Lost) => {
                        eprintln!("Surface lost!");
                        renderer.reconfigure();
                    },
                    Err(wgpu::SurfaceError::OutOfMemory) => {
                        eprintln!("Out of memory!");
                        *control_flow = ControlFlow::Exit;
                    },
                    Err(e) => {
                        eprintln!("Surface error: {:?}", e);
                    },
                };
                std::thread::sleep(std::time::Duration::from_secs_f32(1.0/30.0)); // This limits FPS for my poor laptop that crashes if it runs at max fps
                window.request_redraw();
            },
            _ => ()
        }
    });
}