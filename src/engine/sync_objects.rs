use ash::{
    vk::{Fence, FenceCreateFlags, FenceCreateInfo, Semaphore, SemaphoreCreateFlags, SemaphoreCreateInfo},
    Device,
};

pub fn create_semaphore(device: &Device) -> Semaphore {
    let create_info = SemaphoreCreateInfo::default().flags(SemaphoreCreateFlags::default());
    unsafe { device.create_semaphore(&create_info, None).unwrap() }
}

pub fn create_fence(device: &Device) -> Fence {
    let create_info = FenceCreateInfo::default().flags(FenceCreateFlags::SIGNALED);
    unsafe { device.create_fence(&create_info, None).unwrap() }
}

