use egui::{DragValue, global_theme_preference_buttons};
use egui_simpletabs::{dial::Dial, tabs::TabWidgetExt};

fn main() {
    let mut value = 10f64;

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Dial test", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            global_theme_preference_buttons(ui);

            ui.add(Dial::new(&mut value));
            ui.add(DragValue::new(&mut value));
        });
    })
    .unwrap();
}
