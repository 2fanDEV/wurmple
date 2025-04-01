use std::sync::Arc;

use ash::{
    vk::{CommandBuffer, CommandPool, Format, Queue, RenderPass, SurfaceKHR},
    Device,
};
use egui::Context;
use winit::window::Window;

use crate::engine::command_buffers;

use super::renderpass;

pub struct EGUIConfiguration {
   pub _context: Context,
   pub egui_state: egui_winit::State,
   pub gfx_queue: Arc<Queue>,
   surface: Arc<SurfaceKHR>,
   pub device: Arc<Device>,
   pub render_pass: RenderPass,
}

impl EGUIConfiguration {
    pub fn new(
        device: Arc<Device>,
        window: &Window,
        gfx_queue: Arc<Queue>,
        surface: Arc<SurfaceKHR>,
    ) -> EGUIConfiguration {
        let context = egui::Context::default();
        let egui_state = egui_winit::State::new(
            context.clone(),
            egui::viewport::ViewportId::ROOT,
            &window,
            Some(window.scale_factor() as f32),
            None,
            Some(2 * 1024),
        );
        let render_pass =
            renderpass::allocate_render_pass(&device, &Format::R32G32B32A32_SFLOAT).unwrap();
        Self {
            _context: context,
            egui_state,
            gfx_queue,
            surface,
            device,
            render_pass,
        }
    }
}
