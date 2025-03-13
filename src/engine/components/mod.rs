use ash::vk::{Extent2D, Format, ImageView, Pipeline, RenderPass};
use ash::{
    ext::debug_utils,
    khr::surface,
    vk::{DebugUtilsMessengerEXT, PhysicalDevice, QueueFlags, SurfaceKHR, SwapchainKHR},
    Entry,
};
use ash::{Device, Instance};
use instance::{create_instance, load_vulkan_library};
use renderpass::allocate_render_pass;
use swapchain_support_details::SwapchainSupportDetails;
use winit::window::Window;

mod device;
mod instance;
mod pipeline;
mod renderpass;
mod swapchain;
mod swapchain_support_details;
mod command_buffers;
mod util;

#[derive(Default, Clone, Copy)]
pub struct QueueFamilyIndices {
    pub graphics_q_idx: Option<u32>,
    pub presentation_q_idx: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn find_queue_family_indices(
        physical_device: PhysicalDevice,
        instance: &Instance,
        surface_instance: &surface::Instance,
        surface: SurfaceKHR,
    ) -> QueueFamilyIndices {
        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };

        let mut indices = QueueFamilyIndices {
            graphics_q_idx: None,
            presentation_q_idx: None,
        };

        for (idx, property) in queue_family_properties.iter().enumerate() {
            if property.queue_flags.contains(QueueFlags::GRAPHICS) {
                indices.graphics_q_idx = Some(idx as u32);
                let surface_support = unsafe {
                    surface_instance
                        .get_physical_device_surface_support(physical_device, idx as u32, surface)
                        .unwrap()
                };
                if surface_support {
                    indices.presentation_q_idx = Some(idx as u32);
                }
            }
        }
        indices
    }

    pub fn is_complete(&self) -> bool {
        self.graphics_q_idx.is_some() && self.presentation_q_idx.is_some()
    }
}

pub fn create_entry_and_instance(window: &Window) -> (Entry, Instance) {
    let entry = load_vulkan_library().unwrap();
    let instance = create_instance(&entry, window).unwrap();
    (entry, instance)
}

pub fn create_debugger(
    entry: &Entry,
    instance: &Instance,
) -> (debug_utils::Instance, DebugUtilsMessengerEXT) {
    instance::create_debugger(&entry, &instance)
}
pub fn create_device(
    instance: &Instance,
    surface_instance: &surface::Instance,
    surface: SurfaceKHR,
) -> (PhysicalDevice, ash::Device) {
    let res = device::create_device(instance, &surface_instance, surface);
    match res.0 {
        Some(physical_device) => match res.1 {
            Some(device) => (physical_device, device),
            None => panic!("Failed to create device"),
        },
        None => panic!("Failed to create physical_device"),
    }
}

pub fn get_queue_family_indices(
    physical_device: PhysicalDevice,
    instance: &Instance,
    surface_instance: &surface::Instance,
    surface: SurfaceKHR,
) -> QueueFamilyIndices {
    QueueFamilyIndices::find_queue_family_indices(
        physical_device,
        instance,
        surface_instance,
        surface,
    )
}

pub fn create_swapchain(
    physical_device: PhysicalDevice,
    device: &ash::khr::swapchain::Device,
    instance: &surface::Instance,
    surface: SurfaceKHR,
    window: &Window,
    indices: QueueFamilyIndices,
) -> SwapchainKHR {
    match swapchain::create_swapchain(physical_device, device, instance, surface, window, indices) {
        Ok(swapchain) => swapchain,
        Err(_) => panic!("Failed to create swapchain"),
    }
}

pub fn get_swapchain_support_details(
    physical_device: PhysicalDevice,
    instance: &surface::Instance,
    surface: SurfaceKHR,
) -> SwapchainSupportDetails {
    SwapchainSupportDetails::get_swapchain_support_details(physical_device, instance, surface)
        .unwrap()
}

pub fn create_image_views(
    device: &Device,
    swapchain_device: &ash::khr::swapchain::Device,
    swapchain_support_details: &SwapchainSupportDetails,
    swapchain: SwapchainKHR,
) -> Vec<ImageView> {
    match swapchain::create_image_views(
        device,
        swapchain_device,
        swapchain_support_details,
        swapchain,
    ) {
        Ok(image_views) => {
            if image_views.len() == 0 {
                panic!("failed to get image views");
            }
            return image_views;
        }
        Err(_) => panic!("failed to get image views"),
    }
}

pub fn create_graphics_pipelines(
    device: &Device,
    render_pass: &RenderPass,
    extent: &Extent2D,
) -> Vec<Pipeline> {
    pipeline::create_graphics_pipeline(device, render_pass, extent).unwrap()
}

pub fn create_render_pass(device: &Device, format: &Format) -> RenderPass {
    renderpass::allocate_render_pass(&device, format).unwrap()
}
