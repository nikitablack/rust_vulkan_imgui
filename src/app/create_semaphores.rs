use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_semaphores(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    let device = vulkan_data.get_device_ref();

    let create_info = vk::SemaphoreCreateInfo {
        ..Default::default()
    };

    app_data.image_available_semaphore =
        match unsafe { device.create_semaphore(&create_info, None) } {
            Ok(semaphore) => semaphore,
            Err(_) => return Err(String::from("failed to create image available semaphore")),
        };

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        app::set_debug_utils_object_name(
            debug_utils,
            device.handle(),
            app_data.image_available_semaphore,
            String::from("image available semaphore"),
        );
    }

    app_data.graphics_finished_semaphore =
        match unsafe { device.create_semaphore(&create_info, None) } {
            Ok(semaphore) => semaphore,
            Err(_) => return Err(String::from("failed to create graphics finished semaphore")),
        };

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        app::set_debug_utils_object_name(
            debug_utils,
            device.handle(),
            app_data.graphics_finished_semaphore,
            String::from("graphics finished semaphore"),
        );
    }

    Ok(())
}
