use eframe::egui;
use clash::components::sidebar;
fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([320.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Confirm exit",
        options,
        Box::new(|_cc| Ok(Box::new(MyApp::new_test()))),
    )
}

#[derive(Default)]
struct MyApp {
    side_bar: sidebar::SideBar,
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        self.side_bar.show(ctx);
    }
}
impl MyApp {
    //
    pub fn new_test()->Self{
        let side_bar = vec!["test1".to_string(),"test2".to_string()];
        
        MyApp{side_bar:sidebar::SideBar::new("test".to_string(),side_bar)}
    }
}