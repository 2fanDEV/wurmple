use std::io::{Cursor, Error};

use ash::{
    util::read_spv,
    vk::{
        BlendFactor, BlendOp, ColorComponentFlags, CullModeFlags, DynamicState, Extent2D, FrontFace, Offset2D, Pipeline, PipelineColorBlendAttachmentState, PipelineDynamicStateCreateInfo, PipelineInputAssemblyStateCreateInfo, PipelineMultisampleStateCreateInfo, PipelineRasterizationStateCreateInfo, PipelineVertexInputStateCreateInfo, PipelineViewportStateCreateInfo, PolygonMode, PrimitiveTopology, Rect2D, SampleCountFlags, ShaderModule, ShaderModuleCreateInfo, Viewport
    },
    Device,
};

use crate::util::read_file_as_cursor;

pub fn load_shader_module(file_path: &str, device: &Device) -> Result<ShaderModule, Error> {
    let code = read_spv(&mut read_file_as_cursor(file_path)).unwrap();
    let create_info = ShaderModuleCreateInfo::default().code(&code);
    Ok(unsafe { device.create_shader_module(&create_info, None).unwrap() })
}

pub fn create_graphics_pipeline(device: &Device, extent: Extent2D) -> Result<Pipeline, Error> {
    let dynamic_states_create_info =
        dynamic_states(&[DynamicState::VIEWPORT, DynamicState::SCISSOR]);
    let vertex_input_state = PipelineVertexInputStateCreateInfo::default();
    let input_assembly_state = PipelineInputAssemblyStateCreateInfo::default()
        .topology(PrimitiveTopology::TRIANGLE_LIST)
        .primitive_restart_enable(false);
    let viewports = [create_viewport(extent)];
    let scissors = [create_scissor(extent)];
    let viewport_state = create_pipeline_viewport_state(&viewports, &scissors);

}

fn dynamic_states<'a>(states: &'a [DynamicState]) -> PipelineDynamicStateCreateInfo<'a> {
    PipelineDynamicStateCreateInfo::default().dynamic_states(states)
}

fn create_viewport(extent: Extent2D) -> Viewport {
    Viewport::default()
        .x(0.0)
        .y(0.0)
        .width(extent.width as f32)
        .height(extent.height as f32)
        .min_depth(0.0)
        .max_depth(1.0)
}

fn create_scissor(extent: Extent2D) -> Rect2D {
    Rect2D::default()
        .offset(Offset2D::default().x(0).y(0))
        .extent(extent)
}

fn create_pipeline_viewport_state<'a>(
    viewports: &'a [Viewport],
    scissors: &'a [Rect2D],
) -> PipelineViewportStateCreateInfo<'a> {
    PipelineViewportStateCreateInfo::default()
        .scissors(scissors)
        .viewports(viewports)
}

fn create_rasterizer_state() -> PipelineRasterizationStateCreateInfo<'static> {
    PipelineRasterizationStateCreateInfo::default()
        .depth_bias_enable(false)
        .rasterizer_discard_enable(false)
        .line_width(1.0)
        .polygon_mode(PolygonMode::FILL)
        .cull_mode(CullModeFlags::BACK)
        .front_face(FrontFace::CLOCKWISE)
        .depth_bias_constant_factor(0.0)
        .depth_bias_slope_factor(0.0)
        .depth_bias_clamp(0.0)
}

fn multisampling_state() -> PipelineMultisampleStateCreateInfo<'static> {
    PipelineMultisampleStateCreateInfo::default()
        .sample_shading_enable(false)
        .rasterization_samples(SampleCountFlags::TYPE_1)
        .min_sample_shading(1.0)
        .alpha_to_one_enable(false)
        .alpha_to_coverage_enable(false)
}

fn color_blending_state() -> PipelineColorBlendAttachmentState {
    PipelineColorBlendAttachmentState::default()
        .color_write_mask(ColorComponentFlags::R | ColorComponentFlags::G | ColorComponentFlags::B | ColorComponentFlags::A)
        .blend_enable(false)
        .src_color_blend_factor(BlendFactor::ONE)
        .dst_color_blend_factor(BlendFactor::ZERO)
        .color_blend_op(BlendOp::ADD)
        .src_alpha_blend_factor(BlendFactor::ONE)
        .dst_color_blend_factor(BlendFactor::ZERO)
        .alpha_blend_op(BlendOp::ADD)
}

