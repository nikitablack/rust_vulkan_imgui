use crate::app;
use ash::version::DeviceV1_0;

pub fn handle_window_resize(
    app_data: &mut app::AppData,
    vulkan_data: &mut app::VulkanInitData,
    window: &winit::window::Window,
    imgui_context: &mut imgui::Context,
) -> app::AppResult {
    unsafe {
        let _ = vulkan_data.get_device_ref().device_wait_idle();

        for &f in &app_data.framebuffers {
            vulkan_data.get_device_ref().destroy_framebuffer(f, None);
        }
    }
    app_data.framebuffers.clear();

    app::rebuild_swapchain_data(vulkan_data, window)?;
    app::create_framebuffers(app_data, vulkan_data)?;

    imgui_context.fonts().clear();
    imgui_context.fonts().clear_fonts();
    imgui_context.fonts().clear_tex_data();
    imgui_context.fonts().clear_input_data();

    let font_size = ((window.inner_size().width as f32) * crate::FONT_SIZE_RATIO).round();
    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("../../resources/arial.ttf"),
            size_pixels: font_size,
            config: Some(imgui::FontConfig {
                name: Some(String::from("Arial")),
                ..imgui::FontConfig::default()
            }),
        }]);

    let mut imgui_fonts = imgui_context.fonts();
    let font_texture = imgui_fonts.build_rgba32_texture();

    let _ = vulkan_data.get_allocator_ref().destroy_image(
        app_data.font_mem_image.image,
        &app_data.font_mem_image.allocation,
    );

    unsafe {
        vulkan_data
            .get_device_ref()
            .destroy_image_view(app_data.font_mem_image.view, None);
        app_data.font_mem_image = Default::default();
    }

    app::create_font_image(
        app_data,
        vulkan_data,
        &[font_texture.width, font_texture.height],
    )?;
    app::create_font_image_view(app_data, vulkan_data)?;
    app::copy_data_to_image(app_data, vulkan_data, font_texture.data)?;
    app::update_descriptor_sets(app_data, vulkan_data);

    Ok(())
}
