use crate::app;

pub fn get_swapchain_images(vulkan_data: &mut app::VulkanInitData) -> app::VulkanInitResult {
    vulkan_data.swapchain_images = match unsafe {
        vulkan_data
            .get_swapchain_loader_ref()
            .get_swapchain_images(vulkan_data.swapchain)
    } {
        Ok(images) => images,
        Err(_) => return Err(String::from("failed to get swapchain images")),
    };

    Ok(())
}
