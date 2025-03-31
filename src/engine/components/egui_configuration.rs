use std::sync::Arc;

use ash::{vk::{CommandBuffer, CommandPool, Format, Queue, RenderPass, SurfaceKHR}, Device};
use egui::Context;
use winit::window::Window;

use crate::engine::command_buffers;

use super::renderpass;


pub struct EGUIConfiguration {
    _context: Context,
    egui_state: egui_winit::State,
    gfx_queue: Arc<Queue>,
    surface: Arc<SurfaceKHR>,
    device: Arc<Device>,
    render_pass: RenderPass,
    command_buffer: CommandBuffer
}

impl EGUIConfiguration {
    pub fn new(
        device: Arc<Device>,
        window: &Window,
        gfx_queue: Arc<Queue>,
        surface: Arc<SurfaceKHR>,
        render_pass: RenderPass,
        command_pool: Arc<CommandPool>,
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
        let render_pass = renderpass::allocate_render_pass(&device, &Format::R32G32B32_SFLOAT).unwrap();
        let command_buffer = command_buffers::allocate_command_buffer(&device, *command_pool);
        Self {
            _context: context,
            egui_state,
            gfx_queue,
            surface,
            device,
            render_pass,
            command_buffer
        }
    }



}
