use crate::app;
use ash::vk;

pub fn get_present_mode(vulkan_data: &mut app::VulkanInitData) -> app::VulkanInitResult {
    let modes = match unsafe {
        vulkan_data
            .get_surface_loader_ref()
            .get_physical_device_surface_present_modes(
                vulkan_data.physical_device,
                vulkan_data.surface,
            )
    } {
        Ok(formats) => formats,
        Err(_) => {
            return Err(String::from(
                "failed to get physical device surface present modes",
            ));
        }
    };

    if modes.is_empty() {
        return Err(String::from(
            "failed to get physical device surface present modes",
        ));
    }

    if modes.contains(&vk::PresentModeKHR::MAILBOX) {
        vulkan_data.present_mode = vk::PresentModeKHR::MAILBOX;
        return Ok(());
    }

    if modes.contains(&vk::PresentModeKHR::IMMEDIATE) {
        vulkan_data.present_mode = vk::PresentModeKHR::IMMEDIATE;
        return Ok(());
    }

    vulkan_data.present_mode = vk::PresentModeKHR::FIFO;

    Ok(())
}
