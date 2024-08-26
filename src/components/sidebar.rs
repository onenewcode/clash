use eframe::egui::{self, text::LayoutJob, Button, Color32, FontFamily, FontId, Label, TextFormat};
use std::{
    sync::{atomic::AtomicUsize, mpsc::Receiver},
    usize,
};
pub const SIDE_BAR: &str = "side_bar";
// 侧边栏按钮的状态
#[derive(Clone)]
enum State {
    Checked,
    UnChecked,
}
#[derive(Default)]
pub struct SideBar {
    // 侧边栏,标头名称
    title: String,
    // 按钮名称列表
    button_names: Vec<String>,
    // 按钮状态
    states: Vec<State>,
}
impl SideBar {
    // 渲染侧边栏
    pub fn show(&mut self, ctx: &egui::Context) {
        let size = ctx.available_rect();
        // 生成侧边Panel,同时禁止侧边栏拖动，
        let side_panel = egui::SidePanel::left(SIDE_BAR)
            .resizable(false)
            .default_width(size.width() / 4.);
        // todo 指定宽度
        // let rec=side_panel.
        side_panel.show(ctx, |ui| {
            // 渲染标题
            // 逐个渲染按钮
                ui.horizontal_centered(|ui| {
                    (0..self.button_names.len())
                        .into_iter()
                        .for_each(|i| match self.states[i] {
                            State::Checked => {
                                ui.button(self.button_names[i].clone()).highlight();
                            }
                            State::UnChecked => {
                                if ui.button(self.button_names[i].clone()).clicked() {
                                    self.refresh_button(i);
                                }
                            }
                        })
            });
        });
    }
    // 用于更新按钮被被点击后的状态,index为按钮下标
    fn refresh_button(&mut self, index: usize) {
        self.states.fill(State::UnChecked);
        self.states[index] = State::Checked;
    }

    pub fn new(title: String, names: Vec<String>) -> SideBar {
        let states = vec![State::UnChecked; names.len()];
        Self {
            title: title,
            button_names: names,
            states: states,
        }
    }
}
