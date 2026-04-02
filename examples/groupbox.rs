use egui::{global_theme_preference_buttons, global_theme_preference_switch, Color32, Stroke};
use egui_simpletabs::groupbox::{FrameGroupBoxExt, GroupBox, UiGroupBoxExt};

fn main() {
    let mut label = String::from("Test box");
    let mut scene_rect = egui::Rect::ZERO;
    eframe::run_simple_native("GroupBox test", Default::default(), move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut label);
            global_theme_preference_buttons(ui);

            ui.add_space(30.0);

            ui.group_box("Hello egui", |ui| {
                ui.label("Welcome back to 2008");
            });

            egui::Frame::group(ui.style())
                .fill(Color32::BLUE)
                .corner_radius(30.0)
                .outer_margin(30.0)
                .inner_margin(30.0)
                .group_box(&label)
                .text_color(Color32::RED)
                .stroke(Stroke::new(0.5, Color32::RED))
                .show(ui, |ui| {
                    ui.label("This statement is false or whatever");
                });

            egui::Scene::new()
                .zoom_range(0.01..=100.0)
                .show(ui, &mut scene_rect, |ui| {
                    GroupBox::new(ui.style(), &label) 
                        .show(ui, |ui| {
                            ui.label("This statement is false or whatever");
                        });
                });
        });
    })
    .unwrap();
}
