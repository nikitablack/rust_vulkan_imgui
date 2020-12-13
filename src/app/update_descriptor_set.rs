use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn update_descriptor_set(
    app_data: &app::AppData,
    vulkan_data: &app::VulkanInitData,
    resource_index: usize,
) {
    let device = vulkan_data.get_device_ref();
    let set = app_data.descriptor_sets[resource_index];

    let buffer_info = vk::DescriptorBufferInfo {
        buffer: app_data.vertex_mem_buffers[resource_index].buffer,
        offset: 0,
        range: vk::WHOLE_SIZE,
    };

    let image_info = vk::DescriptorImageInfo {
        sampler: vk::Sampler::null(),
        image_view: app_data.font_mem_image.view,
        image_layout: vk::ImageLayout::SHADER_READ_ONLY_OPTIMAL,
    };

    let infos = [buffer_info];
    let buffer_desc_write = vk::WriteDescriptorSet::builder()
        .dst_set(set)
        .dst_binding(0)
        .descriptor_type(vk::DescriptorType::STORAGE_BUFFER)
        .buffer_info(&infos)
        .build();

    let infos = [image_info];
    let texture_desc_write = vk::WriteDescriptorSet::builder()
        .dst_set(set)
        .dst_binding(1)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .image_info(&infos)
        .build();

    unsafe {
        device.update_descriptor_sets(&[buffer_desc_write, texture_desc_write], &[]);
    }
}

pub fn update_descriptor_sets(app_data: &app::AppData, vulkan_data: &app::VulkanInitData) {
    for i in 0..app_data.descriptor_sets.len() {
        update_descriptor_set(app_data, vulkan_data, i);
    }
}
