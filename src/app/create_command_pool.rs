use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_command_pool(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    let create_info = vk::CommandPoolCreateInfo::builder()
        .flags(vk::CommandPoolCreateFlags::RESET_COMMAND_BUFFER)
        .queue_family_index(vulkan_data.queue_family);

    app_data.command_pool = match unsafe {
        vulkan_data
            .get_device_ref()
            .create_command_pool(&create_info, None)
    } {
        Ok(pool) => pool,
        Err(_) => return Err(String::from("failed to create command pool")),
    };

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        app::set_debug_utils_object_name(
            debug_utils,
            vulkan_data.get_device_ref().handle(),
            app_data.command_pool,
            String::from("command pool"),
        );
    }

    Ok(())
}
