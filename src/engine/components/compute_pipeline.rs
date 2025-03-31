use std::{io::Error, sync::Arc};

use ash::{
    vk::{
        ComputePipelineCreateInfo, DescriptorSetLayout, Pipeline, PipelineCache, PipelineLayout, PipelineLayoutCreateInfo, PipelineShaderStageCreateFlags, PipelineShaderStageCreateInfo, ShaderStageFlags
    },
    Device,
};
use log::debug;

use crate::engine::deletion_queue::DeletionQueue;

use super::util::load_shader_module;
pub fn init_background_pipelines(
    device: Arc<Device>,
    layouts: &[DescriptorSetLayout],
    deletion_queue: &mut DeletionQueue,
) -> Result<(PipelineLayout, Pipeline), Error> {

    let create_info = PipelineLayoutCreateInfo::default().set_layouts(layouts);
    let pipeline_layout = unsafe { device.create_pipeline_layout(&create_info, None).unwrap() };
    let shader_module = load_shader_module("shaders/shader.spv", device.clone()).unwrap();
    let shader_stage_info = PipelineShaderStageCreateInfo::default()
        .module(shader_module)
        .name(c"main")
        .stage(ShaderStageFlags::COMPUTE);

    let pipeline_create_info = vec![ComputePipelineCreateInfo::default()
        .stage(shader_stage_info)
        .layout(pipeline_layout)];
    let device_clone = device.clone();
    let device_clone2 = device.clone();
    let device_clone3 = device.clone();
    unsafe {
        deletion_queue.enqueue(move || device_clone.destroy_shader_module(shader_module, None));
        deletion_queue.enqueue(move || device_clone2.destroy_pipeline_layout(pipeline_layout, None));
        debug!("BEFORE CREATION");
        let pipeline = *device
            .create_compute_pipelines(PipelineCache::null(), &pipeline_create_info, None)
            .unwrap().get(0).unwrap();
        debug!("AFTER CREATION");
        deletion_queue.enqueue(move|| device_clone3.destroy_pipeline(pipeline,None));
        Ok((pipeline_layout, pipeline))
    }
}
