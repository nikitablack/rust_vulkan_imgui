use crate::app;
use ash::version::DeviceV1_0;

pub fn rebuild_swapchain_data(
    vulkan_data: &mut app::VulkanInitData,
    window: &winit::window::Window,
) -> app::VulkanInitResult {
    let device = vulkan_data.get_device_ref();

    unsafe {
        for &view in &vulkan_data.swapchain_image_views {
            device.destroy_image_view(view, None);
        }
    }
    vulkan_data.swapchain_image_views.clear();

    app::get_surface_capabilities(vulkan_data)?;
    app::get_surface_extent(vulkan_data, window);
    app::create_swapchain(vulkan_data)?;
    app::get_swapchain_images(vulkan_data)?;
    app::get_swapchain_image_views(vulkan_data)?;

    Ok(())
}
