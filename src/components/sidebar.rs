use std::default;

use eframe::egui::{self, panel::Side, Context, Response, Ui};
pub const SIDE_BAR:&str="side_bar";
pub enum State{
    Default,
    General,
}
pub struct SideBar{
    c:Context,
   title:String,
   pub state:State,
   // 存储按钮对应的界面
   page:Vec<(State,Response)>
}
impl SideBar {
    pub fn show(&mut self,ctx: &egui::Context) {
        let side_panel = egui::SidePanel::left(SIDE_BAR);
        
        // resizable 可以通过拖动面板边缘来调整面板大小吗？
        side_panel.resizable(false).show(ctx,|ui| {
            // ui.separator();
            //      ui.add(egui::Separator::default());
            // 加载器
            // ui.add(egui::widgets::Spinner::new())
            let default=ui.button("Default");
            let default=ui.button("Default");
               if  ui.button("Default").clicked(){
                   self.state=State::Default
               }
               if  ui.button("General").clicked(){
                self.state=State::General
               }
        });
    }
    pub fn new(title:String,ctx:Context)->Self {
        let mut page=Vec::with_capacity(10);
        // 添加按钮
        page.push((State::Default,ctx.));
        Self{
            c:ctx,
            title:title,
            state:State::Default,
            page:page
        }
    }
}