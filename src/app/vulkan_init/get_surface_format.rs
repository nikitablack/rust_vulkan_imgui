use crate::app;
use ash::vk;

pub fn get_surface_format(vulkan_data: &mut app::VulkanInitData) -> app::VulkanInitResult {
    let formats = match unsafe {
        vulkan_data
            .get_surface_loader_ref()
            .get_physical_device_surface_formats(vulkan_data.physical_device, vulkan_data.surface)
    } {
        Ok(formats) => formats,
        Err(_) => {
            return Err(String::from(
                "failed to get physical device surface formats",
            ));
        }
    };

    if formats.is_empty() {
        return Err(String::from(
            "failed to get physical device surface formats",
        ));
    }

    if formats.len() == 1 && formats[0].format == vk::Format::UNDEFINED {
        vulkan_data.surface_format = vk::SurfaceFormatKHR {
            format: vk::Format::B8G8R8A8_UNORM,
            color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
        };

        return Ok(());
    }

    for f in &formats {
        if f.format == vk::Format::B8G8R8A8_UNORM
            && f.color_space == vk::ColorSpaceKHR::SRGB_NONLINEAR
        {
            vulkan_data.surface_format = vk::SurfaceFormatKHR {
                format: vk::Format::B8G8R8A8_UNORM,
                color_space: vk::ColorSpaceKHR::SRGB_NONLINEAR,
            };

            return Ok(());
        }
    }

    vulkan_data.surface_format = formats[0];

    Ok(())
}
