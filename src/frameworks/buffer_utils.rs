use rustc_hash::FxHashMap;
use wgpu::{BindGroup, BindGroupDescriptor, BindGroupEntry, BindGroupLayout, Buffer, Device};

pub fn initiate_buffer_map() -> FxHashMap<String, (u32, Buffer)> {
    FxHashMap::default()
}

pub fn add_buffer(map: &mut FxHashMap<&'static str, (u32, Buffer)>, buffer: Buffer, binding: u32, label: &'static str) {
    map.insert(label, (binding, buffer));
}

pub fn create_bind_group(device: &Device, buffers: &FxHashMap<&'static str, (u32, Buffer)>, bind_group_layout: &BindGroupLayout, label: Option<&str>) -> BindGroup {
    let entries: &[BindGroupEntry] = &buffers.iter().map(|(_label, (binding, buffer))| {
        BindGroupEntry {
            binding: *binding,
            resource: buffer.as_entire_binding()
        }
    }).collect::<Vec<BindGroupEntry>>();

    device.create_bind_group(&BindGroupDescriptor {
        layout: &bind_group_layout,
        entries,
        label
    })
}