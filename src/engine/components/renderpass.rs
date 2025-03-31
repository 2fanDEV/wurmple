use std::io::Error;

use ash::{
    vk::{
        AttachmentDescription, AttachmentLoadOp, AttachmentReference, AttachmentStoreOp, Format,
        ImageLayout, PipelineBindPoint, RenderPass, RenderPassCreateInfo, SampleCountFlags,
        SubpassDescription,
    },
    Device,
};


pub fn allocate_render_pass(device: &Device, format: &Format) -> Result<RenderPass, Error> {
    let color_attachment = create_attachment(*format);
    let color_attachment_ref = vec![create_attachment_ref()];
    let subpass_description = create_subpass_description(&color_attachment_ref);
    Ok(unsafe {
        device
            .create_render_pass(
                &render_pass_create_info(&[color_attachment], &[subpass_description]),
                None,
            )
            .unwrap()
    })
}

fn render_pass_create_info<'a>(
    attachments: &'a [AttachmentDescription],
    subpass_description: &'a [SubpassDescription],
) -> RenderPassCreateInfo<'a> {
    RenderPassCreateInfo::default()
        .attachments(attachments)
        .subpasses(subpass_description)
}

fn create_attachment(image_format: Format) -> AttachmentDescription {
    AttachmentDescription::default()
        .format(image_format)
        .samples(SampleCountFlags::TYPE_1)
        .load_op(AttachmentLoadOp::CLEAR)
        .store_op(AttachmentStoreOp::STORE)
        .stencil_load_op(AttachmentLoadOp::DONT_CARE)
        .stencil_store_op(AttachmentStoreOp::DONT_CARE)
        .initial_layout(ImageLayout::UNDEFINED)
        .final_layout(ImageLayout::PRESENT_SRC_KHR)
}

fn create_attachment_ref() -> AttachmentReference {
    AttachmentReference::default().layout(ImageLayout::COLOR_ATTACHMENT_OPTIMAL)
}

fn create_subpass_description(color_attachments: &[AttachmentReference]) -> SubpassDescription<'_> {
    SubpassDescription::default()
        .color_attachments(color_attachments)
        .pipeline_bind_point(PipelineBindPoint::GRAPHICS)
        .color_attachments(color_attachments)
}
