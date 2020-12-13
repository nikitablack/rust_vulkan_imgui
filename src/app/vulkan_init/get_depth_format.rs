use crate::app;
use ash::version::InstanceV1_0;
use ash::vk;

pub fn get_depth_format(vulkan_data: &mut app::VulkanInitData) -> app::VulkanInitResult {
    let format_candidates = [
        vk::Format::D24_UNORM_S8_UINT,
        vk::Format::D32_SFLOAT_S8_UINT,
        vk::Format::D16_UNORM_S8_UINT,
    ];

    for &format in &format_candidates {
        let props = unsafe {
            vulkan_data
                .get_instance_ref()
                .get_physical_device_format_properties(vulkan_data.physical_device, format)
        };

        if props
            .optimal_tiling_features
            .contains(vk::FormatFeatureFlags::DEPTH_STENCIL_ATTACHMENT)
        {
            vulkan_data.depth_format = format;
            return Ok(());
        }
    }

    Err(String::from("failed to find depth format"))
}
