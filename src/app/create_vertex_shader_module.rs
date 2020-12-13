use crate::app;

pub fn create_vertex_shader_module(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    let device = vulkan_data.get_device_ref();

    app_data.vertex_shader_module = match app::create_shader_module(
        device,
        std::path::Path::new("resources/vertex_shader.vert.spv"),
    ) {
        Ok(module) => module,
        Err(msg) => return Err(msg),
    };

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        app::set_debug_utils_object_name(
            debug_utils,
            device.handle(),
            app_data.vertex_shader_module,
            String::from("vertex shader module"),
        );
    }

    Ok(())
}
