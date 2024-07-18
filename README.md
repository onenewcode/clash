# egui
## 概述
egui（发音类似“e-gooey”）是一个用Rust编程语言编写的即时模式GUI（Graphical User Interface，图形用户界面）库。它被设计得简单、快速且高度可移植，能够在多种平台上运行，包括Web（通过WebAssembly和WebGL）、桌面应用以及游戏引擎。

特点：
1. 即时模式GUI：
egui采用即时模式（immediate mode），这意味着界面的构建是在每一帧重新计算的，而不是基于窗口小部件的树状结构。这种模式简化了GUI的开发，因为开发者不需要担心复杂的布局管理和控件的状态管理。
2. 跨平台兼容性：
egui可以在Web上运行（通过编译为WebAssembly并使用WebGL渲染），也可以在桌面环境中运行，甚至可以集成到游戏引擎中，只要有能力绘制纹理三角形即可。
3. 高效渲染：
使用现代图形API（如Vulkan、Metal等）进行渲染，这使得egui能够高效地处理图形输出，即使在复杂的用户界面下也能保持良好的性能。
易于使用：
提供了简单直观的API，使开发者能够快速创建和修改用户界面元素，如按钮、滑块、文本框等。
4. 高度可定制化：
虽然egui提供了默认的样式和主题，但它也允许开发者自定义外观和行为，以适应特定的应用需求。

## hello world
下面将展示官方的第一个例子，并讲述他的运行原理，同时我们需要在运行前，在`Cargo.toml`文件中添加`eframe`依赖，依赖版本如下。
```toml
eframe = "0.28.1"
env_logger = { version = "0.10", default-features = false, features = [
    "auto-color",
    "humantime",
] }
```
```rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // 初始化日志库，用于调试
    // 设置窗口大小
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };

    // 设置我们app的初始化参数
    let mut name = "Arthur".to_owned();
    let mut age = 42;
// 运行我们的egui程序
    eframe::run_simple_native("My egui App", options, move |ctx, _frame| {
        // 创建一个窗口
        egui::CentralPanel::default().show(ctx, |ui| {
            // 创建一个标题
            ui.heading("My egui Application");
            // 创建一个水平布局
            ui.horizontal(|ui| {
                let name_label = ui.label("Your name: ");
                ui.text_edit_singleline(&mut name)
                    .labelled_by(name_label.id);
            });
            // 创建一个滑块
            ui.add(egui::Slider::new(&mut age, 0..=120).text("age"));
            // 创建一个按钮，并在它被点击时增加年龄
            if ui.button("Increment").clicked() {
                age += 1;
            }
            // 显示一个标签
            ui.label(format!("Hello '{name}', age {age}"));
        });
    })
}
```
## 多窗口
```rs
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(rustdoc::missing_crate_level_docs)] // it's an example

use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init();
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Multiple viewports",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
  ///即时视图是立即显示的，所以传递状态到/从他们是很容易的。
///缺点是，他们的绘画是链接到父视窗:
///如果其中一个需要重新粉刷，它们都被重新粉刷。
    show_immediate_viewport: bool,

    ///延迟视图独立于父视图运行，可以保存
/// CPU如果只有一些视图需要重新绘制。
///但是，这需要使用' Arc '和锁传递状态。
    show_deferred_viewport: Arc<AtomicBool>,
}

impl eframe::App for MyApp {
    // 视图更新逻辑
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // 创建一个窗口
        egui::CentralPanel::default().show(ctx, |ui| {
            // 显示一个标签
            ui.label("Hello from the root viewport");
            // 创建一个复选框
            ui.checkbox(
                &mut self.show_immediate_viewport,
                "Show immediate child viewport",
            );
             // 通过arc获取值
            let mut show_deferred_viewport = self.show_deferred_viewport.load(Ordering::Relaxed);
            ui.checkbox(&mut show_deferred_viewport, "Show deferred child viewport");
            self.show_deferred_viewport
                .store(show_deferred_viewport, Ordering::Relaxed);
        });
        // 判断是否产生新的窗口
        if self.show_immediate_viewport {
            //创建一个即时模式的视口。
            //这个视口会在当前帧立即显示，而不是等待下一个帧的重绘周期
            ctx.show_viewport_immediate(
                egui::ViewportId::from_hash_of("immediate_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Immediate Viewport")
                    .with_inner_size([200.0, 100.0]),
                |ctx, class| {
                    // 前视口确实是即时模式的。如果不是即时模式，断言将失败
                    assert!(
                        class == egui::ViewportClass::Immediate,
                        "This egui backend doesn't support multiple viewports"
                    );
                 
                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello from immediate viewport");
                    });
                  // 检查用户是否请求关闭当前视口。如果用户尝试关闭视口，则将 self.show_immediate_viewport 设置为 false
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent viewport that we should not show next frame:
                        self.show_immediate_viewport = false;
                    }
                },
            );
        }
        if self.show_deferred_viewport.load(Ordering::Relaxed) {
            let show_deferred_viewport = self.show_deferred_viewport.clone();
            ctx.show_viewport_deferred(
                egui::ViewportId::from_hash_of("deferred_viewport"),
                egui::ViewportBuilder::default()
                    .with_title("Deferred Viewport")
                    .with_inner_size([200.0, 100.0]),
                move |ctx, class| {
                    assert!(
                        class == egui::ViewportClass::Deferred,
                        "This egui backend doesn't support multiple viewports"
                    );

                    egui::CentralPanel::default().show(ctx, |ui| {
                        ui.label("Hello from deferred viewport");
                    });
                    if ctx.input(|i| i.viewport().close_requested()) {
                        // Tell parent to close us.
                        show_deferred_viewport.store(false, Ordering::Relaxed);
                    }
                },
            );
        }
    }
}
```

## 确认对话框
```rs
use eframe::egui;

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
    )
}

#[derive(Default)]
struct MyApp {
    show_confirmation_dialog: bool,
    allowed_to_close: bool,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Try to close the window");
        });
    // 检擦用户是否关闭当前视口
        if ctx.input(|i| i.viewport().close_requested()) {
            // 如果用户没有允许关闭窗口，则显示确认对话框
            if self.allowed_to_close {
            } else {
                // 取消关闭窗口
                ctx.send_viewport_cmd(egui::ViewportCommand::CancelClose);
                self.show_confirmation_dialog = true;
            }
        }

        if self.show_confirmation_dialog {
            egui::Window::new("Do you want to quit?")
                .collapsible(false)
                .resizable(false)
                .show(ctx, |ui| {
                    ui.horizontal(|ui| {
                        if ui.button("No").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = false;
                        }

                        if ui.button("Yes").clicked() {
                            self.show_confirmation_dialog = false;
                            self.allowed_to_close = true;
                            ui.ctx().send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                });
        }
    }
}
```

## 3d
```rs
use eframe::{egui, egui_glow, glow};

use egui::mutex::Mutex;
use std::sync::Arc;

fn main() -> eframe::Result {
    env_logger::init(); 
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([350.0, 380.0]),
        multisampling: 4,
        renderer: eframe::Renderer::Glow,
        ..Default::default()
    };
    eframe::run_native(
        "Custom 3D painting in eframe using glow",
        options,
        Box::new(|cc| Ok(Box::new(MyApp::new(cc)))),
    )
}

struct MyApp {
    ///在Arc<Mutex<…>>后面，这样我们可以将它传递给[' egui::PaintCallback ']，然后再进行绘制。
    rotating_triangle: Arc<Mutex<RotatingTriangle>>,
    angle: f32,
}

impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let gl = cc
            .gl
            .as_ref()
            .expect("You need to run eframe with the glow backend");
        Self {
            rotating_triangle: Arc::new(Mutex::new(RotatingTriangle::new(gl))),
            angle: 0.0,
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.spacing_mut().item_spacing.x = 0.0;
                ui.label("The triangle is being painted using ");
                // 添加超链接
                ui.hyperlink_to("glow", "https://github.com/grovesNL/glow");
                ui.label(" (OpenGL).");
            });
            // canvas方法将Frame转换为一个画布，允许在窗口内直接绘制
            egui::Frame::canvas(ui.style()).show(ui, |ui| {
                self.custom_painting(ui);
            });
            ui.label("Drag to rotate!");
        });
    }
    /// 退出时，销毁glow
    fn on_exit(&mut self, gl: Option<&glow::Context>) {
        if let Some(gl) = gl {
            self.rotating_triangle.lock().destroy(gl);
        }
    }
}

impl MyApp {
    /// 自定义绘画
    fn custom_painting(&mut self, ui: &mut egui::Ui) {
        // 绘制一个对拖拽敏感的的区域
        let (rect, response) =
            ui.allocate_exact_size(egui::Vec2::splat(300.0), egui::Sense::drag());
        //将鼠标水平方向的位移转换成角度变化，每次鼠标移动1单位，self.angle增加0.01弧度。
        self.angle += response.drag_motion().x * 0.01;

        //克隆局部变量，这样我们就可以把它们移动到paint回调中:
        let angle = self.angle;
        let rotating_triangle: Arc<Mutex<RotatingTriangle>> = self.rotating_triangle.clone();

        let callback = egui::PaintCallback {
            rect,
            callback: std::sync::Arc::new(egui_glow::CallbackFn::new(move |_info, painter| {
                rotating_triangle.lock().paint(painter.gl(), angle);
            })),
        };
        ui.painter().add(callback);
    }
}
// 三角区域
struct RotatingTriangle {
    // OpenGL着色器程序，用于处理顶点和片段数据
    program: glow::Program,
    // 顶点数组对象，用于存储顶点数据
    vertex_array: glow::VertexArray,
}

impl RotatingTriangle {
    fn new(gl: &glow::Context) -> Self {
        use glow::HasContext as _;
        // 根据目标架构选择正确的OpenGL着色语言版本（GLSL）。对于WebAssembly目标，使用ES版本
        let shader_version = if cfg!(target_arch = "wasm32") {
            "#version 300 es"
        } else {
            "#version 330"
        };

        unsafe {
            // 创建着色器程序
            let program = gl.create_program().expect("Cannot create program");
            // 定义顶点和片段着色器源代码，并编译它们
            let (vertex_shader_source, fragment_shader_source) = (
                r#"
                    const vec2 verts[3] = vec2[3](
                        vec2(0.0, 1.0),
                        vec2(-1.0, -1.0),
                        vec2(1.0, -1.0)
                    );
                    const vec4 colors[3] = vec4[3](
                        vec4(1.0, 0.0, 0.0, 1.0),
                        vec4(0.0, 1.0, 0.0, 1.0),
                        vec4(0.0, 0.0, 1.0, 1.0)
                    );
                    out vec4 v_color;
                    uniform float u_angle;
                    void main() {
                        v_color = colors[gl_VertexID];
                        gl_Position = vec4(verts[gl_VertexID], 0.0, 1.0);
                        gl_Position.x *= cos(u_angle);
                    }
                "#,
                r#"
                    precision mediump float;
                    in vec4 v_color;
                    out vec4 out_color;
                    void main() {
                        out_color = v_color;
                    }
                "#,
            );

            let shader_sources = [
                (glow::VERTEX_SHADER, vertex_shader_source),
                (glow::FRAGMENT_SHADER, fragment_shader_source),
            ];
            // 遍历着色器源代码，创建和编译着色器，并将它们附加到程序中
            let shaders: Vec<_> = shader_sources
                .iter()
                .map(|(shader_type, shader_source)| {
                    let shader = gl
                        .create_shader(*shader_type)
                        .expect("Cannot create shader");
                    gl.shader_source(shader, &format!("{shader_version}\n{shader_source}"));
                    gl.compile_shader(shader);
                    assert!(
                        gl.get_shader_compile_status(shader),
                        "Failed to compile {shader_type}: {}",
                        gl.get_shader_info_log(shader)
                    );
                    gl.attach_shader(program, shader);
                    shader
                })
                .collect();
            // 检查链接状态
            gl.link_program(program);
            assert!(
                gl.get_program_link_status(program),
                "{}",
                gl.get_program_info_log(program)
            );
            // 清理不需要的着色器
            for shader in shaders {
                gl.detach_shader(program, shader);
                gl.delete_shader(shader);
            }
            // 创建顶点数组对象
            let vertex_array = gl
                .create_vertex_array()
                .expect("Cannot create vertex array");

            Self {
                program,
                vertex_array,
            }
        }
    }

    fn destroy(&self, gl: &glow::Context) {
        use glow::HasContext as _;
        unsafe {
            gl.delete_program(self.program);
            gl.delete_vertex_array(self.vertex_array);
        }
    }

    fn paint(&self, gl: &glow::Context, angle: f32) {
        use glow::HasContext as _;
        unsafe {
            gl.use_program(Some(self.program));
            gl.uniform_1_f32(
                gl.get_uniform_location(self.program, "u_angle").as_ref(),
                angle,
            );
            gl.bind_vertex_array(Some(self.vertex_array));
            gl.draw_arrays(glow::TRIANGLES, 0, 3);
        }
    }
}
```

## 重置动画实列
```rs

use eframe::{egui, CreationContext, NativeOptions};
use egui::{Button, CentralPanel, Context, UserAttentionType};

use std::time::{Duration, SystemTime};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let native_options = NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([400., 200.]),
        ..Default::default()
    };
    eframe::run_native(
        "User attention test",
        native_options,
        Box::new(|cc| Ok(Box::new(Application::new(cc)))),
    )
}

fn repr(attention: UserAttentionType) -> String {
    format!("{attention:?}")
}

struct Application {
    attention: UserAttentionType,
    request_at: Option<SystemTime>,

    auto_reset: bool,
    reset_at: Option<SystemTime>,
}

impl Application {
    fn new(_cc: &CreationContext<'_>) -> Self {
        Self {
            attention: UserAttentionType::Informational,
            request_at: None,
            auto_reset: false,
            reset_at: None,
        }
    }

    fn attention_reset_timeout() -> Duration {
        Duration::from_secs(3)
    }

    fn attention_request_timeout() -> Duration {
        Duration::from_secs(2)
    }

    fn repaint_max_timeout() -> Duration {
        Duration::from_secs(1)
    }
}

impl eframe::App for Application {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        if let Some(request_at) = self.request_at {
            if request_at < SystemTime::now() {
                self.request_at = None;
                ctx.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(self.attention));
                if self.auto_reset {
                    self.auto_reset = false;
                    self.reset_at = Some(SystemTime::now() + Self::attention_reset_timeout());
                }
            }
        }

        if let Some(reset_at) = self.reset_at {
            if reset_at < SystemTime::now() {
                self.reset_at = None;
                ctx.send_viewport_cmd(egui::ViewportCommand::RequestUserAttention(
                    UserAttentionType::Reset,
                ));
            }
        }

        CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.horizontal(|ui| {
                    ui.label("Attention type:");
                    egui::ComboBox::new("attention", "")
                        .selected_text(repr(self.attention))
                        .show_ui(ui, |ui| {
                            for kind in [
                                UserAttentionType::Informational,
                                UserAttentionType::Critical,
                            ] {
                                ui.selectable_value(&mut self.attention, kind, repr(kind));
                            }
                        })
                });

                let button_enabled = self.request_at.is_none() && self.reset_at.is_none();
                let button_text = if button_enabled {
                    format!(
                        "Request in {} seconds",
                        Self::attention_request_timeout().as_secs()
                    )
                } else {
                    match self.reset_at {
                        None => "Unfocus the window, fast!".to_owned(),
                        Some(t) => {
                            if let Ok(elapsed) = t.duration_since(SystemTime::now()) {
                                format!("Resetting attention in {} s…", elapsed.as_secs())
                            } else {
                                "Resetting attention…".to_owned()
                            }
                        }
                    }
                };

                let resp = ui
                    .add_enabled(button_enabled, Button::new(button_text))
                    .on_hover_text_at_pointer(
                        "After clicking, unfocus the application's window to see the effect",
                    );

                ui.checkbox(
                    &mut self.auto_reset,
                    format!(
                        "Reset after {} seconds",
                        Self::attention_reset_timeout().as_secs()
                    ),
                );

                if resp.clicked() {
                    self.request_at = Some(SystemTime::now() + Self::attention_request_timeout());
                }
            });
        });

        ctx.request_repaint_after(Self::repaint_max_timeout());
    }
}
```