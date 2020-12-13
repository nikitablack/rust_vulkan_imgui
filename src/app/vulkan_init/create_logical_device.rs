use crate::app;
use ash::version::InstanceV1_0;
use ash::vk;

pub fn create_logical_device(
    vulkan_data: &mut app::VulkanInitData,
    device_extensions: &Vec<&'static std::ffi::CStr>,
) -> app::VulkanInitResult {
    let queue_indices = [vulkan_data.queue_family];

    let mut queue_priorities = Vec::new();
    for _ in &queue_indices {
        queue_priorities.push(vec![1.0f32]);
    }

    let mut queue_create_infos = Vec::with_capacity(queue_indices.len());

    for (ind, &family_index) in queue_indices.iter().enumerate() {
        let info = vk::DeviceQueueCreateInfo::builder()
            .queue_family_index(family_index)
            .queue_priorities(&queue_priorities[ind]);

        queue_create_infos.push(info.build());
    }

    let features = vk::PhysicalDeviceFeatures::builder();
    //.tessellation_shader(true)
    //.fill_mode_non_solid(true);

    let device_extensions_raw = device_extensions
        .iter()
        .map(|&s| s.as_ptr())
        .collect::<Vec<*const std::os::raw::c_char>>();

    let create_info = vk::DeviceCreateInfo::builder()
        .queue_create_infos(&queue_create_infos)
        .enabled_extension_names(&device_extensions_raw)
        .enabled_features(&features);

    vulkan_data.device = match unsafe {
        vulkan_data.get_instance_ref().create_device(
            vulkan_data.physical_device,
            &create_info,
            None,
        )
    } {
        Ok(device) => Some(device),
        Err(_) => return Err(String::from("failed to create device")),
    };

    Ok(())
}
