use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_descriptor_pool(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    let pool_size_1 = vk::DescriptorPoolSize {
        ty: vk::DescriptorType::STORAGE_BUFFER,
        descriptor_count: 1,
    };

    let pool_size_2 = vk::DescriptorPoolSize {
        ty: vk::DescriptorType::COMBINED_IMAGE_SAMPLER,
        descriptor_count: 1,
    };

    let sizes = [pool_size_1, pool_size_2];
    let create_info = vk::DescriptorPoolCreateInfo::builder()
        .max_sets(10)
        .pool_sizes(&sizes)
        .build();

    app_data.descriptor_pool = match unsafe {
        vulkan_data
            .get_device_ref()
            .create_descriptor_pool(&create_info, None)
    } {
        Ok(p) => p,
        Err(_) => return Err(String::from("failed to create descriptor pool")),
    };

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        app::set_debug_utils_object_name(
            debug_utils,
            vulkan_data.get_device_ref().handle(),
            app_data.descriptor_pool,
            String::from("descriptor pool"),
        );
    }

    Ok(())
}
