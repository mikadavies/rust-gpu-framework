use std::sync::Arc;

use wgpu::{Adapter, Device, DeviceDescriptor, Features, Instance, Limits, PowerPreference, PresentMode, Queue, RequestAdapterOptions, Surface, SurfaceConfiguration};
use winit::{dpi::PhysicalSize, window::Window};

use crate::framework::windowed_app::app::WindowedApp;

pub struct GPUWrapper {
    _instance: Instance,
    pub surface: Surface<'static>,
    _adapter: Adapter,
    pub device: Device,
    pub queue: Queue,
    pub config: SurfaceConfiguration,
}

impl GPUWrapper {
    async fn new(window: &Arc<Window>) -> Self {
        // Initialise instance, adapter and surface
        let instance: Instance = Instance::default();
        let surface: Surface = instance.create_surface(window.clone()).unwrap();
        let adapter: Adapter = instance
            .request_adapter(&RequestAdapterOptions {
                power_preference: PowerPreference::HighPerformance,
                force_fallback_adapter: false,
                compatible_surface: Some(&surface),
            })
            .await
            .expect("Failed to find appropriate adapter");

        // Create logical device and command queue
        let (device, queue): (Device, Queue) = adapter
            .request_device(
                &DeviceDescriptor {
                    label: None,
                    required_features: Features::empty(),
                    required_limits: Limits::default(),
                },
                None,
            )
            .await
            .expect("Failed to create device");

        // Configure surface
        // -> Create config with window dimensions
        let mut size: PhysicalSize<u32> = window.inner_size();
        size.width = size.width.max(1);
        size.height = size.height.max(1);
        let mut config: SurfaceConfiguration = surface
            .get_default_config(&adapter, size.width, size.height)
            .unwrap();
        // -> SRGB support
        config.view_formats.push(config.format.add_srgb_suffix());
        // -> Preferred Presentation Mode (Mailbox = Fast VSync)
        config.present_mode = PresentMode::Mailbox;
        // -> Configure surface
        surface.configure(&device, &config);

        // Update App's GPU device field
        Self {
            _instance: instance,
            surface,
            _adapter: adapter,
            device,
            queue,
            config,
        }
    }
}

impl WindowedApp {
    pub fn init_gpu(&mut self) {
        log::debug!("Initialising GPU...");
        self.gpu_wrapper = Some(pollster::block_on(GPUWrapper::new(
            self.window.as_mut().unwrap(),
        )));
        log::debug!("Initialised GPU successfuly");
    }
}