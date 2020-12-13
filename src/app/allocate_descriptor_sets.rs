use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn allocate_descriptor_sets(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    debug_assert!(app_data.descriptor_sets.is_empty());

    let device_ref = vulkan_data.get_device_ref();

    let layouts = [app_data.descriptor_set_layout; crate::NUM_RESOURCES_IN_FLIGHT as usize];

    let alloc_info = vk::DescriptorSetAllocateInfo::builder()
        .descriptor_pool(app_data.descriptor_pool)
        .set_layouts(&layouts)
        .build();

    app_data.descriptor_sets = match unsafe { device_ref.allocate_descriptor_sets(&alloc_info) } {
        Ok(sets) => sets,
        Err(_) => return Err(String::from("failed to allocate descriptor sets")),
    };

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        for (i, &set) in app_data.descriptor_sets.iter().enumerate() {
            app::set_debug_utils_object_name(
                debug_utils,
                device_ref.handle(),
                set,
                format!("descriptor set {}", i),
            );
        }
    }

    Ok(())
}
