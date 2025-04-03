use std::{io::Error, sync::Arc};

use ash::{
    khr::{surface, swapchain},
    vk::{
        BufferCreateInfo, ComponentMapping, ComponentSwizzle, CompositeAlphaFlagsKHR, Extent3D, Format, Image, ImageAspectFlags, ImageSubresourceRange, ImageUsageFlags, ImageView, ImageViewCreateFlags, ImageViewCreateInfo, ImageViewType, MemoryPropertyFlags, PhysicalDevice, SharingMode, SurfaceKHR, SwapchainCreateInfoKHR, SwapchainKHR
    },
    Device, Instance,
};
use vk_mem::{Alloc, MemoryUsage};
use winit::window::{self, Window};

use crate::engine::{
    allocated_image::AllocatedImage, egui_renderer::RenderInformation, image_ops::{image_create_info, image_subresource_range, image_view_create_info}
};

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
    let swapchain_support_details = SwapchainSupportDetails::get_swapchain_support_details(
        physical_device,
        instance,
        surface,
        window,
    )
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

pub fn create_allocated_image(
    device: &Device,
    swapchain_device: &ash::khr::swapchain::Device,
    swapchain_support_details: &SwapchainSupportDetails,
    swapchain: SwapchainKHR,
    vma_allocator: Arc<vk_mem::Allocator>,
) -> Result<AllocatedImage, Error> {
    let extent = Extent3D::default()
        .width(swapchain_support_details.window_sizes.width)
        .height(swapchain_support_details.window_sizes.height)
        .depth(1);

    let image_create_info = image_create_info(
        Format::R16G16B16A16_SFLOAT,
        ImageUsageFlags::TRANSFER_SRC
            | ImageUsageFlags::TRANSFER_DST
            | ImageUsageFlags::STORAGE
            | ImageUsageFlags::COLOR_ATTACHMENT,
        extent,
    );

    let mut allocation_create_info = vk_mem::AllocationCreateInfo::default();
    allocation_create_info.required_flags = MemoryPropertyFlags::DEVICE_LOCAL;
    allocation_create_info.usage = MemoryUsage::GpuOnly;

    let (image, allocation) = unsafe {
        vma_allocator
            .create_image(&image_create_info, &allocation_create_info)
            .unwrap()
    };

    let image_view_create_info =
        image_view_create_info(image, Format::R16G16B16A16_SFLOAT, ImageAspectFlags::COLOR);
    let image_view = unsafe {
        device
            .create_image_view(&image_view_create_info, None)
            .unwrap()
    };
    let allocated_image = AllocatedImage::new(
        image,
        image_view,
        allocation,
        extent,
        Format::R16G16B16A16_SFLOAT,
    );
    Ok(allocated_image)
}

pub fn create_swapchain_image_and_views(
    device: &Device,
    swapchain_device: &ash::khr::swapchain::Device,
    swapchain_support_details: SwapchainSupportDetails,
    swapchain: SwapchainKHR,
) -> Result<(Vec<Image>, Vec<ImageView>), Error> {
    unsafe {
        let images = swapchain_device.get_swapchain_images(swapchain).unwrap();
        let image_views = images
            .iter()
            .map(|image| {
                let image_view_create_info = ImageViewCreateInfo::default()
                    .image(*image)
                    .format(
                        swapchain_support_details
                            .clone()
                            .choose_swapchain_format()
                            .format,
                    )
                    .subresource_range(image_subresource_range(ImageAspectFlags::COLOR))
                    .view_type(ImageViewType::TYPE_2D)
                    .components(
                        ComponentMapping::default()
                            .r(ComponentSwizzle::IDENTITY)
                            .g(ComponentSwizzle::IDENTITY)
                            .b(ComponentSwizzle::IDENTITY)
                            .a(ComponentSwizzle::IDENTITY),
                    );

                device
                    .create_image_view(&image_view_create_info, None)
                    .unwrap()
            })
            .collect::<Vec<ImageView>>();
        Ok((images, image_views))
    }
}
