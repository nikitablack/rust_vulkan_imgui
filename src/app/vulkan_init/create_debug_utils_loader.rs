use crate::app;

pub fn create_debug_utils_loader(vulkan_data: &mut app::VulkanInitData) {
    vulkan_data.debug_utils_loader = Some(ash::extensions::ext::DebugUtils::new(
        vulkan_data.get_entry_ref(),
        vulkan_data.get_instance_ref(),
    ));
}
