use crate::app;
use ash::version::InstanceV1_0;
use ash::vk;

pub fn get_queue_family(vulkan_data: &mut app::VulkanInitData) -> app::VulkanInitResult {
    let props = unsafe {
        vulkan_data
            .get_instance_ref()
            .get_physical_device_queue_family_properties(vulkan_data.physical_device)
    };

    for (ind, p) in props.iter().enumerate() {
        if p.queue_count > 0 && p.queue_flags.contains(vk::QueueFlags::GRAPHICS) {
            let present_supported = match unsafe {
                vulkan_data
                    .get_surface_loader_ref()
                    .get_physical_device_surface_support(
                        vulkan_data.physical_device,
                        ind as u32,
                        vulkan_data.surface,
                    )
            } {
                Ok(result) => result,
                Err(_) => {
                    return Err(String::from(
                        "failed to get physical device surface_support",
                    ))
                }
            };

            if present_supported {
                vulkan_data.queue_family = ind as u32;
                return Ok(());
            }
        }
    }

    Err(String::from(
        "failed to find graphics queue with present support",
    ))
}
