use egui::{Color32, DragValue, Painter, Pos2, Shape, Stroke, Vec2, epaint::CubicBezierShape, global_theme_preference_buttons};
use egui_simpletabs::{
    dial::{Dial, DragMode},
    tabs::TabWidgetExt, utils::circular_arc,
};

fn main() {
    let mut value = 10f64;

    let mut drag_mode = DragMode::default();

    let mut has_min = false;
    let mut has_max = false;

    let mut min: f32 = 0.0;
    let mut max: f32 = 1.0;

    let mut invert = false;

    let mut origin_angle = -std::f64::consts::FRAC_PI_2;

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Dial test", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            global_theme_preference_buttons(ui);

            ui.group(|ui| {
                ui.add(
                    Dial::new(&mut value)
                        .drag_mode(drag_mode)
                        .min_value(has_min.then(|| min))
                        .max_value(has_max.then(|| max))
                        .invert(invert)
                        .origin_angle(origin_angle),
                );
                ui.add(DragValue::new(&mut value));
            });

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

            ui.checkbox(&mut invert, "Invert");

            ui.horizontal(|ui| {
                ui.label("Origin angle: ");
                ui.add(DragValue::new(&mut origin_angle).speed(1e-2));
            });

            /*
            let mut value_int = 10i32;
            let mut deadzone = 0.1;
            let mut has_deadzone = 0.1;
            ui.group(|ui| {
                ui.add(Dial::new(&mut value_int).range(min..=max, None))
            });
            */

            let pos = ui.next_widget_position() + Vec2::splat(50.0);
            ui.painter().circle_stroke(pos, 20.0, Stroke::new(2.0, Color32::RED));
            circular_arc(ui.painter(), pos, 20.0, 0.0, std::f32::consts::TAU, 20, Stroke::new(2.0, Color32::WHITE));
        });
    })
    .unwrap();
}
