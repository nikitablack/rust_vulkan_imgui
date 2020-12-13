use crate::app;

pub fn create_surface(
    vulkan_data: &mut app::VulkanInitData,
    window: &winit::window::Window,
) -> app::VulkanInitResult {
    vulkan_data.surface = match unsafe {
        ash_window::create_surface(
            vulkan_data.get_entry_ref(),
            vulkan_data.get_instance_ref(),
            window,
            None,
        )
    } {
        Ok(surface) => surface,
        Err(_) => return Err(String::from("failed to create surface")),
    };

    Ok(())
}
