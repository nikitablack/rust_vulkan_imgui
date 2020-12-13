use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn create_pipeline_layout(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    let device = vulkan_data.get_device_ref();

    let pc_range = vk::PushConstantRange {
        stage_flags: vk::ShaderStageFlags::VERTEX,
        offset: 0,
        size: 4 * 4,
    };

    let laytouts = [app_data.descriptor_set_layout];
    let ranges = [pc_range];
    let create_info = vk::PipelineLayoutCreateInfo::builder()
        .set_layouts(&laytouts)
        .push_constant_ranges(&ranges)
        .build();

    app_data.pipeline_layout = match unsafe { device.create_pipeline_layout(&create_info, None) } {
        Ok(pl) => pl,
        Err(_) => return Err(String::from("failed to create pipeline layout")),
    };

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        app::set_debug_utils_object_name(
            debug_utils,
            device.handle(),
            app_data.pipeline_layout,
            String::from("pipeline layout"),
        );
    }

    Ok(())
}
