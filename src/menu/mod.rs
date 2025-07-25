use crate::common::*;
use crate::imgui_rs_fix::ImguiFixedFunctions;
use imgui::*;
pub mod font;
use std::cell::RefCell;
use std::rc::Rc;
// ======================
// 状态管理结构体
// ======================

#[derive(PartialEq, Clone, Copy)]
enum Headers {
    Player,
    Update,
    About,
    Hide,
}

#[derive(Default, Clone, Copy)]
struct LastCoordinate {
    pos_x: f32,
    pos_y: f32,
    size_x: f32,
    size_y: f32,
}

struct AppState {
    permeate_record: bool,
    permeate_record_ini: bool,
    show_menu: bool,
    tab: Headers,
    last_coordinate: LastCoordinate,
    app_close: bool,
    should_restore_window: bool,
}

impl AppState {
    fn new() -> Self {
        AppState {
            permeate_record: false,
            permeate_record_ini: false,
            show_menu: true,
            tab: Headers::Player,
            last_coordinate: LastCoordinate::default(),
            app_close: false,
            should_restore_window: false,
        }
    }
}

#[derive(Default)]
struct Vector3 {
    X: f32,
    Y: f32,
    Z: f32,
}

impl Vector3 {
    fn new(x: f32, y: f32, z: f32) -> Self {
        Vector3 { X: x, Y: y, Z: z }
    }
}

// ======================
// UI 渲染函数
// ======================
use android_native_window::Window as NativeWindow;

fn layout_tick_ui(ui: &Ui, state: &mut AppState) {
    // 处理窗口大小和位置重置逻辑

    // 绘制主界面
    if state.show_menu {
        // 设置窗口大小约束
        state.permeate_record_ini = false;
        let mut size = [800.0, 400.0];
        // 窗口背景色
        let windowbg = ui.push_style_color(
            StyleColor::WindowBg,
            [20.0 / 255.0, 23.0 / 255.0, 25.0 / 255.0, 1.0],
        );
        // 子窗口背景色
        let childbg = ui.push_style_color(
            StyleColor::ChildBg,
            [24.0 / 255.0, 28.0 / 255.0, 30.0 / 255.0, 1.0],
        );
        // 文本颜色
        let text = ui.push_style_color(StyleColor::Text, [1.0, 1.0, 1.0, 1.0]);

        // 头部颜色
        let header = ui.push_style_color(
            StyleColor::Header,
            [30.0 / 255.0, 138.0 / 255.0, 200.0 / 255.0, 1.0],
        );
        let headerhover = ui.push_style_color(
            StyleColor::HeaderHovered,
            [31.0 / 255.0, 110.0 / 255.0, 171.0 / 255.0, 1.0],
        );
        let headeractive = ui.push_style_color(
            StyleColor::HeaderActive,
            [30.0 / 255.0, 116.0 / 255.0, 215.0 / 255.0, 1.0],
        );

        // 按钮颜色
        let button = ui.push_style_color(
            StyleColor::Button,
            [25.0 / 255.0, 145.0 / 255.0, 215.0 / 255.0, 1.0],
        );
        let buttonhover = ui.push_style_color(
            StyleColor::ButtonHovered,
            [31.0 / 255.0, 110.0 / 255.0, 171.0 / 255.0, 1.0],
        );
        let buttonactive = ui.push_style_color(
            StyleColor::ButtonActive,
            [100.0 / 255.0, 161.0 / 255.0, 222.0 / 255.0, 1.0],
        );

        // 复选框颜色
        let cheackmark = ui.push_style_color(StyleColor::CheckMark, [0.0, 0.0, 0.0, 1.0]);
        let framebg = ui.push_style_color(
            StyleColor::FrameBg,
            [25.0 / 255.0, 158.0 / 255.0, 215.0 / 255.0, 200.0 / 255.0],
        );
        let framebgactive = ui.push_style_color(
            StyleColor::FrameBgActive,
            [25.0 / 255.0, 164.0 / 255.0, 215.0 / 255.0, 1.0],
        );
        let framebghover = ui.push_style_color(
            StyleColor::FrameBgHovered,
            [20.0 / 255.0, 212.0 / 255.0, 250.0 / 255.0, 1.0],
        );

        // 边框颜色
        let border = ui.push_style_color(StyleColor::Border, [0.0, 0.0, 0.0, 1.0]);
        // 设置圆角
        let wr = ui.push_style_var(StyleVar::WindowRounding(15.0));
        let fr = ui.push_style_var(StyleVar::FrameRounding(5.0));
        let sr = ui.push_style_var(StyleVar::ScrollbarRounding(5.0));
        let gr = ui.push_style_var(StyleVar::GrabRounding(2.3));
        let tr = ui.push_style_var(StyleVar::TabRounding(2.3));
        let cr = ui.push_style_var(StyleVar::ChildRounding(10.0));
        let wb = ui.push_style_var(StyleVar::WindowBorderSize((2.0)));

        let gm = ui.push_style_var(StyleVar::GrabMinSize(40.0));
        // 创建主窗口
        let mut window = Window::new(ui, "Mono").flags(WindowFlags::NO_TITLE_BAR);
        if state.should_restore_window {
            window = window
                .position(
                    [state.last_coordinate.pos_x, state.last_coordinate.pos_y],
                    Condition::Always,
                )
                .size(
                    [state.last_coordinate.size_x, state.last_coordinate.size_y],
                    Condition::Always,
                );
            state.should_restore_window = false;
        } else {
            window = window.size(size, Condition::FirstUseEver);
        }
        window.build(|| {
            // 保存当前窗口位置和大小
            state.last_coordinate.pos_x = ui.window_pos()[0];
            state.last_coordinate.pos_y = ui.window_pos()[1];
            state.last_coordinate.size_x = ui.window_size()[0];
            state.last_coordinate.size_y = ui.window_size()[1];

            // 左侧面板
            ui.child_window("MainLeft")
                .flags(WindowFlags::NO_SCROLLBAR)
                .size([
                    ui.content_region_avail()[0] * 0.16,
                    ui.content_region_avail()[1],
                ])
                .build(|| {
                    // 顶部标题区域
                    ui.child_window("LeftSide1")
                        .size([
                            ui.content_region_avail()[0],
                            ui.content_region_avail()[1] * 0.16,
                        ])
                        .build(|| {
                            ui.set_window_font_scale(1.4);
                            let project_name = "Mono";
                            let text_size = ui.calc_text_size(project_name);
                            let pos_x = (ui.content_region_avail()[0] - text_size[0]) * 0.5;
                            let pos_y = (ui.content_region_avail()[1] - text_size[1]) * 0.5;

                            ui.set_cursor_pos([pos_x, pos_y]);
                            ui.text_colored(
                                [84.0 / 255.0, 160.0 / 255.0, 227.0 / 255.0, 1.0],
                                project_name,
                            );
                        });

                    // 标签选择区域
                    ui.child_window("LeftSide2")
                        .size(ui.content_region_avail())
                        .build(|| {
                            let tab_names = ["Player", "Update", "About", "Hide"];
                            let item_height = ui.current_font_size() * 1.3;
                            for (i, name) in tab_names.iter().enumerate() {
                                let selected = match i {
                                    0 => state.tab == Headers::Player,
                                    1 => state.tab == Headers::Update,
                                    2 => state.tab == Headers::About,
                                    3 => state.tab == Headers::Hide,
                                    _ => false,
                                };
                                let clicked = ui
                                    .selectable_config(name)
                                    .selected(selected)
                                    .size([ui.content_region_avail()[0] * 0.85, item_height])
                                    .build();

                                if clicked {
                                    state.tab = match i {
                                        0 => Headers::Player,
                                        1 => Headers::Update,
                                        2 => Headers::About,
                                        3 => Headers::Hide,
                                        _ => Headers::Player,
                                    };
                                }
                            }
                        });
                });

            ui.same_line();

            // 右侧内容区域
            ui.child_window("RightSide")
                .size(ui.content_region_avail())
                .build(|| {
                    ui.spacing();
                    ui.spacing();
                    ui.indent();
                    match state.tab {
                        Headers::Player => {
                            if ui.checkbox("过录制", &mut state.permeate_record) {
                                state.permeate_record_ini = true;
                            }
                        }
                        Headers::Update => {
                            ui.text("更新内容将在这里显示");
                        }
                        Headers::About => {
                            if ui.button("关闭窗口") {
                                state.app_close = true;
                            }
                            ui.text(&format!("gui版本 : {}", imgui::dear_imgui_version()));

                            ui.text_colored(
                                [1.0, 0.0, 1.0, 1.0],
                                &format!(
                                    "应用平均 {:.3} ms/frame ({:.1} FPS)",
                                    1000.0 / ui.io().framerate,
                                    ui.io().framerate
                                ),
                            );

                            ui.text("by虚空遁地猪qwq,很虚空!");
                        }
                        Headers::Hide => {
                            state.tab = Headers::Player;
                            state.show_menu = false;
                        }
                    }

                    ui.unindent();
                    ui.spacing();
                    ui.spacing();
                });
        });
        // 恢复样式
        wr.pop();
        fr.pop();
        sr.pop();
        gr.pop();
        tr.pop();
        cr.pop();
        wb.pop();
        gm.pop();
        windowbg.pop();
        childbg.pop();
        text.pop();
        header.pop();
        headerhover.pop();
        headeractive.pop();
        button.pop();
        buttonhover.pop();
        buttonactive.pop();
        cheackmark.pop();
        framebg.pop();
        framebgactive.pop();
        framebghover.pop();
        border.pop();
    } else {
        let wr = ui.push_style_var(StyleVar::WindowRounding(15.0));
        let fr = ui.push_style_var(StyleVar::FrameRounding(5.0));
        let sr = ui.push_style_var(StyleVar::ScrollbarRounding(5.0));
        let gr = ui.push_style_var(StyleVar::GrabRounding(2.3));
        let tr = ui.push_style_var(StyleVar::TabRounding(2.3));
        let cr = ui.push_style_var(StyleVar::ChildRounding(5.0));
        let wb = ui.push_style_var(StyleVar::WindowBorderSize((2.0)));
        // 隐藏菜单时显示的小按钮
        let window = Window::new(ui, "Mono")
            .size([60.0, 60.0], Condition::Always)
            .flags(WindowFlags::NO_TITLE_BAR | WindowFlags::NO_RESIZE | WindowFlags::NO_SCROLLBAR);

        window.build(|| {
            if ui.button("----") {
                state.show_menu = true;
                state.should_restore_window = true;
            }
        });
        wr.pop();
        fr.pop();
        sr.pop();
        gr.pop();
        tr.pop();
        cr.pop();
        wb.pop();
    }
}

// ======================
// 主入口函数
// ======================

pub fn init_menu() -> Result<(), Box<dyn std::error::Error>> {
    let record = Rc::new(RefCell::new(false));
    let should_exit = Rc::new(RefCell::new(false));
    loop {
        let mut state = AppState::new();
        state.permeate_record = *record.borrow();
        // 创建系统实例
        let system = System::new("虚空遁地猪qwq", &*record.borrow())?;
        let record_clone = record.clone();
        let should_exit_clone = should_exit.clone();
        // 主循环
        system.run((), move |run, ui| {
            // 绘制主UI
            layout_tick_ui(ui, &mut state);
            // 检查是否需要关闭UI
            *record_clone.borrow_mut() = state.permeate_record;
            if state.app_close {
                *run = false;
                *should_exit_clone.borrow_mut() = true;
            }
            if state.permeate_record_ini {
                *run = false;
            }
        })?;
        if *should_exit.borrow() {
            break;
        }
    }

    Ok(())
}
