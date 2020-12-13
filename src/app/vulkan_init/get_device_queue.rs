use crate::app;
use ash::version::DeviceV1_0;

pub fn get_device_queue(vulkan_data: &mut app::VulkanInitData) {
    vulkan_data.queue = unsafe {
        vulkan_data
            .get_device_ref()
            .get_device_queue(vulkan_data.queue_family, 0)
    };
}
