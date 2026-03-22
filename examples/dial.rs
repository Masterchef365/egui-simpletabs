use egui::{global_theme_preference_buttons, DragValue};
use egui_simpletabs::{
    dial::{Dial, DragMode},
    tabs::TabWidgetExt,
};

fn main() {
    let mut value = 10f64;
    let mut drag_mode = DragMode::default();

    let mut has_min = false;
    let mut has_max = false;

    let mut min: f32 = 0.0;
    let mut max: f32 = 1.0;

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Dial test", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            global_theme_preference_buttons(ui);

            ui.add(
                Dial::new(&mut value)
                    .drag_mode(drag_mode)
                    .min_value(has_min.then(|| min))
                    .max_value(has_max.then(|| max)),
            );
            ui.add(DragValue::new(&mut value));

            ui.horizontal(|ui| {
                ui.selectable_value(&mut drag_mode, DragMode::CoordinateY, "CoordinateY");
                ui.selectable_value(&mut drag_mode, DragMode::CoordinateX, "CoordinateX");
                ui.selectable_value(&mut drag_mode, DragMode::Radial, "Radial");
                ui.selectable_value(
                    &mut drag_mode,
                    DragMode::DistanceFromCenter,
                    "DistanceFromCenter",
                );
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut has_min, "Has min");
                ui.add(DragValue::new(&mut min));
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut has_max, "Has max");
                ui.add(DragValue::new(&mut max));
            });
        });
    })
    .unwrap();
}
