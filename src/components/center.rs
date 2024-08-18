use eframe::egui;
#[derive(Default)]
pub struct CPanel {}

impl CPanel {
    fn name(&self) -> &'static str {
        "🗖 Panels"
    }

    pub fn show(&mut self, ctx: &egui::Context) {
        // 获取侧边栏,大小
        // let sidebar_rect=ctx.read_response(SIDE_BAR).unwrap().rect;
        egui::CentralPanel::default().show(ctx, |ui| {
            self.ui(ui);
        });

    }
}

// impl View for Panels {
impl CPanel {
    fn ui(&mut self, ui: &mut egui::Ui) {
        egui::CentralPanel::default().show_inside(ui, |ui| {
            ui.vertical_centered(|ui| {
                ui.heading("Central Panel");
            });
            egui::ScrollArea::vertical().show(ui, |ui| {
                lorem_ipsum(ui);
            });
        });
    }
}

fn lorem_ipsum(ui: &mut egui::Ui) {
    ui.with_layout(
        egui::Layout::top_down(egui::Align::LEFT).with_cross_justify(true),
        |ui| {
            ui.label(egui::RichText::new("cdsacs").small().weak());
            ui.add(egui::Separator::default().grow(8.0));
            ui.label(egui::RichText::new("dxc").small().weak());
        },
    );
}
