use winit::{application::ApplicationHandler, event::WindowEvent, event_loop::ActiveEventLoop, window::WindowId};

use crate::framework::windowed_app::app::WindowedApp;

impl ApplicationHandler for WindowedApp {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        self.create_window(event_loop);
        self.init_gpu();

        self.init_renderer();
    }

    fn window_event(
        &mut self,
        event_loop: &ActiveEventLoop,
        _window_id: WindowId,
        event: WindowEvent,
    ) {
        match event {
            WindowEvent::CloseRequested => {
                log::debug!("Window close requested");
                event_loop.exit();
            }
            WindowEvent::Resized(size) => self.resize_window(size),
            WindowEvent::RedrawRequested => self.redraw_window(),
            _ => self.handle_window_event(event),
        }
    }
}