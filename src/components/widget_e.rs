//! 创建组件的例子

use eframe::egui;


/// iOS-style toggle switch:
///
/// ``` text
///      _____________
///     /       /.....\
///    |       |.......|
///     \_______\_____/
/// ```
///
/// ## Example:
/// ``` ignore
/// toggle_ui(ui, &mut my_bool);
/// ```
pub fn toggle_ui(ui: &mut egui::Ui, on: &mut bool) -> egui::Response {
   // Widget代码可以分成四个步骤:
//  1 决定小部件的大小
//  2 为它分配空间
//  3 处理与小部件的交互(如果有的话)
//  4 绘制小部件

 // 1 决定部件大小:
//你可以查询' ui '有多少可用空间，
//但是在这个例子中，我们有一个固定大小的小部件，基于一个标准按钮的高度:
    let desired_size = ui.spacing().interact_size.y * egui::vec2(2.0, 1.0);

    // 2. 分配空间:
    //这是我们分配屏幕区域的地方。
//我们还告诉Ui在分配的区域感知点击。
    let (rect, mut response) = ui.allocate_exact_size(desired_size, egui::Sense::click());

    // 3. 互动: 检查点击
    if response.clicked() {
        *on = !*on;
        response.mark_changed(); // 报告值变化
    }

    //在响应中附加一些元数据，供屏幕阅读器使用
    response.widget_info(|| {
        egui::WidgetInfo::selected(egui::WidgetType::Checkbox, ui.is_enabled(), *on, "")
    });

    // 4. 绘制
    if ui.is_rect_visible(rect) {
  //我们从egui请求一个简单的动画。
// egui跟踪与id和关联的布尔值的变化
//返回一个0-1范围内的动画值，表示我们打开了多少。
        let how_on = ui.ctx().animate_bool_responsive(response.id, *on);
//我们将按照当前的样式询问
//“与之交互的东西应该如何被绘制?”
//这将，例如，给我们不同的颜色，当小部件被悬停或点击。
        let visuals = ui.style().interact_selectable(&response, *on);
  //所有的坐标都是绝对屏幕坐标，所以我们使用' rect '来放置元素。
        let rect = rect.expand(visuals.expansion);
        let radius = 0.5 * rect.height();
        ui.painter()
            .rect(rect, radius, visuals.bg_fill, visuals.bg_stroke);
        // Paint the circle, animating it from left to right with `how_on`:
        let circle_x = egui::lerp((rect.left() + radius)..=(rect.right() - radius), how_on);
        let center = egui::pos2(circle_x, rect.center().y);
        ui.painter()
            .circle(center, 0.75 * radius, visuals.bg_fill, visuals.fg_stroke);
    }

    // All done! Return the interaction response so the user can check what happened
    // (hovered, clicked, ...) and maybe show a tooltip:
    response
}

// A wrapper that allows the more idiomatic usage pattern: `ui.add(toggle(&mut my_bool))`
/// iOS-style toggle switch.
///
/// ## Example:
/// ``` ignore
/// ui.add(toggle(&mut my_bool));
/// ```
pub fn toggle(on: &mut bool) -> impl egui::Widget + '_ {
    move |ui: &mut egui::Ui| toggle_ui(ui, on)
}

pub fn url_to_file_source_code() -> String {
    format!("https://github.com/emilk/egui/blob/master/{}", file!())
}