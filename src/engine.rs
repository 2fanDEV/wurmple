use std::io::Error;

use ash::vk::{
    ClearColorValue, CommandBufferResetFlags, CommandBufferUsageFlags, ImageAspectFlags, ImageLayout, PresentInfoKHR, SubmitInfo
};
use command_buffers::{allocate_command_buffer, begin_command_buffer, create_command_pool};
use configuration::VkConfiguration;
use data::FrameData;
use image_ops::{image_subresource_range, image_transition};
use sync_objects::{create_fence, create_semaphore};
use winit::window::Window;
mod command_buffers;
mod components;
mod configuration;
mod data;
mod image_ops;
mod sync_objects;

const MAX_FRAMES: u32 = 2;

pub struct Engine {
    configuration: VkConfiguration,
    frame_data: Vec<FrameData>,
    current_frame: usize,
}
impl Engine {
    pub fn new(window: &Window) -> Result<Self, Error> {
        let configuration = VkConfiguration::new(window);
        let mut frame_data = Vec::new();
        let command_pool = create_command_pool(
            &configuration.device,
            configuration.indices.graphics_q_idx.unwrap(),
        );
        let current_frame = 0;
        for _i in 0..MAX_FRAMES {
            let command_buffer = allocate_command_buffer(&configuration.device, command_pool);
            let fence = create_fence(&configuration.device);
            let swapchain_semaphore = create_semaphore(&configuration.device);
            let render_semaphore = create_semaphore(&configuration.device);
            frame_data.push(FrameData::new(
                command_pool,
                command_buffer,
                swapchain_semaphore,
                render_semaphore,
                fence,
            ));
        }
        Ok(Self {
            configuration,
            frame_data,
            current_frame,
        })
    }

    pub fn draw(&mut self) {
        let current_frame_data = &self.frame_data[self.current_frame];
        let device = &self.configuration.device;
        let fences = vec![self.frame_data[self.current_frame].render_fence];
        unsafe {
            device.wait_for_fences(&fences, true, u64::MAX).unwrap();
            device.reset_fences(&fences).unwrap();
            let next_image = self
                .configuration
                .swapchain_device
                .acquire_next_image(
                    self.configuration.swapchain,
                    u64::MAX,
                    current_frame_data.swapchain_semaphore,
                    current_frame_data.render_fence,
                )
                .unwrap();

            device
                .reset_command_buffer(
                    current_frame_data.command_buffer,
                    CommandBufferResetFlags::empty(),
                )
                .unwrap();
            begin_command_buffer(
                &device,
                current_frame_data.command_buffer,
                CommandBufferUsageFlags::ONE_TIME_SUBMIT,
            );

            image_transition(
                device,
                current_frame_data.command_buffer,
                next_image.0,
                self.configuration.images[self.current_frame],
                ImageLayout::UNDEFINED,
                ImageLayout::GENERAL,
            );

            let flash = (self.current_frame as f32 / 120.0).sin().abs();
            let clear_color_value = ClearColorValue {
                float32: [0.0, 0.0, flash, 1.0],
            };

            let image_subresource_range = image_subresource_range(ImageAspectFlags::COLOR);

            device.cmd_clear_color_image(
                current_frame_data.command_buffer,
                self.configuration.images[self.current_frame],
                ImageLayout::GENERAL,
                &clear_color_value,
                &[image_subresource_range],
            );

            image_transition(
                device,
                current_frame_data.command_buffer,
                next_image.0,
                self.configuration.images[self.current_frame],
                ImageLayout::GENERAL,
                ImageLayout::PRESENT_SRC_KHR,
            );

            device
                .end_command_buffer(current_frame_data.command_buffer)
                .unwrap();
                
            let command_buffers = vec![current_frame_data.command_buffer];
            let wait_semaphores = vec![current_frame_data.swapchain_semaphore];
            let signal_semaphores = vec![current_frame_data.render_semaphore];
            let submit_info = SubmitInfo::default().command_buffers(&command_buffers)
                .wait_semaphores(&wait_semaphores)
                .signal_semaphores(&signal_semaphores);

            device.queue_submit(self.configuration.graphics_queue, &[submit_info], current_frame_data.render_fence).unwrap();
            
            let swapchains = vec![self.configuration.swapchain];
            let indices = vec![next_image.0];
            let present_info = PresentInfoKHR::default().wait_semaphores(&signal_semaphores)
                .swapchains(&swapchains)
                .image_indices(&indices);

            self.configuration.swapchain_device.queue_present(self.configuration.graphics_queue, &present_info).unwrap();

            self.current_frame = (self.current_frame + 1) % MAX_FRAMES as usize;
        }
    }
}
