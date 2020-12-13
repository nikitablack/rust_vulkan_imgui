use crate::app;

pub fn create_swapchain_loader(vulkan_data: &mut app::VulkanInitData) {
    vulkan_data.swapchain_loader = Some(ash::extensions::khr::Swapchain::new(
        vulkan_data.get_instance_ref(),
        vulkan_data.get_device_ref(),
    ));
}
