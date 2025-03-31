use wurmple::app::App;
use log::LevelFilter;
use winit::event_loop::EventLoop;
fn main() {
    println!("Hello, world!");
    let _ = env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .try_init();
    let mut app = App::new();
    let _ = EventLoop::new().unwrap().run_app(&mut app);
}
