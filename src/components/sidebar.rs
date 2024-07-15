

use std::sync::{atomic::AtomicUsize, mpsc::Receiver};

use eframe::egui::{
    self,  text::LayoutJob,  Color32, FontFamily, FontId, Label,
 TextFormat, 
};
pub const SIDE_BAR: &str = "side_bar";
pub enum State {
    Default,
    General,
}
// 全局变量
static GLOBAL_ID_COUNTER: AtomicUsize = AtomicUsize::new(0);
pub struct SideBar {
    title: String,
    pub state: State,
    // 判断是否开启网络请求
    rbool: bool,
    // 用于接受，网络上传，下载的速度
    // 第一个上传，第二个下载
    receiver: Receiver<(f32, f32)>,
}
impl SideBar {
    pub fn show(&mut self, ctx: &egui::Context) {
        let side_panel = egui::SidePanel::left(SIDE_BAR);
        side_panel.resizable(false).show(ctx, |ui| {
            // 在下一个小部件之前添加额外的空间。
            // ui.add_space  colored_label
            // 添加可选按钮
            //        ui.add(egui::Checkbox::new(&mut my_bool, "Checked"));

            // ui.colored_label(Color32::BLUE, "test".to_owned());
            //  添加按钮
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
    // 构建上传信息的标签
    fn upload_lable(&self) -> Label {
        // 用于控制布局
        let mut job = LayoutJob::default();
      
        let mut speed = format!("0k \n,0k");
        if self.rbool == true {
            //   接受网络速率的数据
            match self.receiver.try_recv() {
                Ok((up,down)) => speed = format!("{}k \n,{}k", up, down),
                // 寄存上一个frame todo
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
            state: State::Default,
            rbool: false,
            receiver: receiver,
        }
    }
}
