use framework::windowed_app::app::WindowedApp;

mod framework;

const APP_TYPE: AppType = AppType::Windowed;

fn main() {
    simple_logger::init_with_level(log::Level::Info).unwrap();
    match APP_TYPE {
        AppType::Windowed => {
            let mut app: WindowedApp = WindowedApp::new("Window");
            app.run();
        }
        AppType::Windowless => (),
    }
}

#[allow(dead_code)]
enum AppType {
    Windowed,
    Windowless,
}
