mod app;
mod utils;

#[macro_use]
extern crate static_assertions;

const NUM_RESOURCES_IN_FLIGHT: u32 = 2u32;
const FONT_SIZE_RATIO: f32 = 0.05;

fn init_internal(
    vulkan_data: &mut app::VulkanInitData,
    font_texture: &imgui::FontAtlasTexture,
) -> Result<app::AppData, String> {
    let mut app_data = app::AppData::default();

    app::create_render_pass(&mut app_data, vulkan_data)?;
    app::create_framebuffers(&mut app_data, vulkan_data)?;
    app::create_vertex_shader_module(&mut app_data, vulkan_data)?;
    app::create_fragment_shader_module(&mut app_data, vulkan_data)?;
    app::create_command_pool(&mut app_data, vulkan_data)?;
    app::create_vertex_buffers(&mut app_data, vulkan_data)?;
    app::create_index_buffers(&mut app_data, vulkan_data)?;
    app::create_descriptor_pool(&mut app_data, vulkan_data)?;
    app::create_sampler(&mut app_data, vulkan_data)?;
    app::create_descriptor_set_layout(&mut app_data, vulkan_data)?;
    app::create_pipeline_layout(&mut app_data, vulkan_data)?;
    app::create_fences(&mut app_data, vulkan_data)?;
    app::create_semaphores(&mut app_data, vulkan_data)?;
    app::create_pipeline(&mut app_data, vulkan_data)?;
    app::allocate_descriptor_sets(&mut app_data, vulkan_data)?;
    app::create_font_image(
        &mut app_data,
        vulkan_data,
        &[font_texture.width, font_texture.height],
    )?;
    app::create_font_image_view(&mut app_data, vulkan_data)?;
    app::copy_data_to_image(&mut app_data, vulkan_data, font_texture.data)?;
    app::update_descriptor_sets(&mut app_data, vulkan_data);

    Ok(app_data)
}

fn main() {
    let event_loop = winit::event_loop::EventLoop::new();
    let window = winit::window::WindowBuilder::new()
        .with_title("Vulkan GUI")
        .with_inner_size(winit::dpi::LogicalSize::new(800.0, 600.0))
        .build(&event_loop)
        .unwrap();

    let mut imgui_context = imgui::Context::create();
    imgui_context.set_ini_filename(None);
    let mut platform = imgui_winit_support::WinitPlatform::init(&mut imgui_context);
    platform.attach_window(
        imgui_context.io_mut(),
        &window,
        imgui_winit_support::HiDpiMode::Locked(1.0),
    );

    let font_size = ((window.inner_size().width as f32) * FONT_SIZE_RATIO).round();
    imgui_context
        .fonts()
        .add_font(&[imgui::FontSource::TtfData {
            data: include_bytes!("../resources/arial.ttf"),
            size_pixels: font_size,
            config: Some(imgui::FontConfig {
                name: Some(String::from("Arial")),
                ..imgui::FontConfig::default()
            }),
        }]);

    let device_extensions = vec![ash::extensions::khr::Swapchain::name()];

    let instance_extensions = app::get_required_instance_extensions(&window).unwrap();

    let result = app::init_vulkan(&window, &instance_extensions, &device_extensions);

    let mut vulkan_data = match result {
        Ok(vd) => vd,
        Err(msg) => panic!(msg),
    };

    let result = {
        let mut imgui_fonts = imgui_context.fonts();
        let font_texture = imgui_fonts.build_rgba32_texture();
        init_internal(&mut vulkan_data, &font_texture)
    };

    imgui_context.fonts().clear_tex_data();

    let mut app_data = match result {
        Ok(ad) => ad,
        Err(msg) => panic!(msg),
    };

    app_data.command_buffers =
        vec![ash::vk::CommandBuffer::null(); NUM_RESOURCES_IN_FLIGHT as usize];

    let mut app_exit = false;
    let mut last_frame = std::time::Instant::now();

    use winit::event::Event;
    use winit::event::WindowEvent;
    use winit::event_loop::ControlFlow;

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Poll;

        match event {
            Event::NewEvents(_) => {
                imgui_context
                    .io_mut()
                    .update_delta_time(std::time::Instant::now() - last_frame);
                last_frame = std::time::Instant::now();
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;

                app::clear(&mut app_data, &mut vulkan_data);

                app_exit = true;
            }

            Event::MainEventsCleared => {
                if app_exit {
                    return;
                }

                if window.inner_size().width == 0 && window.inner_size().height == 0 {
                    return;
                }

                if app_data.window_resized {
                    app_data.window_resized = false;

                    app::handle_window_resize(
                        &mut app_data,
                        &mut vulkan_data,
                        &window,
                        &mut imgui_context,
                    )
                    .unwrap();
                }

                platform
                    .prepare_frame(imgui_context.io_mut(), &window)
                    .expect("Failed to prepare frame");

                let imgui_ui = imgui_context.frame();
                platform.prepare_render(&imgui_ui, &window);
                let (draw_data, ui_result) = app::render_ui(&vulkan_data, imgui_ui);

                match ui_result {
                    app::UiResult::Quit => {
                        *control_flow = ControlFlow::Exit;
                        app::clear(&mut app_data, &mut vulkan_data);
                        app_exit = true;
                        return;
                    }
                    app::UiResult::None => (),
                }

                app::wait_resource_available(&app_data, &vulkan_data).unwrap();
                app::copy_ui_data(&mut app_data, &vulkan_data, draw_data).unwrap();
                app::draw(&mut app_data, &vulkan_data, draw_data).unwrap();

                app_data.resource_index += 1;
                app_data.resource_index %= NUM_RESOURCES_IN_FLIGHT;
            }

            Event::WindowEvent {
                event: WindowEvent::Resized { .. },
                ..
            } => {
                app_data.window_resized = true;
                platform.handle_event(imgui_context.io_mut(), &window, &event);
            }

            event => {
                platform.handle_event(imgui_context.io_mut(), &window, &event);
                // other application-specific event handling
            }
        }
    });
}
