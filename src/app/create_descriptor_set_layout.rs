use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_descriptor_set_layout(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    let vertex_binding = vk::DescriptorSetLayoutBinding {
        binding: 0,
        descriptor_type: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 1,
        stage_flags: vk::ShaderStageFlags::VERTEX,
        ..Default::default()
    };

    let samplers = [app_data.sampler];
    let font_image_binding = vk::DescriptorSetLayoutBinding::builder()
        .binding(1)
        .descriptor_type(vk::DescriptorType::COMBINED_IMAGE_SAMPLER)
        .descriptor_count(1)
        .stage_flags(vk::ShaderStageFlags::FRAGMENT)
        .immutable_samplers(&samplers)
        .build();

    let bindings = [vertex_binding, font_image_binding];
    let create_info = vk::DescriptorSetLayoutCreateInfo::builder()
        .bindings(&bindings)
        .build();

    app_data.descriptor_set_layout = match unsafe {
        vulkan_data
            .get_device_ref()
            .create_descriptor_set_layout(&create_info, None)
    } {
        Ok(layout) => layout,
        Err(_) => return Err(String::from("failed to create descriptor set layout")),
    };

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        app::set_debug_utils_object_name(
            debug_utils,
            vulkan_data.get_device_ref().handle(),
            app_data.descriptor_set_layout,
            String::from("descriptor set layout"),
        );
    }

    Ok(())
}
