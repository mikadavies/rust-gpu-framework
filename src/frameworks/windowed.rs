#![allow(dead_code)]

use std::sync::Arc;

use wgpu::{util::{backend_bits_from_env, dx12_shader_compiler_from_env, gles_minor_version_from_env, initialize_adapter_from_env_or_default}, Adapter, AdapterInfo, Backends, Device, DeviceDescriptor, DownlevelCapabilities, DownlevelFlags, Dx12Compiler, Features, Gles3MinorVersion, Instance, InstanceDescriptor, InstanceFlags, Limits, Queue, ShaderModel, Surface, SurfaceConfiguration, SurfaceTexture, TextureFormat, TextureView, TextureViewDescriptor};
use winit::{dpi::PhysicalSize, event::{Event, StartCause, WindowEvent}, event_loop::{EventLoop, EventLoopWindowTarget}, window::{Window, WindowBuilder}};

// General traits for the windowed application
pub trait WindowedApp: 'static + Sized {
    const SRGB: bool = true;

    fn optional_features() -> Features {
        Features::empty()
    }

    fn required_features() -> Features {
        Features::empty()
    }

    fn required_downlevel_capabilities() -> DownlevelCapabilities {
        DownlevelCapabilities {
            flags: DownlevelFlags::empty(),
            shader_model: ShaderModel::Sm5,
            ..DownlevelCapabilities::default()
        }
    }

    fn required_limits() -> Limits {
        Limits::downlevel_webgl2_defaults()
    }

    fn init(
        config: &SurfaceConfiguration,
        adapter: &Adapter,
        device: &Device,
        queue: &Queue
    ) -> Self;

    fn resize(&mut self,
        config: &SurfaceConfiguration,
        device: &Device,
        queue: &Queue
    );

    fn update(&mut self, event: WindowEvent);

    fn render(&mut self,
        view: &TextureView,
        device: &Device,
        queue: &Queue
    );
}

// Starts the logger
fn init_logger(level: log::Level) {
    match simple_logger::init_with_level(level) {
        Ok(_) => (),
        Err(err) => panic!("Could not initialise logger! Error: {err:?}")
    }
}

// Wrapper type for the window and event loop
struct EventLoopWrapper {
    event_loop: EventLoop<()>,
    window: Arc<Window>,
}

impl EventLoopWrapper {
    pub fn new(title: &str) -> Self {
        let event_loop: EventLoop<()> = EventLoop::new().unwrap();
        let builder: WindowBuilder = WindowBuilder::new().with_title(title);
        let window: Arc<Window> = Arc::new(builder.build(&event_loop).unwrap());
        Self {event_loop, window}
    }
}

// Wrapper type for the surface and surface configuration
struct SurfaceWrapper {
    surface: Option<Surface<'static>>,
    config: Option<SurfaceConfiguration>
}

impl SurfaceWrapper {
    fn new() -> Self {
        Self {
            surface: None,
            config: None
        }
    }

    // Check if an event is the start condition for the surface
    fn start_condition(event: &Event<()>) -> bool {
        match event {
            Event::NewEvents(StartCause::Init) => true,
            _ => false
        }
    }

    // Create surface once start condition is received
    fn resume(&mut self,
        context: &AppContext,
        window: Arc<Window>,
        srgb: bool
    ) {
        // Window size only actually valid after entering the event loop.
        let window_size: PhysicalSize<u32> = window.inner_size();
        let width: u32 = window_size.width.max(1);
        let height: u32 = window_size.height.max(1);
        log::info!("Surface resume {window_size:?}");

        // Create surface
        self.surface = Some(context.instance.create_surface(window).unwrap());
        let surface: &Surface = self.surface.as_ref().unwrap();

        // Get default config
        let mut config: SurfaceConfiguration = surface
            .get_default_config(&context.adapter, width, height)
            .expect("Surface not supported by adapter.");
        if srgb {
            config.view_formats.push(config.format.add_srgb_suffix());
        } else {
            let format: TextureFormat = config.format.remove_srgb_suffix();
            config.format = format;
            config.view_formats.push(format);
        }

        surface.configure(&context.device, &config);
        self.config = Some(config);
    }

    // Resize surface (not to zero)
    fn resize(&mut self,
        context: &AppContext,
        size: &PhysicalSize<u32>
    ) {
        log::info!("Surface resize {size:?}");

        let config: &mut SurfaceConfiguration = self.config.as_mut().unwrap();
        config.width = size.width.max(1);
        config.height = size.height.max(1);
        let surface: &Surface = self.surface.as_ref().unwrap();
        surface.configure(&context.device, config);
    }

    // Acquire next surface textures
    fn acquire(&mut self, context: &AppContext) -> SurfaceTexture {
        let surface: &Surface = self.surface.as_ref().unwrap();

        match surface.get_current_texture() {
            Ok(frame) => frame,
            // If timed out, try again
            Err(wgpu::SurfaceError::Timeout) => surface
                .get_current_texture()
                .expect("Failed to acquire next surface texture!"),
            Err(
                // If the surface is outdated or was lost, reconfigure it.
                wgpu::SurfaceError::Outdated
                | wgpu::SurfaceError::Lost
                // If OutOfMemory happens, reconfiguring may not help, but might as well try
                | wgpu::SurfaceError::OutOfMemory,
            ) => {
                surface.configure(&context.device, self.config());
                surface
                    .get_current_texture()
                    .expect("Failed to acquire next surface texture!")
            }
        }
    }

    fn get(&self) -> Option<&Surface> {
        self.surface.as_ref()
    }

    fn config(&self) -> &SurfaceConfiguration {
        self.config.as_ref().unwrap()
    }
}

// Context with global wgpu resources
struct AppContext {
    instance: Instance,
    adapter: Adapter,
    device: Device,
    queue: Queue
}

impl AppContext {
    // Initialize context
    async fn init_async<App: WindowedApp>(surface: &mut SurfaceWrapper) -> Self {
        log::info!("Initializing wgpu...");

        let backends: Backends = backend_bits_from_env().unwrap_or_default();
        let dx12_shader_compiler: Dx12Compiler = dx12_shader_compiler_from_env().unwrap_or_default();
        let gles_minor_version: Gles3MinorVersion = gles_minor_version_from_env().unwrap_or_default();

        let instance: Instance = Instance::new(InstanceDescriptor {
            backends,
            flags: InstanceFlags::from_build_config().with_env(),
            dx12_shader_compiler,
            gles_minor_version
        });

        let adapter: Adapter = initialize_adapter_from_env_or_default(&instance, surface.get())
            .await
            .expect("No suitable GPU adapters found on the system!");

        let adapter_info: AdapterInfo = adapter.get_info();
        log::info!("Using {} ({:?})", adapter_info.name, adapter_info.backend);

        let optional_features: Features = App::optional_features();
        let required_features: Features = App::required_features();
        let adapter_features: Features = adapter.features();
        assert!(
            adapter_features.contains(required_features),
            "GPU adapter does not support required features: {:?}",
            required_features - adapter_features
        );
        if !adapter_features.contains(optional_features) {
            log::warn!("GPU adapter does not support optional features: {:?}",
                optional_features - adapter_features
            );
        }

        let required_downlevel_capabilities: DownlevelCapabilities = App::required_downlevel_capabilities();
        let downlevel_capabilities: DownlevelCapabilities = adapter.get_downlevel_capabilities();
        assert!(
            downlevel_capabilities.shader_model >= required_downlevel_capabilities.shader_model,
            "GPU adapter does not support the minimum shader model: {:?}",
            required_downlevel_capabilities.shader_model
        );
        assert!(
            downlevel_capabilities
                .flags
                .contains(required_downlevel_capabilities.flags),
            "GPU adapter does not support the downlevel capabilities: {:?}",
            required_downlevel_capabilities.flags - downlevel_capabilities.flags
        );

        // Use the texture resolution limits from the adapter to support images the size of the surface.
        let needed_limits: Limits = App::required_limits().using_resolution(adapter.limits());

        let trace_dir: Result<String, std::env::VarError> = std::env::var("GPU_TRACE");
        let (device, queue): (Device, Queue) = adapter.request_device(
            &DeviceDescriptor { 
                label: None, 
                required_features: (optional_features & adapter_features) | required_features, 
                required_limits: needed_limits 
            },
            trace_dir.ok().as_ref().map(std::path::Path::new)
        ).await.expect("Unable to find a suitable GPU adapter");

        Self {
            instance,
            adapter,
            device,
            queue
        }
    }
}

// Frame counter for performance logging
struct FrameCounter {
    // Last printed instant
    last_printed_instant: web_time::Instant,
    // Number of frames since the last frame-time print
    frame_count: u32
}

impl FrameCounter {
    fn new() -> Self {
        Self {
            last_printed_instant: web_time::Instant::now(),
            frame_count: 0
        }
    }

    fn update(&mut self) {
        self.frame_count += 1;
        let new_instant: web_time::Instant = web_time::Instant::now();
        let elapsed_secs: f32 = (new_instant - self.last_printed_instant).as_secs_f32();
        if elapsed_secs > 1.0 {
            let elapsed_ms: f32 = elapsed_secs * 1000.0;
            let frame_time: f32 = elapsed_ms / self.frame_count as f32;
            let fps: f32 = self.frame_count as f32 / elapsed_secs;
            log::info!("Frame time {:.2}ms ({:.1} FPS)", frame_time, fps);

            self.last_printed_instant = new_instant;
            self.frame_count = 0;
        }
    }
}

async fn start<App: WindowedApp>(title: &str, log_level: log::Level) {
    init_logger(log_level);
    let window_loop: EventLoopWrapper = EventLoopWrapper::new(title);
    let mut surface: SurfaceWrapper = SurfaceWrapper::new();
    let context: AppContext = AppContext::init_async::<App>(&mut surface).await;
    let mut frame_counter: FrameCounter = FrameCounter::new();
    
    // Wait until valid surface to create app
    let mut app: Option<App> = None;

    let event_loop_function = EventLoop::run;
    log::info!("Entering event loop...");
    let _ = (event_loop_function)(
        window_loop.event_loop,
        move |event: Event<()>, target: &EventLoopWindowTarget<()>| {
            match event {
                ref e if SurfaceWrapper::start_condition(e) => {
                    surface.resume(&context, window_loop.window.clone(), App::SRGB);

                    if app.is_none() {
                        app = Some(App::init(
                            surface.config(),
                            &context.adapter,
                            &context.device,
                            &context.queue
                        ));
                    }
                },
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Resized(size) => {
                        surface.resize(&context, &size);
                        app.as_mut().unwrap().resize(
                            surface.config(), 
                            &context.device, 
                            &context.queue
                        );
                        window_loop.window.request_redraw();
                    },
                    WindowEvent::CloseRequested => {
                        target.exit();
                    },
                    WindowEvent::RedrawRequested => {
                        // Drop requested redraw if request comes in before Init (e.g. MacOS)
                        if app.is_none() {
                            return;
                        }
                        frame_counter.update();
                        let frame: SurfaceTexture = surface.acquire(&context);
                        let view: TextureView = frame.texture.create_view(&TextureViewDescriptor {
                            format: Some(surface.config().view_formats[0]),
                            ..TextureViewDescriptor::default()
                        });

                        app.as_mut().unwrap().render(&view, &context.device, &context.queue);
                        frame.present();
                        window_loop.window.request_redraw();
                    },
                    _ => app.as_mut().unwrap().update(event)
                },
                _ => {}
            }
        }
    );
}

pub fn run<App: WindowedApp>(title: &'static str, log_level: log::Level) {
    pollster::block_on(start::<App>(title, log_level));
}