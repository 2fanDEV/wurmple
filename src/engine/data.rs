use ash::vk::{CommandBuffer, CommandPool, Fence, Semaphore};

use super::deletion_queue::DeletionQueue;

pub struct FrameData {
    pub command_pool: CommandPool,
    pub command_buffer: CommandBuffer,
    pub swapchain_semaphore: Semaphore,
    pub render_semaphore: Semaphore,
    pub render_fence: Fence,
    pub deletion_queue: DeletionQueue
}

impl FrameData {
    pub fn new(
        command_pool: CommandPool,
        command_buffer: CommandBuffer,
        swapchain_semaphore: Semaphore,
        render_semaphore: Semaphore,
        render_fence: Fence,
    ) -> Self {
        Self {
            command_pool,
            command_buffer,
            swapchain_semaphore,
            render_semaphore,
            render_fence,
            deletion_queue: DeletionQueue::new()
        }
    }
}
