use egui::{Color32, Stroke};
use egui_simpletabs::groupbox::FrameGroupBoxExt;

fn main() {
    let mut paused = false;
    let mut scene_rect = egui::Rect::ZERO;
    eframe::run_simple_native("Button test", Default::default(), move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Scene::new().zoom_range(0.01..=100.0).show(ui, &mut scene_rect, |ui| {
                let resp = egui::Frame::group(ui.style()).groupbox("Test box").show(ui, |ui| {
                    ui.label("This statement is false or whatever");
                });
            });
        });
    })
    .unwrap();
}
