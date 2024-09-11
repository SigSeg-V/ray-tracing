use std::num::NonZeroU64;

use eframe::{
    egui_wgpu::wgpu::util::DeviceExt,
    egui_wgpu::{self, wgpu},
};
use egui::Vec2;

use crate::buf::{self, StorageBuffer, UniformBuffer};

use nalgebra_glm as glm;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct TemplateApp {
    // Example stuff:
    frame: f32,
    canvas_size: Vec2,
   }

impl Default for TemplateApp {
    fn default() -> Self {
        
        
        
        Self {
            // Example stuff:
            frame: 0.,
            canvas_size: Vec2::new(0.,0.),
        }
    }
}

impl TemplateApp {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Option<Self> {
        // Get the WGPU render state from the eframe creation context. This can also be retrieved
        // from `eframe::Frame` when you don't have a `CreationContext` available.
        let wgpu_render_state = cc.wgpu_render_state.as_ref()?;

        let device = &wgpu_render_state.device;

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("custom3d"),
            source: wgpu::ShaderSource::Wgsl(include_str!("./custom3d_wgpu_shader.wgsl").into()),
        });

        let uniforms = VertexUniforms {
            view_projection_matrix: unit_quad_projection_matrix(),
            model_matrix: glm::identity(),
        };
        let vertex_uniform_buffer = UniformBuffer::new_from_bytes(
            device,
            bytemuck::bytes_of(&uniforms),
            0_u32,
            Some("uniforms"),
        );

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            contents: bytemuck::cast_slice(VERTICES),
            usage: wgpu::BufferUsages::VERTEX,
            label: Some("VertexInput buffer"),
        });
        let vertex_uniform_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[vertex_uniform_buffer.layout(wgpu::ShaderStages::VERTEX)],
                label: Some("uniforms layout"),
            });
        let vertex_uniform_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &vertex_uniform_bind_group_layout,
            entries: &[vertex_uniform_buffer.binding()],
            label: Some("uniforms bind group"),
        });

        let frame_buffer = UniformBuffer::new(device, 16, 0, Some("frame data"));

        let image_buffer = {
            let buf = vec![[0.0_f32; 3]; 1920 * 1080usize];
            StorageBuffer::new_from_bytes(
                device, 
                bytemuck::cast_slice(buf.as_slice()), 
                1, 
                Some("image buffer"))
        };

        let image_bind_group_layout = device.create_bind_group_layout(
            &wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    frame_buffer.layout(wgpu::ShaderStages::FRAGMENT),
                    image_buffer.layout(wgpu::ShaderStages::FRAGMENT, false),
                ],
                label: Some("image layout"),
        });

        let image_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &image_bind_group_layout,
            entries: &[frame_buffer.binding(), image_buffer.binding()],
            label: Some("image bind group")
        });

        let bind_groups = vec![vertex_uniform_bind_group, image_bind_group];

        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("custom3d"),
            bind_group_layouts: &[&vertex_uniform_bind_group_layout, &image_bind_group_layout],
            push_constant_ranges: &[],
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("custom3d"),
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc()],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu_render_state.target_format.into())],
                compilation_options: wgpu::PipelineCompilationOptions::default(),
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState::default(),
            multiview: None,
        });

        // Because the graphics pipeline must have the same lifetime as the egui render pass,
        // instead of storing the pipeline in our `Custom3D` struct, we insert it into the
        // `paint_callback_resources` type map, which is stored alongside the render pass.
        wgpu_render_state
            .renderer
            .write()
            .callback_resources
            .insert(TriangleRenderResources {
                pipeline,
                bind_groups,
                frame_buffer,
                vertex_buffer,
            });
        Some(Default::default())
    }
}

impl eframe::App for TemplateApp {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            self.canvas_size = ui.available_size();
                    ui.vertical_centered_justified(|ui| {    
                        egui::Frame::canvas(ui.style()).show(ui, |ui| {
                        self.custom_painting(ui);
                        ui.ctx().request_repaint();
                    });
                });
        });
    }
}

struct CustomTriangleCallback {
    viewport: Vec2,
    frame: f32,
}

impl egui_wgpu::CallbackTrait for CustomTriangleCallback {
    fn prepare(
        &self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _screen_descriptor: &egui_wgpu::ScreenDescriptor,
        _egui_encoder: &mut wgpu::CommandEncoder,
        resources: &mut egui_wgpu::CallbackResources,
    ) -> Vec<wgpu::CommandBuffer> {
        let resources: &TriangleRenderResources = resources.get().unwrap();
        resources.prepare(device, queue, self.viewport, self.frame);
        Vec::new()
    }

    fn paint<'a>(
        &self,
        _info: egui::PaintCallbackInfo,
        render_pass: &mut wgpu::RenderPass<'a>,
        resources: &'a egui_wgpu::CallbackResources,
    ) {
        let resources: &TriangleRenderResources = resources.get().unwrap();
        resources.paint(render_pass);
    }
}

impl TemplateApp {
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        let (rect, response) =
            ui.allocate_exact_size(self.canvas_size, egui::Sense::drag());

        self.frame += 1.;
        ui.painter().add(egui_wgpu::Callback::new_paint_callback(
            rect,
            CustomTriangleCallback { viewport: self.canvas_size, frame: self.frame },
        ));
    }
}

struct TriangleRenderResources {
    pipeline: wgpu::RenderPipeline,
    bind_groups: Vec<wgpu::BindGroup>,
    frame_buffer: UniformBuffer,
    vertex_buffer: wgpu::Buffer
}

impl TriangleRenderResources {
    fn prepare(&self, _device: &wgpu::Device, queue: &wgpu::Queue, viewport: Vec2, frame: f32) {
        // Update our uniform buffer with the angle from the UI
        queue.write_buffer(
            self.frame_buffer.handle(),
            0,
            bytemuck::cast_slice(&[viewport.x, viewport.y, frame, 0.0]),
        );
    }

    fn paint<'rp>(&'rp self, render_pass: &mut wgpu::RenderPass<'rp>) {
        // Draw our triangle!
        render_pass.set_pipeline(&self.pipeline);

        self.bind_groups.iter().enumerate().for_each(|(index, bind_group)| render_pass.set_bind_group(index as u32, bind_group, &[]));
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice(..));
        render_pass.draw(0..6, 0..1);
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct VertexUniforms {
    view_projection_matrix: glm::Mat4,
    model_matrix: glm::Mat4,
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}

impl Vertex {
    fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                // @location(0)
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                // @location(1)
                wgpu::VertexAttribute {
                    format: wgpu::VertexFormat::Float32x2,
                    offset: std::mem::size_of::<[f32; 2]>() as u64,
                    shader_location: 1,
                },
            ],
        }
    }
}

fn unit_quad_projection_matrix() -> glm::Mat4 {
    let sw = 0.5_f32;
    let sh = 0.5_f32;

    // Our ortho camera is just centered at (0, 0)

    let left = -sw;
    let right = sw;
    let bottom = -sh;
    let top = sh;

    // DirectX, Metal, wgpu share the same left-handed coordinate system
    // for their normalized device coordinates:
    // https://github.com/gfx-rs/gfx/tree/master/src/backend/dx12
    glm::ortho_lh_zo(left, right, bottom, top, -1_f32, 1_f32)
}

const VERTICES: &[Vertex] = &[
    Vertex {
        position: [-0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [-0.5, -0.5],
        tex_coords: [0.0, 1.0],
    },
    Vertex {
        position: [0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [-0.5, 0.5],
        tex_coords: [0.0, 0.0],
    },
    Vertex {
        position: [0.5, -0.5],
        tex_coords: [1.0, 1.0],
    },
    Vertex {
        position: [0.5, 0.5],
        tex_coords: [1.0, 0.0],
    },
];