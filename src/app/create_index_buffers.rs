use crate::app;
use ash::vk;

pub fn create_index_buffer(
    vulkan_data: &app::VulkanInitData,
    size: vk::DeviceSize,
    name: String,
) -> Result<app::MemBuffer, String> {
    let device = vulkan_data.get_device_ref();

    let mem_buffer = match app::create_buffer(
        vulkan_data.get_allocator_ref(),
        size,
        vk::BufferUsageFlags::INDEX_BUFFER,
        vk_mem::MemoryUsage::CpuToGpu,
        vk_mem::AllocationCreateFlags::MAPPED,
    ) {
        Ok(buf) => buf,
        Err(_) => return Err(format!("failed to allocate buffer {}", name.clone())),
    };

    if let Some(debug_utils) = vulkan_data.debug_utils_loader.as_ref() {
        app::set_debug_utils_object_name(
            debug_utils,
            device.handle(),
            mem_buffer.buffer,
            name.clone(),
        );

        app::set_debug_utils_object_name(
            debug_utils,
            device.handle(),
            mem_buffer.get_allocation_info_ref().get_device_memory(),
            name,
        );
    }

    Ok(mem_buffer)
}

pub fn create_index_buffers(
    app_data: &mut app::AppData,
    vulkan_data: &app::VulkanInitData,
) -> app::AppResult {
    debug_assert!(app_data.index_mem_buffers.is_empty());

    app_data
        .index_mem_buffers
        .reserve(crate::NUM_RESOURCES_IN_FLIGHT as usize);

    for i in 0..crate::NUM_RESOURCES_IN_FLIGHT {
        let mem_buffer = app::create_index_buffer(vulkan_data, 100, format!("index buffer {}", i))?;

        app_data.index_mem_buffers.push(mem_buffer);
    }

    Ok(())
}
