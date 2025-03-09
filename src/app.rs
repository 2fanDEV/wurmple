use muda::dpi::PhysicalSize;
use winit::{application::ApplicationHandler, window::{Window, WindowAttributes}};

use crate::engine::Engine;


pub struct App {
    window: Option<Window>,
    engine: Option<Engine>,
}

impl App {
    pub fn new() -> Self {
        Self { window: None, 
        engine: None}
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes = WindowAttributes::default().with_inner_size(PhysicalSize::new(1080.0, 720.0)); 
        self.window = event_loop.create_window(window_attributes).ok();
        self.engine = Engine::new(self.window.as_mut().unwrap()).ok();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
    }
}

