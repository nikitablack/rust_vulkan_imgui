use crate::app;
use ash::vk;

pub fn create_swapchain(vulkan_data: &mut app::VulkanInitData) -> app::VulkanInitResult {
    let capabilities_ref = &vulkan_data.surface_capabilities;

    let image_count = std::cmp::min(
        std::cmp::max(capabilities_ref.min_image_count, 3),
        capabilities_ref.max_image_count,
    );

    let old_swapchain = vulkan_data.swapchain;

    let create_info = vk::SwapchainCreateInfoKHR::builder()
        .surface(vulkan_data.surface)
        .min_image_count(image_count)
        .image_format(vulkan_data.surface_format.format)
        .image_color_space(vulkan_data.surface_format.color_space)
        .image_extent(vulkan_data.surface_extent)
        .image_array_layers(1)
        .image_usage(vk::ImageUsageFlags::COLOR_ATTACHMENT)
        .image_sharing_mode(vk::SharingMode::EXCLUSIVE)
        .pre_transform(capabilities_ref.current_transform)
        .composite_alpha(vk::CompositeAlphaFlagsKHR::OPAQUE)
        .present_mode(vulkan_data.present_mode)
        .clipped(true)
        .old_swapchain(old_swapchain);

    vulkan_data.swapchain = match unsafe {
        vulkan_data
            .get_swapchain_loader_ref()
            .create_swapchain(&create_info, None)
    } {
        Ok(swapchain) => swapchain,
        Err(_) => return Err(String::from("failed to create swapchain")),
    };

    if old_swapchain == vk::SwapchainKHR::null() {
        unsafe {
            vulkan_data
                .get_swapchain_loader_ref()
                .destroy_swapchain(old_swapchain, None)
        };
    }

    Ok(())
}
