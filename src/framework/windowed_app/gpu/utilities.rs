#![allow(dead_code)]

use wgpu::{
    util::{BufferInitDescriptor, DeviceExt}, 
    BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, BindGroupLayoutDescriptor, BindGroupLayoutEntry, BindingType, Buffer, BufferBindingType, BufferUsages, CommandEncoder, ComputePipeline, ComputePipelineDescriptor, Device, FragmentState, MultisampleState, PipelineCompilationOptions, PipelineLayout, PipelineLayoutDescriptor, PrimitiveState, RenderPass, RenderPipeline, RenderPipelineDescriptor, ShaderModule, ShaderStages, TextureView, VertexState};

use crate::framework::windowed_app::rendering::renderer::BufferMap;

// <---- Bind groups ---->
pub fn create_bind_group(
    device: &Device,
    buffers: &BufferMap,
    bind_group_layout: &BindGroupLayout,
    label: Option<&str>,
) -> BindGroup {
    let entries: &[BindGroupEntry] = &buffers
        .iter()
        .map(|(_label, (binding, buffer))| BindGroupEntry {
            binding: *binding,
            resource: buffer.as_entire_binding(),
        })
        .collect::<Vec<BindGroupEntry>>();

    device.create_bind_group(&BindGroupDescriptor {
        layout: bind_group_layout,
        entries,
        label,
    })
}

pub fn create_bind_group_entry(binding: u32, buffer: &Buffer) -> BindGroupEntry {
    BindGroupEntry {
        binding,
        resource: buffer.as_entire_binding(),
    }
}

pub fn create_bind_group_layout(
    device: &Device,
    entries: &[BindGroupLayoutEntry],
    label: Option<&str>,
) -> BindGroupLayout {
    device.create_bind_group_layout(&BindGroupLayoutDescriptor { label, entries })
}

pub fn create_bind_group_layout_entry(
    binding: u32,
    visibility: ShaderStages,
    binding_type: BindingType,
) -> BindGroupLayoutEntry {
    BindGroupLayoutEntry {
        binding,
        visibility,
        ty: binding_type,
        count: None,
    }
}

// Bind group layout entry subfunctions
pub fn create_compute_bind_group_layout_entry(
    binding: u32,
    binding_type: BindingType,
) -> BindGroupLayoutEntry {
    create_bind_group_layout_entry(binding, ShaderStages::COMPUTE, binding_type)
}

pub fn create_fragment_bind_group_layout_entry(
    binding: u32,
    binding_type: BindingType,
) -> BindGroupLayoutEntry {
    create_bind_group_layout_entry(binding, ShaderStages::FRAGMENT, binding_type)
}

pub fn create_vertex_bind_group_layout_entry(
    binding: u32,
    binding_type: BindingType,
) -> BindGroupLayoutEntry {
    create_bind_group_layout_entry(binding, ShaderStages::VERTEX, binding_type)
}

pub fn create_render_bind_group_layout_entry(
    binding: u32,
    binding_type: BindingType,
) -> BindGroupLayoutEntry {
    create_bind_group_layout_entry(binding, ShaderStages::VERTEX_FRAGMENT, binding_type)
}

//-----
pub fn create_buffer_binding_type(
    storage: bool,
    read_only: bool,
    has_dynamic_offset: bool,
    buffer: &Buffer,
) -> BindingType {
    //log::info!("min_binding_size: {:?}", std::num::NonZeroU64::new(buffer.size()).unwrap());
    BindingType::Buffer {
        ty: match storage {
            false => BufferBindingType::Uniform,
            true => BufferBindingType::Storage {
                read_only,
            },
        },
        has_dynamic_offset,
        min_binding_size: Some(std::num::NonZeroU64::new(buffer.size()).unwrap()),
    }
}

//======================================================================
// <---- Buffers ---->
pub fn create_buffer(
    device: &Device,
    contents: &[u8],
    usage: BufferUsages,
    label: Option<&str>,
) -> Buffer {
    device.create_buffer_init(&BufferInitDescriptor {
        label,
        contents,
        usage,
    })
}

pub fn add_buffer(
    map: &mut BufferMap,
    buffer: Buffer,
    binding: u32,
    label: &'static str,
) {
    map.insert(label, (binding, buffer));
}

pub fn update_buffer(
    map: &mut BufferMap,
    label: &str,
    new_buffer: Buffer,
) {
    map.get_mut(label).unwrap().1 = new_buffer;
}

pub fn get_buffer_usage(
    map: &BufferMap,
    label: &str,
) -> BufferUsages {
    map.get(label).unwrap().1.usage()
}

//======================================================================
// <---- Pipelines ---->
pub fn create_render_pipeline(
    device: &Device,
    layout: &PipelineLayout,
    vertex: VertexState,
    fragment: FragmentState,
    label: Option<&str>,
) -> RenderPipeline {
    device.create_render_pipeline(&RenderPipelineDescriptor {
        label,
        layout: Some(layout),
        vertex,
        fragment: Some(fragment),
        primitive: PrimitiveState::default(),
        depth_stencil: None,
        multiview: None,
        multisample: MultisampleState::default(),
    })
}

pub fn create_compute_pipeline(
    device: &Device,
    layout: &PipelineLayout,
    module: &ShaderModule,
    entry_point: &str,
    label: Option<&str>,
) -> ComputePipeline {
    device.create_compute_pipeline(&ComputePipelineDescriptor {
        label,
        layout: Some(layout),
        module,
        entry_point,
        compilation_options: PipelineCompilationOptions::default(),
    })
}

pub fn create_pipeline_layout(
    device: &Device,
    bind_group_layouts: &[&BindGroupLayout],
    label: Option<&str>,
) -> PipelineLayout {
    device.create_pipeline_layout(&PipelineLayoutDescriptor {
        label,
        bind_group_layouts,
        push_constant_ranges: &[],
    })
}

// <---- Render Pass ---->
pub fn create_default_render_pass<'a>(
    encoder: &'a mut CommandEncoder,
    view: &'a TextureView,
    label: Option<&'a str>,
) -> RenderPass<'a> {
    encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
        label,
        color_attachments: &[Some(wgpu::RenderPassColorAttachment {
            view,
            resolve_target: None,
            ops: wgpu::Operations {
                load: wgpu::LoadOp::Clear(wgpu::Color::TRANSPARENT),
                store: wgpu::StoreOp::Store,
            },
        })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
    })
}