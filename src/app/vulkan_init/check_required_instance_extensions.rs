use crate::app;
use ash::version::EntryV1_0;
use ash::vk;

pub fn check_required_instance_extensions(
    vulkan_data: &app::VulkanInitData,
    instance_extensions: &Vec<&'static std::ffi::CStr>,
) -> app::VulkanInitResult {
    let supported_instance_extensions = match vulkan_data
        .get_entry_ref()
        .enumerate_instance_extension_properties()
    {
        Ok(props) => props,
        Err(_) => {
            return Err(String::from(
                "failed to enumerate instance extension properies",
            ))
        }
    };

    let mut supported_instance_extensions_set = std::collections::HashSet::new();
    for vk::ExtensionProperties { extension_name, .. } in &supported_instance_extensions {
        supported_instance_extensions_set
            .insert(unsafe { std::ffi::CStr::from_ptr(extension_name.as_ptr()) });
    }

    for &extension_name in instance_extensions {
        if !supported_instance_extensions_set.contains(extension_name) {
            return Err(format!(
                "instance extension {:?} is not supported",
                extension_name
            ));
        }
    }

    Ok(())
}
