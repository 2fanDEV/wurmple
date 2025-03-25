use std::sync::{Arc, Mutex};

use ash::{
    ext::debug_utils,
    khr::{surface, swapchain},
    vk::{
        DebugUtilsMessengerEXT, Extent2D, Image, ImageView, PhysicalDevice, Queue, SurfaceKHR,
        SwapchainKHR,
    },
    Device, Entry, Instance,
};
use vk_mem::Allocator;
use winit::{
    raw_window_handle::{HasDisplayHandle, HasWindowHandle},
    window::Window,
};

use super::{
    allocated_image::AllocatedImage,
    components::{
        create_allocated_image, create_debugger, create_device, create_entry_and_instance,
        create_image_views, create_swapchain, get_queue_family_indices,
        get_swapchain_support_details, QueueFamilyIndices, SwapchainSupportDetail,
    },
    deletion_queue::DeletionQueue,
};

pub const MAX_FRAMES: u32 = 2;
#[allow(dead_code)]
pub struct VkConfiguration {
    entry: Entry,
    pub instance: Instance,
    pub surface: SurfaceKHR,
    pub debug_instance: debug_utils::Instance,
    pub debugger: DebugUtilsMessengerEXT,
    pub physical_device: PhysicalDevice,
    pub device: Arc<Device>,
    pub indices: QueueFamilyIndices,
    pub graphics_queue: Queue,
    pub surface_instance: surface::Instance,
    pub swapchain_device: swapchain::Device,
    pub swapchain_support_details: SwapchainSupportDetail,
    pub extent: Extent2D,
    pub swapchain: SwapchainKHR,
    pub allocated_image: Arc<AllocatedImage>,
    pub images: Vec<Image>,
    pub image_views: Vec<ImageView>,
    pub main_deletion_queue: DeletionQueue,
    pub vma_allocator: Arc<Allocator>, //    graphics_pipelines: Vec<Pipeline>,
                                       //   render_pass: RenderPass,
}

#[allow(dead_code)]
impl VkConfiguration {
    pub fn new(window: &Window) -> Self {
        let mut main_deletion_queue: DeletionQueue = DeletionQueue::new();
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
        let (physical_device, device) =
            create_device(&instance, &surface_instance, surface, window);
        let device_arc = Arc::new(device);
        let indices =
            get_queue_family_indices(physical_device, &instance, &surface_instance, surface);
        let graphics_queue =
            unsafe { device_arc.get_device_queue(indices.graphics_q_idx.unwrap(), 0) };
        let swapchain_device: ash::khr::swapchain::Device =
            swapchain::Device::new(&instance, &device_arc);
        let swapchain_support_details =
            get_swapchain_support_details(physical_device, &surface_instance, surface, window);
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
        let vma_allocator_create_info =
            vk_mem::AllocatorCreateInfo::new(&instance, &device_arc, physical_device);
        let vma_allocator =
            Arc::new(unsafe { vk_mem::Allocator::new(vma_allocator_create_info).unwrap() });
        let allocated_image = Arc::new(create_allocated_image(
            &device_arc,
            &swapchain_device,
            &swapchain_support_details,
            swapchain,
            vma_allocator.clone(),
        ));

        let (images, image_views) = create_image_views(
            &device_arc,
            &swapchain_device,
            swapchain_support_details.clone(),
            swapchain,
        );

        Self::populate_queue(
            &mut main_deletion_queue,
            device_arc.clone(),
            allocated_image.clone(),
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
            device: device_arc.clone(),
            indices,
            graphics_queue,
            surface_instance,
            swapchain_device,
            swapchain_support_details,
            extent,
            swapchain,
            allocated_image,
            main_deletion_queue,
            vma_allocator,
            images,
            image_views, //   graphics_pipelines,
                         //  render_pass,
        }
    }

    fn populate_queue(
        deletion_queue: &mut DeletionQueue,
        device: Arc<Device>,
        allocated_image: Arc<AllocatedImage>,
    ) {
        unsafe {
            deletion_queue.enqueue(move || {
                device.destroy_image_view(allocated_image.clone().image_view, None)
            })
        }
    }

    pub fn cleanup(&self) {
        drop(self.vma_allocator.clone());
        let device = &self.device;
        unsafe {
            device.device_wait_idle().unwrap();
            self.swapchain_device
                .destroy_swapchain(self.swapchain, None);
            device.destroy_device(None);
            self.surface_instance.destroy_surface(self.surface, None);
            self.debug_instance
                .destroy_debug_utils_messenger(self.debugger, None);
            self.instance.destroy_instance(None);
        }
    }
}
