use ash::{
    ext::debug_utils,
    khr::{surface, swapchain},
    vk::{DebugUtilsMessengerEXT, PhysicalDevice, Queue, SurfaceKHR, SwapchainKHR},
    Device, Entry, Instance,
};
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

use super::components::{
    create_debugger, create_device, create_entry_and_instance, create_image_views,
    create_swapchain, get_queue_family_indices, get_swapchain_support_details, QueueFamilyIndices,
};

pub struct VkConfiguration {
    entry: Entry,
    instance: Instance,
    surface: SurfaceKHR,
    debug_instance: debug_utils::Instance,
    debugger: DebugUtilsMessengerEXT,
    physical_device: PhysicalDevice,
    device: Device,
    indices: QueueFamilyIndices,
    graphics_queue: Queue,
    surface_instance: surface::Instance,
    swapchain_device: swapchain::Device,
    swapchain: SwapchainKHR,
}

impl VkConfiguration {
    pub fn new(window: &Window) -> Self {
        let (entry, instance) = create_entry_and_instance(window);
        let (debug_instance, debugger) = create_debugger(&entry, &instance);
        let surface = unsafe {
            ash_window::create_surface(
                &entry,
                &instance,
                window.display_handle().unwrap().as_raw(),
                window.window_handle().unwrap().as_raw(),
                None,
            )
        }
        .unwrap();
        let surface_instance = ash::khr::surface::Instance::new(&entry, &instance);
        let (physical_device, device) = create_device(&instance, &surface_instance, surface);
        let indices =
            get_queue_family_indices(physical_device, &instance, &surface_instance, surface);
        let graphics_queue = unsafe { device.get_device_queue(indices.graphics_q_idx.unwrap(), 0) };
        let swapchain_device: ash::khr::swapchain::Device =
            swapchain::Device::new(&instance, &device);
        let swapchain_support_details =
            get_swapchain_support_details(physical_device, &surface_instance, surface);
        let swapchain = create_swapchain(
            physical_device,
            &swapchain_device,
            &surface_instance,
            surface,
            window,
            indices,
        );
        let iamge_views = create_image_views(
            &device,
            &swapchain_device,
            swapchain_support_details,
            swapchain,
        );
        Self {
            entry,
            instance,
            surface,
            debug_instance,
            debugger,
            physical_device,
            device,
            indices,
            graphics_queue,
            surface_instance,
            swapchain_device,
            swapchain,
        }
    }

    pub fn destroy(&self) {
        unsafe { self.instance.destroy_instance(None) };
    }
}
