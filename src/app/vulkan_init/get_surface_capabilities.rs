use crate::app;
pub use ash::extensions::khr;
pub use ash::vk;

pub fn get_surface_capabilities(vulkan_data: &mut app::VulkanInitData) -> app::VulkanInitResult {
    vulkan_data.surface_capabilities = match unsafe {
        vulkan_data
            .get_surface_loader_ref()
            .get_physical_device_surface_capabilities(
                vulkan_data.physical_device,
                vulkan_data.surface,
            )
    } {
        Ok(capabilities) => capabilities,
        Err(_) => {
            return Err(String::from(
                "failed to get physical device surface capabilities",
            ))
        }
    };

    Ok(())
}
