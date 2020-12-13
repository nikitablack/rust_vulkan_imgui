use crate::app;
use ash::version::InstanceV1_0;

pub fn get_physical_device_properties(vulkan_data: &mut app::VulkanInitData) {
    vulkan_data.device_properties = unsafe {
        vulkan_data
            .get_instance_ref()
            .get_physical_device_properties(vulkan_data.physical_device)
    };
}
