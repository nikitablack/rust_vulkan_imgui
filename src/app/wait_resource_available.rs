use crate::app;
use ash::version::DeviceV1_0;

pub fn wait_resource_available(
    app_data: &app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    let device = vulkan_data.get_device_ref();

    let resource_fence = app_data.fences[app_data.resource_index as usize];

    unsafe {
        if let Err(_) = device.wait_for_fences(&[resource_fence], true, u64::MAX) {
            return Err(format!(
                "failed to wait for resource fence {}",
                app_data.resource_index
            ));
        }

        if let Err(_) = device.reset_fences(&[resource_fence]) {
            return Err(format!(
                "failed to reset resource fence {}",
                app_data.resource_index
            ));
        }
    }

    Ok(())
}
