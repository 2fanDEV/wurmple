use ash::{
    vk::{
        DescriptorPool, DescriptorPoolCreateFlags, DescriptorPoolCreateInfo, DescriptorPoolResetFlags, DescriptorPoolSize, DescriptorSet, DescriptorSetAllocateInfo, DescriptorSetLayout, DescriptorSetLayoutBinding, DescriptorSetLayoutCreateFlags, DescriptorSetLayoutCreateInfo, DescriptorType, ShaderStageFlags
    },
    Device,
};

struct DescriptorAllocator {
    pool: DescriptorPool,
}

struct PoolSizeRatio {
    descriptor_type: DescriptorType,
    ratio: f32,
}

struct DescriptorLayoutBuilder<'a> {
    bindings: Vec<DescriptorSetLayoutBinding<'a>>,
}

impl DescriptorAllocator {
    fn init(device: &Device, max_sets: u32, pool_sizes: Vec<PoolSizeRatio>) -> DescriptorAllocator {
        let mut descriptor_pool_sizes : Vec<DescriptorPoolSize> = vec![];
        for pool_size in pool_sizes {
            descriptor_pool_sizes.push(
                DescriptorPoolSize::default().ty(pool_size.descriptor_type).descriptor_count(pool_size.ratio as u32 * max_sets)
            );
        }
        let create_info = DescriptorPoolCreateInfo::default().max_sets(max_sets)
            .pool_sizes(&descriptor_pool_sizes)
            .flags(DescriptorPoolCreateFlags::empty());
        
        Self {
            pool: unsafe { device.create_descriptor_pool(&create_info, None).unwrap()}
        }
    }

    fn reset_descriptors(&self, device: &Device) {
        unsafe { device.reset_descriptor_pool(self.pool, DescriptorPoolResetFlags::empty()).unwrap() }
    }

    fn destroy_pool(&self, device: &Device) {
        unsafe { device.destroy_descriptor_pool(self.pool, None) }
    }
    
    fn allocate(&self, device: &Device, layouts: &[DescriptorSetLayout]) -> DescriptorSet {
        let mut allocate_info = DescriptorSetAllocateInfo::default().descriptor_pool(self.pool)
            .set_layouts(layouts);
        allocate_info.descriptor_set_count = 1;

        unsafe { *device.allocate_descriptor_sets(&allocate_info).unwrap().get(0).unwrap() }
    }

}

impl<'a> DescriptorLayoutBuilder<'a> {
    fn add_binding(&mut self, binding: u32, descriptor_type: DescriptorType) {
        let descriptor_binding = DescriptorSetLayoutBinding::default()
            .binding(binding)
            .descriptor_type(descriptor_type);

        self.bindings.push(descriptor_binding);
    }

    fn clear(&mut self) {
        self.bindings.clear();
    }

    fn build(
        &mut self,
        device: &Device,
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
