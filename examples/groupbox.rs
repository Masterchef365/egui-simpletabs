use egui::{Color32, Stroke, global_theme_preference_buttons, global_theme_preference_switch};
use egui_simpletabs::groupbox::FrameGroupBoxExt;

fn main() {
    let mut paused = false;
    let mut label = String::from("Test box");
    let mut scene_rect = egui::Rect::ZERO;
    eframe::run_simple_native("Button test", Default::default(), move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.text_edit_singleline(&mut label);
            global_theme_preference_buttons(ui);

            let resp = egui::Frame::group(ui.style()).groupbox(&label).text_color(Color32::RED).stroke(Stroke::new(0.5, Color32::RED)).show(ui, |ui| {
                ui.label("This statement is false or whatever");
            });

            egui::Scene::new().zoom_range(0.01..=100.0).show(ui, &mut scene_rect, |ui| {
                let resp = egui::Frame::group(ui.style()).groupbox(&label).show(ui, |ui| {
                    ui.label("This statement is false or whatever");
                });
            });

        });
    })
    .unwrap();
}
