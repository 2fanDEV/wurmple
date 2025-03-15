use ash::{
    ext::debug_utils,
    khr::{surface, swapchain},
    vk::{
        DebugUtilsMessengerEXT, Extent2D, Image, ImageView, PhysicalDevice, Queue, SurfaceKHR, SwapchainKHR
    },
    Device, Entry, Instance,
};
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

use super::components::{
     create_debugger, create_device, create_entry_and_instance,
    create_image_views, create_swapchain,
    get_queue_family_indices, get_swapchain_support_details, QueueFamilyIndices, SwapchainSupportDetail,
};

pub struct VkConfiguration {
    entry: Entry,
    instance: Instance,
    surface: SurfaceKHR,
    debug_instance: debug_utils::Instance,
    debugger: DebugUtilsMessengerEXT,
    physical_device: PhysicalDevice,
    pub device: Device,
    pub indices: QueueFamilyIndices,
    pub graphics_queue: Queue,
    surface_instance: surface::Instance,
    pub swapchain_device: swapchain::Device,
    swapchain_support_details: SwapchainSupportDetail,
    extent: Extent2D,
    pub swapchain: SwapchainKHR,
    pub images: Vec<Image>,
    pub image_views: Vec<ImageView>,
    //    graphics_pipelines: Vec<Pipeline>,
    //   render_pass: RenderPass,
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
        let extent = swapchain_support_details
            .clone()
            .choose_swapchain_extent(window);
        let swapchain = create_swapchain(
            physical_device,
            &swapchain_device,
            &surface_instance,
            surface,
            window,
            indices,
        );
        let (images, image_views) = create_image_views(
            &device,
            &swapchain_device,
            &swapchain_support_details,
            swapchain,
        );
        /*    let render_pass = create_render_pass(
                 &device,
                 &swapchain_support_details.choose_swapchain_format().format,
             );
            let graphics_pipelines = create_graphics_pipelines(&device, &render_pass, &extent);
        */
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
            swapchain_support_details,
            extent,
            swapchain,
            images,
            image_views,
            //   graphics_pipelines,
            //  render_pass,
        }
    }

    pub fn destroy(&self) {
        unsafe { self.instance.destroy_instance(None) };
    }
}
