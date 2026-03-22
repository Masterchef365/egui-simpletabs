use egui::{DragValue, global_theme_preference_buttons};
use egui_simpletabs::{dial::{Dial, DragMode}, tabs::TabWidgetExt};

fn main() {
    let mut value = 10f64;
    let mut drag_mode = DragMode::default();

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Dial test", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            global_theme_preference_buttons(ui);

            ui.add(Dial::new(&mut value).drag_mode(drag_mode));
            ui.add(DragValue::new(&mut value));

            ui.horizontal(|ui| {
                ui.selectable_value(&mut drag_mode, DragMode::CoordinateY, "CoordinateY");
                ui.selectable_value(&mut drag_mode, DragMode::CoordinateX, "CoordinateX");
                ui.selectable_value(&mut drag_mode, DragMode::Radial, "Radial");
                ui.selectable_value(&mut drag_mode, DragMode::DistanceFromCenter, "DistanceFromCenter");
            });
        });
    })
    .unwrap();
}
