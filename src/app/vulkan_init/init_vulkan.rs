use crate::app;

fn init_internal(
    vulkan_data: &mut app::VulkanInitData,
    window: &winit::window::Window,
    instance_extensions: &Vec<&'static std::ffi::CStr>,
    device_extensions: &Vec<&'static std::ffi::CStr>,
) -> Result<(), String> {
    vulkan_data.entry = match ash::Entry::new() {
        Ok(entry) => Some(entry),
        Err(_) => return Err(String::from("failed to create Entry")),
    };

    app::check_instance_version(vulkan_data)?;
    app::check_required_instance_extensions(vulkan_data, instance_extensions)?;
    app::create_instance(vulkan_data, instance_extensions)?;
    app::create_debug_utils_loader(vulkan_data);
    app::create_surface_loader(vulkan_data);
    app::create_surface(vulkan_data, window)?;
    app::get_physical_device(vulkan_data, device_extensions)?;
    app::get_surface_format(vulkan_data)?;
    app::get_present_mode(vulkan_data)?;
    //vulkan_data.present_mode = ash::vk::PresentModeKHR::FIFO;
    app::get_queue_family(vulkan_data)?;
    app::get_depth_format(vulkan_data)?;
    app::get_physical_device_properties(vulkan_data);
    app::create_logical_device(vulkan_data, device_extensions)?;
    app::get_device_queue(vulkan_data);
    app::create_swapchain_loader(vulkan_data);
    app::get_surface_capabilities(vulkan_data)?;
    app::get_surface_extent(vulkan_data, window);
    app::create_swapchain(vulkan_data)?;
    app::get_swapchain_images(vulkan_data)?;
    app::get_swapchain_image_views(vulkan_data)?;
    app::create_allocator(vulkan_data)?;

    Ok(())
}

pub fn init_vulkan(
    window: &winit::window::Window,
    instance_extensions: &Vec<&'static std::ffi::CStr>,
    device_extensions: &Vec<&'static std::ffi::CStr>,
) -> Result<app::VulkanInitData, String> {
    let mut vulkan_data = app::VulkanInitData::default();

    if let Err(msg) = init_internal(
        &mut vulkan_data,
        window,
        instance_extensions,
        device_extensions,
    ) {
        app::clear_vulkan_data(&mut vulkan_data);
        return Err(msg);
    }

    Ok(vulkan_data)
}
