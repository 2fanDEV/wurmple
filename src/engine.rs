use std::{io::Error, sync::Arc};

use allocated_image::AllocatedImage;
use ash::vk::{
    CommandBuffer, CommandBufferResetFlags, CommandBufferUsageFlags, Extent2D, Fence, ImageLayout, PipelineBindPoint, PipelineStageFlags, PresentInfoKHR, Queue, SubmitInfo, SurfaceKHR
};
use command_buffers::{allocate_command_buffer, begin_command_buffer, create_command_pool};
use configuration::{VkConfiguration, MAX_FRAMES};
use data::FrameData;
use egui_renderer::EGUIRenderer;
use image_ops::image_transition;
use sync_objects::{create_fence, create_semaphore};
use winit::window::Window;

mod allocated_image;
mod command_buffers;
mod components;
mod configuration;
mod data;
mod deletion_queue;
mod image_ops;
mod sync_objects;
mod egui_renderer;
mod ui;

pub struct Engine {
    configuration: VkConfiguration,
    frame_data: Vec<FrameData>,
    current_frame: usize,
    egui_renderer: EGUIRenderer,
}
#[allow(dead_code)]
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

        let egui_renderer = EGUIRenderer::new(
            configuration.device.clone(),
            window,
            configuration.graphics_queue.clone(),
            configuration.surface.clone()
        );
        Ok(Self {
            configuration,
            frame_data,
            current_frame,
            egui_renderer
        })
    }

    fn draw_background(&self, image: Arc<AllocatedImage>, command_buffer: CommandBuffer) {
        unsafe {
            self.configuration.device.cmd_bind_pipeline(
                command_buffer,
                PipelineBindPoint::COMPUTE,
                self.configuration.compute_pipeline,
            );
            self.configuration.device.cmd_bind_descriptor_sets(
                command_buffer,
                PipelineBindPoint::COMPUTE,
                self.configuration.compute_pipeline_layout,
                0,
                &[self.configuration.descriptor_set],
                &[],
            );

            self.configuration.device.cmd_dispatch(
                command_buffer,
                image.extent.width.div_ceil(16),
                image.extent.height.div_ceil(16),
                1,
            );
        }
    }

    pub fn draw(&mut self) {
        let current_frame_data = &self.frame_data[self.current_frame];
        let command_buffer = current_frame_data.command_buffer;
        let device = &self.configuration.device;
        let fences = vec![current_frame_data.render_fence];
        let allocated_image = self.configuration.allocated_image.image;
        let alloc_extent = Extent2D {
            width: self.configuration.allocated_image.extent.width,
            height: self.configuration.allocated_image.extent.height,
        };

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
                    Fence::null(),
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
                self.configuration.indices.graphics_q_idx.unwrap(),
                allocated_image,
                ImageLayout::UNDEFINED,
                ImageLayout::GENERAL,
            );

            self.draw_background(self.configuration.allocated_image.clone(), command_buffer);

            image_transition(
                device,
                current_frame_data.command_buffer,
                self.configuration.indices.graphics_q_idx.unwrap(),
                allocated_image,
                ImageLayout::GENERAL,
                ImageLayout::TRANSFER_SRC_OPTIMAL,
            );

            image_transition(
                device,
                current_frame_data.command_buffer,
                self.configuration.indices.graphics_q_idx.unwrap(),
                self.configuration.images[next_image.0 as usize],
                ImageLayout::UNDEFINED,
                ImageLayout::TRANSFER_DST_OPTIMAL,
            );

            image_ops::copy_image_to_image(
                device,
                current_frame_data.command_buffer,
                allocated_image,
                self.configuration.images[next_image.0 as usize],
                alloc_extent,
                self.configuration.extent,
            );

            image_transition(
                device,
                current_frame_data.command_buffer,
                self.configuration.indices.graphics_q_idx.unwrap(),
                self.configuration.images[next_image.0 as usize],
                ImageLayout::TRANSFER_DST_OPTIMAL,
                ImageLayout::PRESENT_SRC_KHR,
            );

            device
                .end_command_buffer(current_frame_data.command_buffer)
                .unwrap();

            let command_buffers = vec![current_frame_data.command_buffer];
            let wait_semaphores = vec![current_frame_data.swapchain_semaphore];
            let signal_semaphores = vec![current_frame_data.render_semaphore];
            let dst_stage_mask = vec![PipelineStageFlags::ALL_COMMANDS];
            let submit_info = SubmitInfo::default()
                .command_buffers(&command_buffers)
                .wait_semaphores(&wait_semaphores)
                .signal_semaphores(&signal_semaphores)
                .wait_dst_stage_mask(&dst_stage_mask);

            device
                .queue_submit(
                    *self.configuration.graphics_queue,
                    &[submit_info],
                    current_frame_data.render_fence,
                )
                .unwrap();

            let swapchains = vec![self.configuration.swapchain];
            let indices = vec![next_image.0];
            let present_info = PresentInfoKHR::default()
                .wait_semaphores(&signal_semaphores)
                .swapchains(&swapchains)
                .image_indices(&indices);

            self.configuration
                .swapchain_device
                .queue_present(*self.configuration.graphics_queue, &present_info)
                .unwrap();
            self.current_frame = (self.current_frame + 1) % MAX_FRAMES as usize;
        }
    }
    
    pub fn get_graphics_queue(&self) -> Arc<Queue> {
        self.configuration.graphics_queue.clone()
    }

    pub fn get_surface(&self) -> Arc<SurfaceKHR> {
        self.configuration.surface.clone()
    }

    pub fn cleanup(&self) {
        unsafe {
            let device = &self.configuration.device;
            for i in 0..MAX_FRAMES {
                let frame = &self.frame_data[i as usize];
                device.destroy_command_pool(frame.command_pool, None);
                device.destroy_fence(frame.render_fence, None);
                device.destroy_semaphore(frame.render_semaphore, None);
                device.destroy_semaphore(frame.swapchain_semaphore, None);
            }
            self.configuration.cleanup();
        }
    }
}
