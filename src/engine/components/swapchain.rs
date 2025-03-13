use std::io::Error;

use ash::{
    khr::{surface, swapchain},
    vk::{
        ComponentMapping, ComponentSwizzle, CompositeAlphaFlagsKHR, ImageAspectFlags, ImageSubresourceRange, ImageUsageFlags, ImageView, ImageViewCreateInfo, ImageViewType, PhysicalDevice, SharingMode, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR
    },
    Device, Instance,
};
use winit::window::{self, Window};

use super::{
    swapchain_support_details::{self, SwapchainSupportDetails},
    QueueFamilyIndices,
};

pub fn create_swapchain(
    physical_device: PhysicalDevice,
    device: &swapchain::Device,
    instance: &surface::Instance,
    surface: SurfaceKHR,
    window: &Window,
    indices: QueueFamilyIndices,
) -> Result<SwapchainKHR, Error> {
    let graphics_queue_index = indices.graphics_q_idx.unwrap();
    let presentation_queue_index = indices.presentation_q_idx.unwrap();
    let swapchain_support_details =
        SwapchainSupportDetails::get_swapchain_support_details(physical_device, instance, surface)
            .unwrap();
    let surface_format = swapchain_support_details.clone().choose_swapchain_format();
    let present_mode = swapchain_support_details
        .clone()
        .choose_swapchain_present_mode();
    let extent = swapchain_support_details
        .clone()
        .choose_swapchain_extent(window);
    let image_count = swapchain_support_details.clone().choose_image_count();

    let mut swapchain_create_info = SwapchainCreateInfoKHR::default()
        .surface(surface)
        .min_image_count(image_count)
        .image_format(surface_format.format)
        .image_color_space(surface_format.color_space)
        .image_extent(extent)
        .image_array_layers(1)
        .image_usage(ImageUsageFlags::COLOR_ATTACHMENT | ImageUsageFlags::TRANSFER_DST)
        .pre_transform(swapchain_support_details.capabilities.current_transform)
        .composite_alpha(CompositeAlphaFlagsKHR::OPAQUE)
        .clipped(true)
        .present_mode(present_mode)
        .image_extent(extent);

    let indices_vec = [graphics_queue_index, presentation_queue_index];
    if indices.graphics_q_idx.unwrap() != indices.presentation_q_idx.unwrap() {
        swapchain_create_info = swapchain_create_info
            .image_sharing_mode(SharingMode::CONCURRENT)
            .queue_family_indices(&indices_vec);
    } else {
        swapchain_create_info = swapchain_create_info.image_sharing_mode(SharingMode::EXCLUSIVE);
    }

    Ok(unsafe {
        device
            .create_swapchain(&swapchain_create_info, None)
            .unwrap()
    })
}

pub fn create_image_views(
    device: &Device,
    swapchain_device: &ash::khr::swapchain::Device,
    swapchain_support_details: &SwapchainSupportDetails,
    swapchain: SwapchainKHR,
) -> Result<Vec<ImageView>, Error> {
    let mut image_views = vec![];
    let images = unsafe { swapchain_device.get_swapchain_images(swapchain).unwrap() };
    for image in images {
        let image_view_create_info = ImageViewCreateInfo::default()
            .image(image)
            .view_type(ImageViewType::TYPE_2D)
            .format(
                swapchain_support_details
                    .clone()
                    .choose_swapchain_format()
                    .format,
            )
            .components(
                ComponentMapping::default()
                    .r(ComponentSwizzle::IDENTITY)
                    .g(ComponentSwizzle::IDENTITY)
                    .b(ComponentSwizzle::IDENTITY)
                    .a(ComponentSwizzle::IDENTITY),
            )
            .subresource_range(
                ImageSubresourceRange::default()
                    .aspect_mask(ImageAspectFlags::COLOR)
                    .base_mip_level(0)
                    .level_count(1)
                    .base_array_layer(0)
                    .layer_count(1),
            );
        let image_view = unsafe {
            device
                .create_image_view(&image_view_create_info, None)
                .unwrap()
        };
        image_views.push(image_view);
    }
    Ok(image_views)
}
