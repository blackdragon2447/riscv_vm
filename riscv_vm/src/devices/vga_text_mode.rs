use std::{
    iter,
    sync::{Arc, RwLock},
    time::{Duration, Instant},
};

use wgpu::{
    include_wgsl, Backends, BlendState, ColorTargetState, DeviceDescriptor, Face, Features,
    FragmentState, FrontFace, Instance, InstanceDescriptor, Limits, MultisampleState,
    PipelineLayoutDescriptor, PolygonMode, PowerPreference, PrimitiveState, PrimitiveTopology,
    Queue, RenderPipeline, RenderPipelineDescriptor, RequestAdapterOptions, ShaderModuleDescriptor,
    ShaderSource, Surface, SurfaceConfiguration, SurfaceError, TextureUsages, VertexState,
};
use wgpu_text::{
    glyph_brush::{ab_glyph::FontRef, Layout, OwnedSection, Section, Text, VerticalAlign},
    BrushBuilder, TextBrush,
};
use winit::{
    dpi::{LogicalSize, PhysicalSize},
    event::{ElementState, Event, KeyboardInput, VirtualKeyCode, WindowEvent},
    event_loop::{self, ControlFlow, EventLoop, EventLoopBuilder},
    platform::x11::EventLoopBuilderExtX11,
    window::{Window, WindowBuilder},
};

use wgpu::CreateSurfaceError;
use winit::error::OsError;

use crate::memory::{address::Address, DeviceMemory};

use super::{AsyncDevice, Device, DeviceInitError};

pub struct VgaTextMode {
    surface: Surface,
    device: wgpu::Device,
    queue: Queue,
    config: SurfaceConfiguration,
    size: winit::dpi::PhysicalSize<u32>,
    render_pipeline: RenderPipeline,
    sections: Vec<OwnedSection>,
    // The window must be declared after the surface so
    // it gets dropped after it as the surface contains
    // unsafe references to the window's resources.
    window: Window,
    event_loop: Option<EventLoop<()>>,

    mem_start: Address,
}

const COLOR: [(f32, f32, f32, f32); 16] = [
    (0.0, 0.0, 0.0, 1.0),
    (0.0, 0.0, 2.0 / 3.0, 1.0),
    (0.0, 2.0 / 3.0, 0.0, 1.0),
    (0.0, 2.0 / 3.0, 2.0 / 3.0, 1.0),
    (2.0 / 3.0, 0.0, 0.0, 1.0),
    (2.0 / 3.0, 0.0, 2.0 / 3.0, 1.0),
    (2.0 / 3.0, 1.0 / 3.0, 0.0, 1.0),
    (2.0 / 3.0, 2.0 / 3.0, 2.0 / 3.0, 1.0),
    (1.0 / 3.0, 1.0 / 3.0, 1.0 / 3.0, 1.0),
    (1.0 / 3.0, 1.0 / 3.0, 1.0, 1.0),
    (1.0 / 3.0, 1.0, 1.0 / 3.0, 1.0),
    (1.0 / 3.0, 1.0, 1.0, 1.0),
    (1.0, 1.0 / 3.0, 1.0 / 3.0, 1.0),
    (1.0, 1.0 / 3.0, 1.0, 1.0),
    (1.0, 1.0, 1.0 / 3.0, 1.0),
    (1.0, 1.0, 1.0, 1.0),
];

impl Device for VgaTextMode {
    const MEN_SIZE: u64 = 80 * 25 * 2;

    fn init(mem: &mut DeviceMemory) -> Result<Self, super::DeviceInitError>
    where
        Self: Sized,
    {
        // mem.write_bytes(
        //     &[0x48, 15, 0x65, 15, 0x6C, 15, 0x6C, 15, 0x6F, 15],
        //     0xB0000u64.into(),
        // );

        let padding = 10;

        let event_loop = EventLoopBuilder::new()
            .with_x11()
            .with_any_thread(true)
            .build();
        let window = WindowBuilder::new()
            .with_resizable(false)
            .with_inner_size(PhysicalSize::new(
                80 * 9 + 2 * padding,
                25 * 16 + 2 * padding,
            ))
            .build(&event_loop)?;

        let instance = Instance::new(InstanceDescriptor {
            backends: Backends::all(),
            ..Default::default()
        });

        let surface = unsafe { instance.create_surface(&window) }?;

        let adapter = pollster::block_on(instance.request_adapter(&RequestAdapterOptions {
            power_preference: PowerPreference::default(),
            force_fallback_adapter: false,
            compatible_surface: Some(&surface),
        }))
        .unwrap();

        let (device, queue) = pollster::block_on(adapter.request_device(
            &DeviceDescriptor {
                label: None,
                features: Features::empty(),
                limits: Limits::default(),
            },
            None,
        ))
        .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);
        let config = SurfaceConfiguration {
            usage: TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: window.inner_size().width,
            height: window.inner_size().height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            view_formats: vec![],
        };

        surface.configure(&device, &config);

        let shader = device.create_shader_module(include_wgsl!("../../resources/shader.wgsl"));

        let render_pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("PipelineLayout"),
            bind_group_layouts: &[],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("RenderPipeline"),
            layout: Some(&render_pipeline_layout),
            vertex: VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            fragment: Some(FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(ColorTargetState {
                    format: config.format,
                    blend: Some(BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: FrontFace::Ccw,
                cull_mode: Some(Face::Back),
                unclipped_depth: false,
                polygon_mode: PolygonMode::Fill,
                conservative: false,
            },
            depth_stencil: None,
            multisample: MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
        });

        let mut sections = Vec::new();

        for i in 0..24 {
            sections.push(
                Section::default()
                    .with_bounds((config.width as f32, config.height as f32))
                    .with_screen_position((0.0 + padding as f32, 16.0 * i as f32 + padding as f32))
                    .with_layout(Layout::default().v_align(VerticalAlign::Top))
                    .to_owned(),
            );
        }

        Ok(Self {
            surface,
            device,
            queue,
            config,
            size: window.inner_size(),
            render_pipeline,
            sections,
            window,
            event_loop: Some(event_loop),

            mem_start: mem.start(),
        })
    }
}

impl AsyncDevice for VgaTextMode {
    fn run(mut self, mem: Arc<RwLock<DeviceMemory>>) {
        let event_loop = std::mem::take(&mut self.event_loop).unwrap();

        let target_framerate = Duration::from_secs_f64(1.0 / 5.0);
        let mut delta_time = Instant::now();

        event_loop.run(move |event, _, control_flow| {
            *control_flow = ControlFlow::Poll;

            for i in 0..24 {
                let bytes = mem
                    .read()
                    // we wanna panic here since if we don't have vmem anymore, there is
                    // probably something ver wrong.
                    .unwrap()
                    .read_bytes(self.mem_start + (i * (80 * 2)), 80 * 2)
                    // This unwrap is safe since were always withing the memory we requested.
                    .unwrap();

                let mut vec = Vec::new();
                for i in 0..80 {
                    // This unwrap is safe since were always withing the memory we requested.
                    vec.push((bytes.get(i * 2).unwrap(), bytes.get(i * 2 + 1).unwrap()));
                }

                let mut section = &mut self.sections[i as usize];
                section.text = vec![];
                for p in vec {
                    section.text.push(
                        (&Text::new(&String::from_utf8(vec![*p.0]).unwrap())
                            .with_color(COLOR[*p.1 as usize])
                            .to_owned())
                            .into(),
                    );
                }
            }

            match event {
                Event::WindowEvent {
                    ref event,
                    window_id,
                } if window_id == self.window.id() => {
                    // if !state.input(event) {
                    // UPDATED!
                    match event {
                        WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {
                            input:
                                KeyboardInput {
                                    state: ElementState::Pressed,
                                    virtual_keycode: Some(VirtualKeyCode::Escape),
                                    ..
                                },
                            ..
                        } => *control_flow = ControlFlow::Exit,
                        WindowEvent::Resized(physical_size) => {
                            self.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                            // new_inner_size is &&mut so w have to dereference it twice
                            self.resize(**new_inner_size);
                        }
                        _ => {}
                    }
                    // }
                }
                Event::RedrawRequested(window_id) if window_id == self.window.id() => {
                    // state.update();
                    match self.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            self.resize(self.size);
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => *control_flow = ControlFlow::Exit,

                        Err(wgpu::SurfaceError::Timeout) => println!("Surface timeout"),
                    }
                }
                Event::RedrawEventsCleared => {
                    // RedrawRequested will only trigger once, unless we manually
                    // request it.
                    if target_framerate <= delta_time.elapsed() {
                        self.window.request_redraw();
                        delta_time = Instant::now();
                    } else {
                        *control_flow = ControlFlow::WaitUntil(
                            Instant::now().checked_sub(delta_time.elapsed()).unwrap()
                                + target_framerate,
                        );
                    }
                }
                _ => {}
            }
        });
    }
}

impl VgaTextMode {
    fn render(&mut self) -> Result<(), SurfaceError> {
        let output = self.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let font: &[u8] = include_bytes!("../../resources/font.ttf");
        let mut brush = BrushBuilder::using_font_bytes(font).unwrap().build(
            &self.device,
            self.config.width,
            self.config.height,
            self.config.format,
        );

        let mut sections = vec![];

        for section in &self.sections {
            sections.push(section);
        }

        brush.queue(&self.device, &self.queue, sections).unwrap();

        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });

        {
            let mut render_pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("Render Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: 0.0,
                            g: 0.0,
                            b: 0.0,
                            a: 1.0,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                occlusion_query_set: None,
                timestamp_writes: None,
            });

            // render_pass.set_pipeline(&self.render_pipeline);
            brush.draw(&mut render_pass);
            // render_pass.draw(0..3, 0..1);
        }

        self.queue.submit(iter::once(encoder.finish()));
        output.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }
}

impl From<OsError> for DeviceInitError {
    fn from(value: OsError) -> Self {
        Self::Other(Box::new(value))
    }
}

impl From<CreateSurfaceError> for DeviceInitError {
    fn from(value: CreateSurfaceError) -> Self {
        Self::Other(Box::new(value))
    }
}
