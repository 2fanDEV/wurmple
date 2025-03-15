use ash::{vk::{CommandBuffer, CommandBufferAllocateInfo, CommandBufferBeginInfo, CommandBufferLevel, CommandBufferSubmitInfo, CommandBufferUsageFlags, CommandPool, CommandPoolCreateFlags, CommandPoolCreateInfo}, Device};

pub fn create_command_pool(device: &Device, queue_family_index: u32) -> CommandPool {
    let create_info = CommandPoolCreateInfo::default()
        .queue_family_index(queue_family_index)
        .flags(CommandPoolCreateFlags::RESET_COMMAND_BUFFER);

    unsafe { device.create_command_pool(&create_info, None).unwrap() }
}

pub fn allocate_command_buffer(device: &Device, command_pool: CommandPool) -> CommandBuffer {
    let allocate_info = CommandBufferAllocateInfo::default().command_pool(command_pool)
        .level(CommandBufferLevel::PRIMARY)
        .command_buffer_count(1);

    *unsafe { device.allocate_command_buffers(&allocate_info).unwrap().get(0).unwrap() }
}

pub fn begin_command_buffer(device: &Device, command_buffer: CommandBuffer, usage_flags: CommandBufferUsageFlags) {
    let begin_info = CommandBufferBeginInfo::default()
        .flags(CommandBufferUsageFlags::ONE_TIME_SUBMIT);

    unsafe { device.begin_command_buffer(command_buffer, &begin_info).unwrap() }
        
}

pub fn command_buffer_submit_info<'a>(command_buffer: CommandBuffer) -> CommandBufferSubmitInfo<'a> {
    CommandBufferSubmitInfo::default()
        .command_buffer(command_buffer)
        .device_mask(0)
}
