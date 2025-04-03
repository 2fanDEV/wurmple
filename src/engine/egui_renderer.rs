use std::sync::Arc;

use ash::{
    vk::{
        BufferCreateFlags, BufferCreateInfo, BufferUsageFlags, ClearColorValue,
        ClearDepthStencilValue, ClearValue, CommandBuffer, CommandBufferUsageFlags, CommandPool,
        Extent2D, Fence, Framebuffer, FramebufferCreateInfo, MemoryPropertyFlags, MemoryType,
        Offset2D, Queue, Rect2D, RenderPassBeginInfo, SharingMode, SubpassContents, SurfaceKHR,
    },
    Device,
};
use derive_setters::Setters;
use egui::{ClippedPrimitive, FullOutput, RawInput};
use log::debug;
use vk_mem::{Alloc, Allocation, AllocationCreateInfo, Allocator, MemoryUsage};
use winit::window::Window;

use crate::engine::command_buffers::{allocate_command_buffer, create_command_pool};
use crate::engine::components::EGUIConfig;
use crate::engine::deletion_queue::DeletionQueue;
use crate::engine::sync_objects::*;

use super::{allocated_image::AllocatedImage, command_buffers};

pub trait Renderer {
    fn draw(&self, allocated_image: Arc<AllocatedImage>) -> CommandBuffer;
    fn buffer_allocation(&mut self, allocator: Arc<Allocator>, extent: Extent2D) -> Allocation;
}

#[derive(Setters)]
pub struct EGUIRenderer {
    configuration: EGUIConfig,
    fence: Fence,
    command_pool: CommandPool,
    command_buffer: CommandBuffer,
    deletion_queue: DeletionQueue,
    render_information: RenderInformation,
}

#[derive(Default)]
pub struct RenderInformation {
    pub vertices: Vec<egui::epaint::Vertex>,
    pub indices: Vec<u32>,
    pub extent: ash::vk::Extent2D,
}

pub struct ConfigurationParameter<'a> {
    pub device: Arc<Device>,
    pub window: &'a Window,
    pub gfx_queue: Arc<Queue>,
    pub surface: Arc<SurfaceKHR>,
}

impl EGUIRenderer {
    pub fn new(parameter: ConfigurationParameter, gfx_queue_family_idx: u32) -> EGUIRenderer {
        let fence = create_fence(&parameter.device);
        let command_pool = create_command_pool(&parameter.device, gfx_queue_family_idx);
        let command_buffer = allocate_command_buffer(&parameter.device, command_pool);
        let mut deletion_queue = DeletionQueue::new();
        let device = parameter.device.clone();
        deletion_queue.enqueue(move || unsafe { device.destroy_command_pool(command_pool, None) });
        Self {
            fence,
            command_pool,
            command_buffer,
            configuration: EGUIConfig::new(
                parameter.device,
                parameter.window,
                parameter.gfx_queue,
                gfx_queue_family_idx,
                parameter.surface,
            ),
            deletion_queue,
            render_information: Default::default(),
        }
    }

    fn begin_render_pass(&self, allocated_image: Arc<AllocatedImage>) {
        unsafe {
            let image_views = vec![allocated_image.image_view];
            let framebuffer_create_info = FramebufferCreateInfo::default()
                .width(self.render_information.extent.width)
                .height(self.render_information.extent.height)
                .render_pass(self.configuration.render_pass)
                .attachments(&image_views)
                .layers(1);
            let framebuffer = self
                .configuration
                .device
                .create_framebuffer(&framebuffer_create_info, None)
                .unwrap();
            let render_pass_info = RenderPassBeginInfo::default()
                .render_area(Rect2D {
                    offset: Offset2D::default(),
                    extent: self.render_information.extent,
                })
                .render_pass(self.configuration.render_pass)
                .framebuffer(framebuffer)
                .clear_values(&[ClearValue {
                    color: ClearColorValue {
                        float32: [0.0, 0.0, 0.0, 1.0],
                    },
                }]);
            
            self.configuration.device.cmd_begin_render_pass(
                self.command_buffer,
                &render_pass_info,
                SubpassContents::default(),
            );
        };
    }

    fn end_render_pass(&self) {
        unsafe {
            self.configuration
                .device
                .cmd_end_render_pass(self.command_buffer)
        };
    }

}

impl Renderer for EGUIRenderer {
    fn draw(&self, allocated_image: Arc<AllocatedImage>) -> CommandBuffer {
        let device = self.configuration.device.clone();
        command_buffers::begin_command_buffer(
            &device,
            self.command_buffer,
            CommandBufferUsageFlags::ONE_TIME_SUBMIT,
        );
        self.begin_render_pass(allocated_image);
        //       unsafe { self.configuration.device.cmd_draw_indexed(self.command_buffer, index_count, instance_count, first_index, vertex_offset, first_instance) };
        self.end_render_pass();
        unsafe { device.end_command_buffer(self.command_buffer) };
        self.command_buffer
    }

    fn buffer_allocation(&mut self, allocator: Arc<Allocator>, extent: Extent2D) -> Allocation {
        let context = &self.configuration.context;
        let device = &self.configuration.device;
        let raw_input = RawInput::default();
        debug!("{:?}", raw_input);
        context.begin_pass(raw_input);
        egui::SidePanel::new(egui::panel::Side::Right, "Side Panel").show(
            &self.configuration.context,
            |ui| {
                ui.label("Hello World!");
                let hello_button = ui.button("Hellooo");
                if hello_button.clicked() {}
            },
        );
        let output = context.end_pass();
        let clipped_primitives = self
            .configuration
            .context
            .tessellate(output.shapes, context.pixels_per_point());
        for ClippedPrimitive {
            primitive,
            clip_rect,
        } in clipped_primitives
        {
            match primitive {
                egui::epaint::Primitive::Mesh(mesh) => {
                    self.render_information = RenderInformation {
                        vertices: mesh.vertices,
                        indices: mesh.indices,
                        extent,
                    };
                }
                egui::epaint::Primitive::Callback(paint_callback) => todo!(),
            }
        }

        let queue_family_indices = [self.configuration.gfx_queue_family_index];
        let vertex_buffer_create_info = BufferCreateInfo::default()
            .queue_family_indices(&queue_family_indices)
            .sharing_mode(SharingMode::EXCLUSIVE)
            .usage(BufferUsageFlags::VERTEX_BUFFER)
            .size(self.render_information.vertices.len() as u64);
        let mut allocation_create_info = AllocationCreateInfo::default();
        allocation_create_info.usage = MemoryUsage::Unknown;
        allocation_create_info.memory_type_bits = MemoryPropertyFlags::HOST_VISIBLE.as_raw();
        unsafe {
            let (mut buffer, mut vertex_buffer_allocation) = allocator
                .create_buffer(&vertex_buffer_create_info, &allocation_create_info)
                .unwrap();
            let data = allocator.map_memory(&mut vertex_buffer_allocation);
            vertex_buffer_allocation
        }
    }
}
