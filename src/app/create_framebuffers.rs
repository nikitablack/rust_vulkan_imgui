use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_framebuffers(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    debug_assert!(app_data.framebuffers.is_empty());

    let device = vulkan_data.get_device_ref();

    app_data
        .framebuffers
        .reserve(vulkan_data.swapchain_image_views.len());

    for (i, &view) in vulkan_data.swapchain_image_views.iter().enumerate() {
        let attachments = [view];

        let create_info = vk::FramebufferCreateInfo::builder()
            .render_pass(app_data.render_pass)
            .attachments(&attachments)
            .width(vulkan_data.surface_extent.width)
            .height(vulkan_data.surface_extent.height)
            .layers(1);

        let framebuffer = match unsafe { device.create_framebuffer(&create_info, None) } {
            Ok(fb) => fb,
            Err(_) => return Err(format!("failed to create framebuffer {}", i)),
        };

        app_data.framebuffers.push(framebuffer);

        if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
            app::set_debug_utils_object_name(
                debug_utils,
                device.handle(),
                framebuffer,
                format!("framebuffer {}", i),
            );
        }
    }

    Ok(())
}
