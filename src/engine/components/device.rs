use std::io::Error;

use ash::{
    khr::surface,
    vk::{
        DeviceCreateInfo, DeviceQueueCreateInfo, PhysicalDevice, PhysicalDeviceType, SurfaceKHR, KHR_PORTABILITY_SUBSET_NAME, KHR_PORTABILITY_SUBSET_SPEC_VERSION, KHR_SWAPCHAIN_NAME
    },
    Device, Instance,
};
use log::{debug, error};

use super::{swapchain_support_details::SwapchainSupportDetails, QueueFamilyIndices};

pub fn create_device(
    instance: &Instance,
    surface_instance: &surface::Instance,
    surface: SurfaceKHR,
) -> (Option<PhysicalDevice>, Option<ash::Device>) {
    let physical_device = pick_physical_device(instance, surface_instance, surface);
    match physical_device {
        Some(physical_device) => {
            let indices = QueueFamilyIndices::find_queue_family_indices(
                physical_device,
                instance,
                surface_instance,
                surface,
            );
            let features = unsafe { instance.get_physical_device_features(physical_device) };
            let extensions = vec![
                KHR_SWAPCHAIN_NAME.as_ptr(),
                KHR_PORTABILITY_SUBSET_NAME.as_ptr(),
            ];

            let device_queue_create_infos = vec![DeviceQueueCreateInfo::default()
                .queue_family_index(indices.graphics_q_idx.unwrap())
                .queue_priorities(&[1.0])];
            let device_create_infos = DeviceCreateInfo::default()
                .enabled_features(&features)
                .queue_create_infos(&device_queue_create_infos)
                .enabled_features(&features)
                .enabled_extension_names(&extensions);
            let device = unsafe {
                instance
                    .create_device(physical_device, &device_create_infos, None)
                    .ok()
            };
            (Some(physical_device), device)
        }
        None => (None, None),
    }
}

fn pick_physical_device(
    instance: &Instance,
    surface_instance: &surface::Instance,
    surface: SurfaceKHR,
) -> Option<PhysicalDevice> {
    match unsafe { instance.enumerate_physical_devices() } {
        Ok(devices) => {
            devices
                .into_iter()
                .filter(|device| is_device_suitable(*device, instance, &surface_instance, surface))
                .collect::<Vec<PhysicalDevice>>()
                .first()
                .map(|dev| dev.to_owned()) // we want an owned value to return
        }
        Err(_) => {
            error!("Failed to pick a physical device!");
            None
        }
    }
}

fn check_device_extensions(device: PhysicalDevice, instance: &Instance) -> bool {
    let extensions = vec![KHR_SWAPCHAIN_NAME.to_str().unwrap().to_string()];
    let p_device_extensions = unsafe {
        instance
            .enumerate_device_extension_properties(device)
            .unwrap()
            .iter()
            .map(|extension| {
                extension
                    .extension_name_as_c_str()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_string()
            })
            .collect::<Vec<String>>()
    };
    let mut count = 0;
    for extension in &extensions {
        if p_device_extensions.contains(&extension) {
            count = count + 1;
        }
    }
    extensions.len() == count
}

fn is_device_suitable(
    device: PhysicalDevice,
    instance: &Instance,
    surface_instance: &surface::Instance,
    surface: SurfaceKHR,
) -> bool {
    let queue_family_indices =
        QueueFamilyIndices::find_queue_family_indices(device, instance, surface_instance, surface);
    let features = unsafe { instance.get_physical_device_features(device) };
		let properties = unsafe { instance.get_physical_device_properties(device) };
    let swapchain_support_details =
        SwapchainSupportDetails::get_swapchain_support_details(device, surface_instance, surface)
            .unwrap();
    queue_family_indices.is_complete()
        && check_device_extensions(device, instance)
        && swapchain_support_details.is_swapchain_adequate()
        && features.sampler_anisotropy != 0
}
