use app::App;
use log::LevelFilter;
use winit::event_loop::EventLoop;

mod app;
mod engine;
fn main() {
    println!("Hello, world!");
    let _ = env_logger::Builder::new()
        .filter_level(LevelFilter::Debug)
        .try_init();
    let mut app = App::new();
    let _event_loop = match EventLoop::new() {
        Ok(event_loop) => event_loop.run_app(&mut app).unwrap(),
        Err(_) => panic!("Unable to create the event loop!"),
    };
}
