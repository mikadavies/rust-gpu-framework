use std::sync::Arc;

use winit::{dpi::PhysicalSize, event_loop::ActiveEventLoop, window::Window};

use crate::framework::windowed_app::{app::WindowedApp, gpu::gpu_wrapper::GPUWrapper};

impl WindowedApp {
    pub fn create_window(&mut self, event_loop: &ActiveEventLoop) {
        log::debug!("Creating window...");
        let window: Arc<Window> = Arc::new(
            event_loop
                .create_window(self.window_attributes.to_owned())
                .unwrap(),
        );
        self.window = Some(window.clone());
        log::debug!("Created window successfully");
    }
    
    pub fn redraw_window(&mut self) {
        // Update general components
        self.frametimer.update();
        self.frametimer.log();

        // Render frame
        if self.frametimer.is_it_time_to_refresh(self.target_framerate) {
            // Render
            self.renderer
                .render(self.gpu_wrapper.as_mut().unwrap(), &self.rendered_objects);
        }

        // Request next redraw
        self.window.as_ref().unwrap().request_redraw();
    }

    pub fn resize_window(&mut self, new_size: PhysicalSize<u32>) {
        // Ensure size is non-zero
        let mut size: PhysicalSize<u32> = new_size;
        size.width = size.width.max(1);
        size.height = size.height.max(1);

        // Update config
        let gpu_device: &mut GPUWrapper = self.gpu_wrapper.as_mut().unwrap();
        gpu_device.config.width = size.width;
        gpu_device.config.height = size.height;

        // Update surface
        gpu_device
            .surface
            .configure(&gpu_device.device, &gpu_device.config);
        log::debug!("Window resized to {:}x{:}", size.width, size.height);

        // Request next redraw
        self.window.as_ref().unwrap().request_redraw();
    }
}