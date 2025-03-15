use ash::{
    vk::{
        AccessFlags, CommandBuffer, DependencyFlags, DependencyInfo, Image, ImageAspectFlags,
        ImageLayout, ImageMemoryBarrier, ImageSubresourceRange, PipelineStageFlags,
        REMAINING_ARRAY_LAYERS, REMAINING_MIP_LEVELS,
    },
    Device,
};
use log::{Level, LevelFilter};

pub fn image_transition(
    device: &Device,
    command_buffer: CommandBuffer,
    index: u32,
    image: Image,
    current_image_layout: ImageLayout,
    new_image_layout: ImageLayout,
) {
    let sub_resource_range = image_subresource_range(
        if new_image_layout.eq(&ImageLayout::DEPTH_ATTACHMENT_OPTIMAL) {
            ImageAspectFlags::DEPTH
        } else {
            ImageAspectFlags::COLOR
        },
    );

    let image_memory_barrier = ImageMemoryBarrier::default()
        .src_access_mask(AccessFlags::MEMORY_WRITE)
        .dst_access_mask(AccessFlags::MEMORY_WRITE | AccessFlags::MEMORY_READ)
        .old_layout(current_image_layout)
        .new_layout(new_image_layout)
        .dst_queue_family_index(index)
        .image(image)
        .subresource_range(sub_resource_range);

    unsafe {
        device.cmd_pipeline_barrier(
            command_buffer,
            PipelineStageFlags::ALL_COMMANDS,
            PipelineStageFlags::ALL_COMMANDS,
            DependencyFlags::empty(),
            &[],
            &[],
            &[image_memory_barrier],
        )
    };
}

pub fn image_subresource_range(aspect_flag: ImageAspectFlags) -> ImageSubresourceRange {
     ImageSubresourceRange::default()
        .aspect_mask(aspect_flag)
        .base_mip_level(0)
        .level_count(REMAINING_MIP_LEVELS)
        .base_array_layer(0)
        .layer_count(REMAINING_ARRAY_LAYERS)
}
