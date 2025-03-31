use std::{io::Error, sync::Arc};

use ash::{
    vk::{
        DescriptorImageInfo, DescriptorPool, DescriptorPoolCreateFlags, DescriptorPoolCreateInfo,
        DescriptorPoolResetFlags, DescriptorPoolSize, DescriptorSet, DescriptorSetAllocateInfo,
        DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateFlags,
        DescriptorSetLayoutCreateInfo, DescriptorType, ImageLayout, ShaderStageFlags,
        WriteDescriptorSet,
    },
    Device,
};

use crate::engine::{allocated_image::AllocatedImage, deletion_queue::DeletionQueue};

pub struct DescriptorAllocator {
    pool: DescriptorPool,
}

pub struct PoolSizeRatio {
    descriptor_type: DescriptorType,
    ratio: f32,
}

pub struct DescriptorLayoutBuilder<'a> {
    bindings: Vec<DescriptorSetLayoutBinding<'a>>,
}

pub fn init_descriptors(
    device: Arc<Device>,
    allocated_image: Arc<AllocatedImage>,
    deletion_queue: &mut DeletionQueue,
) -> Result<(Arc<DescriptorAllocator>, DescriptorSetLayout, DescriptorSet), Error> {
    let mut pool_sizes: Vec<PoolSizeRatio> = Vec::new();
    pool_sizes.push(PoolSizeRatio {
        descriptor_type: DescriptorType::STORAGE_IMAGE,
        ratio: 1.0
    });
    let descriptor_allocator = Arc::new(DescriptorAllocator::new(device.clone(), 10, pool_sizes));
    let mut descriptor_layout_builder = DescriptorLayoutBuilder::new();
    descriptor_layout_builder.add_binding(0, DescriptorType::STORAGE_IMAGE);
    let layout = Arc::new(descriptor_layout_builder.build(
        device.clone(),
        ShaderStageFlags::COMPUTE,
        DescriptorSetLayoutCreateFlags::empty(),
    ));
    let descriptor_set = descriptor_allocator.allocate(device.clone(), &[*layout]);
    let descriptor_image_info = vec![DescriptorImageInfo::default()
        .image_layout(ImageLayout::GENERAL)
        .image_view(allocated_image.image_view)];
    let write_descriptor_set = WriteDescriptorSet::default()
        .dst_binding(0)
        .descriptor_count(1)
        .dst_set(descriptor_set)
        .descriptor_type(DescriptorType::STORAGE_IMAGE)
        .image_info(&descriptor_image_info);

    unsafe { device.update_descriptor_sets(&[write_descriptor_set], &[]) };

    let device_clone = device.clone();
    let device_clone2 = device.clone();
    let descriptor_alloc_clone = descriptor_allocator.clone();

    let layout_clone = layout.clone();
    unsafe {
        deletion_queue.enqueue(move || descriptor_alloc_clone.destroy_pool(&device_clone));
        deletion_queue
            .enqueue(move || device_clone2.destroy_descriptor_set_layout(*layout_clone, None));
    }

    Ok((descriptor_allocator, *layout, descriptor_set))
}

impl DescriptorAllocator {
    pub fn new(
        device: Arc<Device>,
        max_sets: u32,
        pool_sizes: Vec<PoolSizeRatio>,
    ) -> DescriptorAllocator {
        let mut descriptor_pool_sizes: Vec<DescriptorPoolSize> = vec![];
        for pool_size in pool_sizes {
            descriptor_pool_sizes.push(
                DescriptorPoolSize::default()
                    .ty(pool_size.descriptor_type)
                    .descriptor_count(pool_size.ratio as u32 * max_sets),
            );
        }
        let create_info = DescriptorPoolCreateInfo::default()
            .max_sets(max_sets)
            .pool_sizes(&descriptor_pool_sizes)
            .flags(DescriptorPoolCreateFlags::empty());

        Self {
            pool: unsafe { device.create_descriptor_pool(&create_info, None).unwrap() },
        }
    }

    pub fn reset_descriptors(&self, device: &Device) {
        unsafe {
            device
                .reset_descriptor_pool(self.pool, DescriptorPoolResetFlags::empty())
                .unwrap()
        }
    }

    pub fn destroy_pool(&self, device: &Device) {
        unsafe { device.destroy_descriptor_pool(self.pool, None) }
    }

    pub fn allocate(&self, device: Arc<Device>, layouts: &[DescriptorSetLayout]) -> DescriptorSet {
        let mut allocate_info = DescriptorSetAllocateInfo::default()
            .descriptor_pool(self.pool)
            .set_layouts(layouts);
        allocate_info.descriptor_set_count = 1;

        unsafe {
            *device
                .allocate_descriptor_sets(&allocate_info)
                .unwrap()
                .get(0)
                .unwrap()
        }
    }
}

impl<'a> DescriptorLayoutBuilder<'a> {
    pub fn new() -> Self {
        Self {
            bindings: Vec::new(),
        }
    }

    pub fn add_binding(&mut self, binding: u32, descriptor_type: DescriptorType) {
        let descriptor_binding = DescriptorSetLayoutBinding::default()
            .binding(binding)
            .descriptor_type(descriptor_type)
            .descriptor_count(1);

        self.bindings.push(descriptor_binding);
    }

    pub fn clear(&mut self) {
        self.bindings.clear();
    }

    pub fn build(
        &mut self,
        device: Arc<Device>,
        shader_stages: ShaderStageFlags,
        flags: DescriptorSetLayoutCreateFlags,
    ) -> DescriptorSetLayout {
        for binding in &mut self.bindings {
            binding.stage_flags = binding.stage_flags | shader_stages
        }

        let descriptor_set_create_info = DescriptorSetLayoutCreateInfo::default()
            .bindings(&self.bindings)
            .flags(flags);

        unsafe {
            device
                .create_descriptor_set_layout(&descriptor_set_create_info, None)
                .unwrap()
        }
    }
}
