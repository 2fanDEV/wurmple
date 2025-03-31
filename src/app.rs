use muda::{dpi::PhysicalSize, Menu, PredefinedMenuItem};
use winit::{
    application::ApplicationHandler,
    window::{Window, WindowAttributes},
};

use crate::{egui_renderer::EGUIRenderer, engine::Engine};

pub struct App {
    window: Option<Window>,
    engine: Option<Engine>,
}

impl App {
    pub fn new() -> Self {
        Self {
            window: None,
            engine: None,
            egui_renderer: None,
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        let window_attributes =
            WindowAttributes::default().with_inner_size(PhysicalSize::new(1280.0, 720.0));
        self.window = event_loop.create_window(window_attributes).ok();
        self.engine = Engine::new(self.window.as_mut().unwrap()).ok();
        let menu = Menu::new();
        menu.append_items(&[
            &PredefinedMenuItem::about(None, None),
            &PredefinedMenuItem::separator(),
            &PredefinedMenuItem::services(None),
            &PredefinedMenuItem::hide(None),
            &PredefinedMenuItem::undo(Some("")),
        ])
        .unwrap();
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        self.engine.as_mut().unwrap().draw();
    }
}
