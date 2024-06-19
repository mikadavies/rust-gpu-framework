use std::sync::Arc;

use winit::{
    event_loop::{ControlFlow, EventLoop},
    window::{Window, WindowAttributes},
};

use super::{
    gpu::gpu_wrapper::GPUWrapper,
    rendering::renderer::{RenderedObjectMap, Renderer},
    timers::frame_timer::FrameTimer,
};

#[derive(Default)]
pub struct WindowedApp {
    // Rendering
    pub window: Option<Arc<Window>>,
    pub window_attributes: WindowAttributes,
    pub gpu_wrapper: Option<GPUWrapper>,
    pub renderer: Renderer,
    pub rendered_objects: RenderedObjectMap,
    pub frametimer: FrameTimer,
    pub target_framerate: f32,
}

impl WindowedApp {
    pub fn new(title: &str) -> Self {
        Self {
            window: None,
            window_attributes: WindowAttributes::default().with_title(title),
            gpu_wrapper: None,
            renderer: Renderer::new(),
            rendered_objects: Default::default(),
            frametimer: Default::default(),
            target_framerate: 0.0,
        }
    }

    pub fn new_with_attributes(window_attributes: WindowAttributes) -> Self {
        Self {
            window: None,
            window_attributes,
            gpu_wrapper: None,
            renderer: Renderer::new(),
            rendered_objects: Default::default(),
            frametimer: Default::default(),
            target_framerate: 0.0,
        }
    }

    pub fn run(&mut self) {
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();
        event_loop.set_control_flow(ControlFlow::Poll);
        event_loop
            .run_app(self)
            .unwrap_or_else(|err| log::error!("Event loop error: {err:?}"));
    }
}
