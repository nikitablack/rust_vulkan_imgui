use crate::app;
use ash::version::DeviceV1_0;
use ash::vk;

pub fn clear(app_data: &mut app::AppData, vulkan_data: &mut app::VulkanInitData) {
    let device_ref = vulkan_data.get_device_ref();
    let allocator_ref = vulkan_data.get_allocator_ref();

    unsafe {
        let _ = device_ref.device_wait_idle();

        device_ref.destroy_render_pass(app_data.render_pass, None);
        app_data.render_pass = vk::RenderPass::null();

        for &f in &app_data.framebuffers {
            device_ref.destroy_framebuffer(f, None);
        }
        app_data.framebuffers.clear();

        device_ref.destroy_shader_module(app_data.vertex_shader_module, None);
        app_data.vertex_shader_module = vk::ShaderModule::null();

        device_ref.destroy_shader_module(app_data.fragment_shader_module, None);
        app_data.fragment_shader_module = vk::ShaderModule::null();

        device_ref.destroy_command_pool(app_data.command_pool, None);
        app_data.command_pool = vk::CommandPool::null();

        for mem_buf in &app_data.vertex_mem_buffers {
            let _ = allocator_ref.destroy_buffer(mem_buf.buffer, &mem_buf.allocation);
        }
        app_data.vertex_mem_buffers.clear();

        for mem_buf in &app_data.index_mem_buffers {
            let _ = allocator_ref.destroy_buffer(mem_buf.buffer, &mem_buf.allocation);
        }
        app_data.index_mem_buffers.clear();

        device_ref.destroy_descriptor_pool(app_data.descriptor_pool, None);
        app_data.descriptor_pool = vk::DescriptorPool::null();

        device_ref.destroy_descriptor_set_layout(app_data.descriptor_set_layout, None);
        app_data.descriptor_set_layout = vk::DescriptorSetLayout::null();

        device_ref.destroy_pipeline_layout(app_data.pipeline_layout, None);
        app_data.pipeline_layout = vk::PipelineLayout::null();

        for &f in &app_data.fences {
            device_ref.destroy_fence(f, None);
        }
        app_data.fences.clear();

        device_ref.destroy_semaphore(app_data.image_available_semaphore, None);
        app_data.image_available_semaphore = vk::Semaphore::null();

        device_ref.destroy_semaphore(app_data.graphics_finished_semaphore, None);
        app_data.graphics_finished_semaphore = vk::Semaphore::null();

        device_ref.destroy_pipeline(app_data.pipeline, None);
        app_data.pipeline = vk::Pipeline::null();

        let _ = allocator_ref.destroy_image(
            app_data.font_mem_image.image,
            &app_data.font_mem_image.allocation,
        );

        device_ref.destroy_image_view(app_data.font_mem_image.view, None);
        app_data.font_mem_image = Default::default();

        device_ref.destroy_sampler(app_data.sampler, None);
        app_data.sampler = vk::Sampler::null();
    }

    app::clear_vulkan_data(vulkan_data);
}
