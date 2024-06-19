use winit::event::WindowEvent;

use crate::framework::windowed_app::app::WindowedApp;

impl WindowedApp {
    pub fn handle_window_event(&mut self, event: WindowEvent) {
        match event {
            _ => (),
        }
    }
}