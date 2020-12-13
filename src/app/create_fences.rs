use crate::app;
use crate::NUM_RESOURCES_IN_FLIGHT;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_fences(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    let device_ref = vulkan_data.get_device_ref();

    let create_info = vk::FenceCreateInfo::builder().flags(vk::FenceCreateFlags::SIGNALED);

    app_data.fences = Vec::with_capacity(NUM_RESOURCES_IN_FLIGHT as usize);

    for i in 0..NUM_RESOURCES_IN_FLIGHT {
        let fence = match unsafe { device_ref.create_fence(&create_info, None) } {
            Ok(fence) => fence,
            Err(_) => return Err(format!("failed to create fence {}", i)),
        };

        app_data.fences.push(fence);
    }

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        for (i, &f) in app_data.fences.iter().enumerate() {
            app::set_debug_utils_object_name(
                debug_utils,
                device_ref.handle(),
                f,
                format!("fence {}", i),
            );
        }
    }

    Ok(())
}
