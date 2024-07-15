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