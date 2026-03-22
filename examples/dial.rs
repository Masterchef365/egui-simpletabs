use egui::{
    Color32, DragValue, Painter, Pos2, Shape, Stroke, Vec2, epaint::CubicBezierShape,
    global_theme_preference_buttons,
};
use egui_simpletabs::{
    dial::{Dial, DialPosition, DragMode},
    tabs::TabWidgetExt,
    utils::circular_arc_stroke,
};

fn main() {
    let mut value = 1f64;

    let mut drag_mode = DragMode::default();

    let mut has_min = true;
    let mut has_max = true;

    let mut min: f32 = -1.0;
    let mut max: f32 = 1.0;

    let mut invert = false;

    let mut underline = true;

    let mut origin_angle = -std::f64::consts::FRAC_PI_2;

    let mut mouse_sensitivity = 5e-2;

    let mut value_per_radian = 1.0;

    let mut has_snap = true;
    let mut snap_thresh = 0.05;

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Dial test", options, move |ctx, _frame| {
        let snap = has_snap.then(|| snap_thresh);

        egui::CentralPanel::default().show(ctx, |ui| {
            global_theme_preference_buttons(ui);

            ui.group(|ui| {
                ui.add(
                    Dial::new(&mut value)
                        .drag_mode(drag_mode)
                        .value_per_radian(value_per_radian)
                        .min_value(has_min.then(|| min))
                        .max_value(has_max.then(|| max))
                        .invert(invert)
                        .origin_angle(origin_angle)
                        .mouse_sensitivity(mouse_sensitivity)
                        .with_position(DialPosition::new(min).label("Min").snap(snap).underline(underline))
                        .with_position(DialPosition::new(0).label("Zero").snap(snap).underline(underline).color(Color32::DARK_GREEN))
                        .with_position(DialPosition::new(max).label("Max").snap(snap).underline(underline))
                );
                ui.add(DragValue::new(&mut value).speed(1e-2));
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
                ui.add_enabled(has_min, DragValue::new(&mut min).speed(1e-2));
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut has_max, "Has max");
                ui.add_enabled(has_max, DragValue::new(&mut max).speed(1e-2));
            });

            ui.checkbox(&mut invert, "Invert");

            ui.checkbox(&mut underline, "Underline");

            ui.horizontal(|ui| {
                ui.label("Origin angle: ");
                ui.add(DragValue::new(&mut origin_angle).speed(1e-2));
            });

            ui.horizontal(|ui| {
                ui.checkbox(&mut has_snap, "Snap");
                ui.add_enabled(has_snap, DragValue::new(&mut snap_thresh).speed(1e-2));
            });

            ui.horizontal(|ui| {
                ui.label("Mouse sensitivity");
                ui.add(DragValue::new(&mut mouse_sensitivity).speed(1e-2));
            });

            ui.horizontal(|ui| {
                ui.label("Value per radian");
                ui.add(DragValue::new(&mut value_per_radian).speed(1e-2));
            });

            /*
            let mut value_int = 10i32;
            let mut deadzone = 0.1;
            let mut has_deadzone = 0.1;
            ui.group(|ui| {
                ui.add(Dial::new(&mut value_int).range(min..=max, None))
            });
            */
        });
    })
    .unwrap();
}
