use std::sync::Arc;

use ash::vk::{Queue, SurfaceKHR};
use egui::Context;
use winit::{event::WindowEvent, window::Window};

pub struct EGUIRenderer {
    _context: Context,
    egui_state: egui_winit::State,
    gfx_queue: Arc<Queue>,
    surface: Arc<SurfaceKHR>
}

impl EGUIRenderer {
    pub fn new(window: &Window, gfx_queue: Arc<Queue>, surface: Arc<SurfaceKHR>) -> EGUIRenderer {
        let context = egui::Context::default();
        let egui_state = egui_winit::State::new(
            context.clone(),
            egui::viewport::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(2 * 1024),
        );

        Self{
            _context: context, 
            egui_state,
            gfx_queue,
            surface
        }
    }

    pub fn handle_input(&mut self, window: &Window, event: &WindowEvent) {
        let _response = self.egui_state.on_window_event(window, event);
    }

}
