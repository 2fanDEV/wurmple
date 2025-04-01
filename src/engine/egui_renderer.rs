use std::sync::Arc;

use ash::{
    vk::{
        CommandBuffer, CommandPool, Fence, Queue, RenderPass, RenderPassBeginInfo, SubpassContents,
        SurfaceKHR,
    },
    Device,
};
use egui::{epaint::Primitive, ClippedPrimitive, FullOutput, RawInput};
use winit::window::{self, Window};

use super::{
    command_buffers::{allocate_command_buffer, create_command_pool},
    components::{self, EGUIConfig},
    deletion_queue::{self, DeletionQueue},
    sync_objects::create_fence,
};

trait Renderer {
    fn draw(&self);
}

pub struct EGUIRenderer {
    configuration: components::EGUIConfig,
    fence: Fence,
    command_pool: CommandPool,
    command_buffer: CommandBuffer,
    deletion_queue: DeletionQueue,
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
                parameter.surface,
            ),
            deletion_queue,
        }
    }

    fn begin_render_pass(&self) {
        let begin_info = RenderPassBeginInfo::default().render_pass(self.configuration.render_pass);

        unsafe {
            self.configuration.device.cmd_begin_render_pass(
                self.command_buffer,
                &begin_info,
                SubpassContents::default(),
            )
        };
    }

    fn end_render_pass(&self) {
        unsafe {
            self.configuration
                .device
                .cmd_end_render_pass(self.command_buffer)
        };
    }

    fn begin_frame(&self) {
        let raw_input = RawInput::default();
        self.configuration._context.begin_pass(raw_input);
    }

    fn end_frame(&self) -> FullOutput {
        self.configuration._context.end_pass()
    }
}

impl Renderer for EGUIRenderer {
    fn draw(&self) {
        self.begin_render_pass();

        self.begin_frame();
        let output = self.end_frame();
        let clipped_primitives: Vec<ClippedPrimitive> = self.configuration._context.tessellate(
            output.shapes,
            self.configuration._context.pixels_per_point(),
        );


        for ClippedPrimitive {
            primitive,
            clip_rect,
        } in clipped_primitives
        {
            match primitive {
                Primitive::Mesh(mesh) => {
                    let indices = mesh.indices;
                    let vertices = mesh.vertices;
                }
                Primitive::Callback(_) => todo!(),
            }
        }
        self.end_render_pass();
    }
}
