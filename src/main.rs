//! This example shows that you can use egui in parallel from multiple threads.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release


use clash::components::sidebar::{SideBar, SIDE_BAR};
use eframe::egui;

fn main() -> Result<(), eframe::Error> {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([1024.0, 768.0]),
        ..Default::default()
    };
    eframe::run_native(
        "My parallel egui App",
        options,
        Box::new(|_cc| Box::new(MyApp::new())),
    )
}

struct MyApp {
   sidebar:SideBar
}

impl MyApp {
    fn new() -> Self {
      Self {
     sidebar:SideBar::new("test".to_owned()),
        }
    }
}

impl std::ops::Drop for MyApp {
    fn drop(&mut self) {

    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        
        egui::CentralPanel::default().show(ctx, |ui| {
            self.sidebar.show(ctx);
        });
        // 获取组件
        let sideBar=ctx.read_response(SIDE_BAR.into()).unwrap();
        // 获取组件矩形区域
        // sideBar.rect
 
         egui::Area::new(egui::Id::new("my_area"))
     .fixed_pos(egui::pos2(100.0, 100.0))
     .show(ctx, |ui| {
        match self.sidebar.state {
            clash::components::sidebar::State::Default => {
                ui.label("Default");
            },
            clash::components::sidebar::State::General => {
                ui.label("General");
            },
        }
         
        });
    }
}