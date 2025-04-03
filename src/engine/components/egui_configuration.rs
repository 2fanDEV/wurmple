use std::sync::Arc;

use ash::{
    vk::{Format, Queue, RenderPass, SurfaceKHR},
    Device,
};
use ash::vk::CommandPool;
use ash::vk::CommandBuffer;
use egui::Context;
use winit::window::Window;


use super::renderpass;

pub struct EGUIConfiguration {
   pub context: Context,
   pub egui_state: egui_winit::State,
   pub gfx_queue: Arc<Queue>,
   pub gfx_queue_family_index: u32,
   surface: Arc<SurfaceKHR>,
   pub device: Arc<Device>,
   pub render_pass: RenderPass,
}

impl EGUIConfiguration {
    pub fn new(
        device: Arc<Device>,
        window: &Window,
        gfx_queue: Arc<Queue>,
        gfx_queue_family_index: u32,
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
            renderpass::allocate_render_pass(&device, &Format::R16G16B16A16_SFLOAT).unwrap();
        Self {
            context,
            egui_state,
            gfx_queue,
            gfx_queue_family_index,
            surface,
            device,
            render_pass,
        }
    }
}
