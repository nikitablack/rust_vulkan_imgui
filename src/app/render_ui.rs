use crate::app;

static mut BORDER_SIZE: f32 = 0.0;

pub fn render_ui<'img_ctx>(
    vulkan_data: &app::VulkanInitData,
    imgui_ui: imgui::Ui<'img_ctx>,
) -> (&'img_ctx imgui::DrawData, app::UiResult) {
    let mut ui_result = app::UiResult::None;

    let window_width = vulkan_data.surface_extent.width as f32;
    let window_height = vulkan_data.surface_extent.height as f32;
    let button_width = window_width * 0.2;
    let button_height = button_width * 0.5;
    let window_padding_h = (window_width - button_width) * 0.5;
    let window_padding_v = (window_height - button_height) * 0.5;

    let styles = imgui_ui.push_style_vars(&[
        imgui::StyleVar::WindowBorderSize(0.0),
        imgui::StyleVar::WindowRounding(0.0),
        imgui::StyleVar::WindowPadding([0.0, 0.0]),
    ]);

    imgui::Window::new(imgui::im_str!("Test"))
        .size([window_width, window_height], imgui::Condition::Always)
        .position([0.0, 0.0], imgui::Condition::Always)
        .flags(imgui::WindowFlags::NO_DECORATION | imgui::WindowFlags::NO_BACKGROUND)
        .build(&imgui_ui, || {
            let draw_list = imgui_ui.get_window_draw_list();
            let _ = draw_list
                .add_rect(
                    unsafe {
                        [
                            window_padding_h - BORDER_SIZE,
                            window_padding_v - BORDER_SIZE,
                        ]
                    },
                    unsafe {
                        [
                            window_padding_h + button_width + BORDER_SIZE,
                            window_padding_v + button_height + BORDER_SIZE,
                        ]
                    },
                    [1.0, 1.0, 1.0, 1.0],
                )
                .filled(true)
                .build();

            let colors = imgui_ui.push_style_colors(&[
                (imgui::StyleColor::Button, [1.0, 0.0, 0.0, 1.0]),
                (imgui::StyleColor::ButtonHovered, [0.0, 1.0, 0.0, 1.0]),
                (imgui::StyleColor::ButtonActive, [0.0, 0.0, 1.0, 1.0]),
                (imgui::StyleColor::Border, [0.0, 0.0, 0.0, 1.0]),
            ]);

            imgui_ui.set_cursor_pos([window_padding_h, window_padding_v]);
            if imgui_ui.button(imgui::im_str!("Quit"), [button_width, button_height]) == true {
                ui_result = app::UiResult::Quit;
            }

            unsafe {
                if imgui_ui.is_item_hovered() {
                    BORDER_SIZE = (BORDER_SIZE + 0.1).min(10.0);
                } else {
                    BORDER_SIZE = (BORDER_SIZE - 0.1).max(0.0);
                }
            }

            colors.pop(&imgui_ui);
        });

    styles.pop(&imgui_ui);

    //imgui_ui.show_demo_window(&mut true);

    (imgui_ui.render(), ui_result)
}
