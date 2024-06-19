use std::borrow::Cow;

use rustc_hash::FxHashMap;
use wgpu::{BindGroup, BindGroupLayout, BindGroupLayoutEntry, BindingType, Buffer, CommandEncoder, CommandEncoderDescriptor, Device, PipelineCompilationOptions, PipelineLayout, RenderPass, RenderPipeline, ShaderModule, ShaderModuleDescriptor, ShaderSource, SurfaceTexture, TextureView, TextureViewDescriptor};

use crate::framework::windowed_app::{app::WindowedApp, gpu::{
    gpu_wrapper::GPUWrapper, 
    utilities::*}};

pub type BufferMap = FxHashMap<&'static str, (u32, Buffer)>;
pub type BindGroupMap = FxHashMap<&'static str, (u32, BindGroup)>;
pub type BindGroupLayoutMap = FxHashMap<&'static str, BindGroupLayout>;
pub type RenderedObjectMap = FxHashMap<&'static str, (u32, Box<dyn RenderedObject>)>;

#[derive(Default)]
pub struct Renderer {
    // Pipelines
    pipeline: Option<RenderPipeline>,
    pipeline_layout: Option<PipelineLayout>,
    // Buffers
    buffers: BufferMap,
    // Bind groups
    bind_groups: BindGroupMap,
    bind_group_layouts: BindGroupLayoutMap,
}

impl Renderer {
    pub fn new() -> Self {
        Default::default()
    }

    fn init(
        &mut self,
        gpu_device: &GPUWrapper,
        rendered_objects: &RenderedObjectMap,
    ) {
        // Load shaders from disk
        let vertex_shader: ShaderModule = gpu_device
            .device
            .create_shader_module(ShaderModuleDescriptor {
                label: Some("vertex_shader"),
                source: ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    // PATH_TO_VERTEX_SHADER 
                    "../shaders/vertex.wgsl"
                ))),
            });
        let fragment_shader: ShaderModule = gpu_device
            .device
            .create_shader_module(ShaderModuleDescriptor {
                label: Some("fragment_shader"),
                source: ShaderSource::Wgsl(Cow::Borrowed(include_str!(
                    // PATH_TO_FRAGMENT_SHADER
                    "../shaders/fragment.wgsl"
                ))),
            });
        
        // Initialise layout entry vec
        let mut layout_entries: Vec<BindGroupLayoutEntry> = Vec::new();

        // Create buffers
        for (label, (binding, object)) in rendered_objects.iter() {
            log::info!("Creating buffer: {label}");
            let buffer: Buffer = object.to_buffer(label, &gpu_device.device);
            let binding_type: BindingType = object.buffer_binding_type(&buffer);
            log::info!("Adding layout entry at binding: {:}", *binding);
            layout_entries.push(create_fragment_bind_group_layout_entry(
                *binding,
                binding_type,
            ));
            log::info!("Adding buffer");
            add_buffer(&mut self.buffers, buffer, *binding, label);
        }

        // Create bind group
        let render_bind_group_layout = create_bind_group_layout(
            &gpu_device.device,
            &layout_entries,
            Some("render_bind_group_layout"),
        );
        let render_bind_group = create_bind_group(
            &gpu_device.device,
            &self.buffers,
            &render_bind_group_layout,
            Some("render_bind_group"),
        );

        // Create render pipeline
        let render_pipeline_layout = create_pipeline_layout(
            &gpu_device.device,
            &[&render_bind_group_layout],
            Some("render_pipeline_layout"),
        );
        let render_pipeline = create_render_pipeline(
            &gpu_device.device,
            &render_pipeline_layout,
            wgpu::VertexState {
                module: &vertex_shader,
                entry_point: "main",
                buffers: &[],
                compilation_options: PipelineCompilationOptions::default(),
            },
            wgpu::FragmentState {
                module: &fragment_shader,
                entry_point: "main",
                targets: &[Some(gpu_device.config.view_formats[0].into())],
                compilation_options: PipelineCompilationOptions::default(),
            },
            Some("render_pipeline"),
        );

        // Update renderer
        self.bind_group_layouts
            .insert("render", render_bind_group_layout);
        self.bind_groups.insert("render", (0, render_bind_group));
        self.pipeline = Some(render_pipeline);
        self.pipeline_layout = Some(render_pipeline_layout);
    }

    pub fn render(
        &mut self,
        gpu_device: &GPUWrapper,
        rendered_objects: &RenderedObjectMap,
    ) {
        //log::info!("Starting render");
        let frame: SurfaceTexture = gpu_device.surface.get_current_texture().unwrap();
        let view: TextureView = frame.texture.create_view(&TextureViewDescriptor {
            format: Some(gpu_device.config.view_formats[0]),
            ..TextureViewDescriptor::default()
        });

        // Create command encoder
        let mut encoder: CommandEncoder =
            gpu_device
                .device
                .create_command_encoder(&CommandEncoderDescriptor {
                    label: Some("command_encoder"),
                });

        // Update buffers
        self.update_buffers(&gpu_device.device, rendered_objects);

        // Render
        {
            // Initialise render pass
            let mut render_pass: RenderPass =
                create_default_render_pass(&mut encoder, &view, Some("render_pass"));
            render_pass.set_pipeline(self.pipeline.as_ref().unwrap());

            // Update bind groups
            self.bind_groups
                .iter_mut()
                .for_each(|(label, (index, bind_group))| {
                    *bind_group = create_bind_group(
                        &gpu_device.device,
                        &self.buffers,
                        self.bind_group_layouts.get(label).unwrap(),
                        Some(label),
                    );
                    render_pass.set_bind_group(*index, bind_group, &[]);
                });

            // Draw
            render_pass.draw(0..6, 0..1);
        }

        // Submit commands
        gpu_device.queue.submit(Some(encoder.finish()));

        // Present frame
        frame.present();
    }

    fn update_buffers(
        &mut self,
        device: &Device,
        rendered_objects: &RenderedObjectMap,
    ) {
        rendered_objects
            .iter()
            .for_each(|(label, (_binding, object))| {
                object.update_buffer(label, device, &mut self.buffers);
            });
    }
}

pub trait RenderedObject {
    fn to_buffer(&self, label: &str, device: &Device) -> Buffer;
    fn buffer_binding_type(&self, buffer: &Buffer) -> BindingType;
    fn update_buffer(&self, label: &str, device: &Device, buffer_map: &mut BufferMap);
}

impl WindowedApp {
    pub fn init_renderer(&mut self) {
        self.renderer
            .init(self.gpu_wrapper.as_mut().unwrap(), &self.rendered_objects);
    }

    pub fn add_to_rendered_objects(
        &mut self,
        object: Box<dyn RenderedObject>,
        label: &'static str,
        binding: u32,
    ) {
        self.rendered_objects.insert(label, (binding, object));
    }
}