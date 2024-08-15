use std::sync::{atomic::AtomicUsize, mpsc::Receiver};
use eframe::egui::{
    self,  text::LayoutJob,  Color32, FontFamily, FontId, Label,
 TextFormat, 
};
pub const SIDE_BAR: &str = "side_bar";
// 侧边栏按钮的状态
enum State {
    Default,
    General,
}
#[derive(Default)]
pub struct SideBar {
    // 侧边栏,标头名称
    title: String,
    // 按钮列表
    bottons: Vec<egui::Button>,
    // 按钮名称列表
    botton_names: Vec<String>,
    // 按钮状态
    states:Vec<State>,
}
impl SideBar {
    pub fn show(&mut self, ctx: &egui::Context) {
        let side_panel = egui::SidePanel::left(SIDE_BAR);
        side_panel.resizable(false).show(ctx, |ui| {
            ui.add(self.upload_lable());
            let mut page = Vec::with_capacity(5);
            page.push((State::Default, egui::Button::new("Default")));
            page.push((State::General, egui::Button::new("General")));
            for i in page {
                let (k, v) = i;
                if ui.add(v).clicked() {
                    self.state = k;
                }
            }
            ui.menu_button("My sub-menu", |ui| {
                if ui.button("Close the menu").clicked() {
                    ui.close_menu();
                }
            });
        });
    }
    fn upload_lable(&self) -> Label {
        let mut job = LayoutJob::default();
        let mut speed = format!("0k \n,0k");
        if self.rbool == true {
            match self.receiver.try_recv() {
                Ok((up,down)) => speed = format!("{}k \n,{}k", up, down),
                Err(r) =>{
                    format!("{}",r);
                },
            };
        }
        job.append(
            &speed,
            0.0,
            TextFormat {
                font_id: FontId::new(14.0, FontFamily::Proportional),
                color: Color32::WHITE,
                ..Default::default()
            },
        );
        egui::Label::new(job)
    }
    pub fn open_receiver(&mut self) {
        self.rbool = true;
    }
    pub fn close_receiver(&mut self) {
        self.rbool = false;
    }
    pub fn new(title: String, receiver: Receiver<(f32, f32)>) -> Self {
        let mut page = Vec::with_capacity(5);
        page.push((State::Default, egui::Button::new("Default")));
        Self {
            // c:ctx,
            title: title,
            ..Default::default()
        }
    }
}
