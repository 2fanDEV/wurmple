use ash::{
    vk::{PhysicalDevice, PhysicalDeviceType, QueueFlags},
    Instance,
};
use log::error;

#[derive(Default)]
pub struct QueueFamilyIndices {
   pub graphics_q_idx: Option<u32>,
}

impl QueueFamilyIndices {
    pub fn find_queue_family_indices(
        physical_device: PhysicalDevice,
        instance: &Instance,
    ) -> QueueFamilyIndices {
        let queue_family_properties =
            unsafe { instance.get_physical_device_queue_family_properties(physical_device) };
        let mut indices = QueueFamilyIndices {
            graphics_q_idx: None,
        };

        for (idx, property) in queue_family_properties.iter().enumerate() {
            if property.queue_flags.eq(&QueueFlags::GRAPHICS) {
                indices.graphics_q_idx = Some(idx as u32);
            }
        }
        indices
    }

    fn is_complete(&self) -> bool {
        self.graphics_q_idx.is_some()
    }
}

fn pick_physical_device(instance: &Instance) -> Option<PhysicalDevice> {
    let physical_devices_res = unsafe { instance.enumerate_physical_devices() };
    match physical_devices_res {
        Ok(devices) => {
                devices.into_iter()
                .filter(|device| is_device_suitable(*device, instance))
                .collect::<Vec<PhysicalDevice>>()
                .first()
                .map(|dev| *dev)
        },
        Err(_) => {
            error!("Failed to pick a physical device!");
            None
        },
    }
}

fn is_device_suitable(device: PhysicalDevice, instance: &Instance) -> bool {
    let properties = unsafe { instance.get_physical_device_properties(device) };
    let features = unsafe { instance.get_physical_device_features(device) };
    let queue_family_indices = QueueFamilyIndices::find_queue_family_indices(device, instance);
    properties.device_type.eq(&PhysicalDeviceType::DISCRETE_GPU)
        && features.geometry_shader.eq(&1) && queue_family_indices.is_complete()
}
