use ash::vk;
pub struct MemImage {
    pub image: vk::Image,
    pub view: vk::ImageView,
    pub allocation: vk_mem::Allocation,
    pub allocation_info: Option<vk_mem::AllocationInfo>,
    pub extent: vk::Extent3D,
}

impl Default for MemImage {
    fn default() -> Self {
        Self {
            image: vk::Image::null(),
            view: vk::ImageView::null(),
            allocation: vk_mem::Allocation::null(),
            allocation_info: None,
            extent: vk::Extent3D::default(),
        }
    }
}

impl MemImage {
    pub fn get_allocation_info_ref(&self) -> &vk_mem::AllocationInfo {
        self.allocation_info
            .as_ref()
            .expect("allocation info shouldn't be empty")
    }
}

pub struct MemBuffer {
    pub buffer: vk::Buffer,
    pub size: vk::DeviceSize,
    pub allocation: vk_mem::Allocation,
    pub allocation_info: Option<vk_mem::AllocationInfo>,
}

impl Default for MemBuffer {
    fn default() -> Self {
        Self {
            buffer: vk::Buffer::null(),
            size: 0,
            allocation: vk_mem::Allocation::null(),
            allocation_info: None,
        }
    }
}

impl MemBuffer {
    pub fn get_allocation_info_ref(&self) -> &vk_mem::AllocationInfo {
        self.allocation_info
            .as_ref()
            .expect("allocation info shouldn't be empty")
    }
}

#[derive(Default)]
pub struct AppData {
    pub render_pass: vk::RenderPass,
    pub framebuffers: Vec<vk::Framebuffer>,
    pub vertex_shader_module: vk::ShaderModule,
    pub fragment_shader_module: vk::ShaderModule,
    pub command_pool: vk::CommandPool,
    pub vertex_mem_buffers: Vec<MemBuffer>,
    pub index_mem_buffers: Vec<MemBuffer>,
    pub descriptor_pool: vk::DescriptorPool,
    pub descriptor_set_layout: vk::DescriptorSetLayout,
    pub pipeline_layout: vk::PipelineLayout,
    pub fences: Vec<vk::Fence>,
    pub image_available_semaphore: vk::Semaphore,
    pub graphics_finished_semaphore: vk::Semaphore,
    pub pipeline: vk::Pipeline,
    pub descriptor_sets: Vec<vk::DescriptorSet>,
    pub resource_index: u32,
    pub command_buffers: Vec<vk::CommandBuffer>,
    pub font_mem_image: MemImage,
    pub sampler: vk::Sampler,

    pub window_resized: bool,
}

pub type AppResult = Result<(), String>;

pub enum UiResult {
    None,
    Quit,
}
