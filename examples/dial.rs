use egui::{global_theme_preference_buttons, Color32, DragValue};
use egui_simpletabs::{
    dial::{Dial, DialPosition, DragMode},
    utils::IndecisiveOption,
};

fn main() {
    let mut value = 1f64;

    let mut drag_mode = DragMode::default();

    let mut min: IndecisiveOption<f32> = Some(-1.0).into();
    let mut max: IndecisiveOption<f32> = Some(2.0).into();

    let mut invert = false;

    let mut underline = true;

    let mut origin_angle = -std::f64::consts::FRAC_PI_2;

    let mut mouse_sensitivity = 5e-2;

    let mut value_per_radian = 1.0;

    let mut show_livezone = true;

    let mut snap: IndecisiveOption<f32> = Some(0.05).into();

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Dial test", options, move |ctx, _frame| {
        egui::SidePanel::left("cfg").show(ctx, |ui| {
            global_theme_preference_buttons(ui);

            ui.horizontal(|ui| {
                ui.label("Min value");
                min.show(ui, |ui, min| ui.add(DragValue::new(min).speed(1e-2)));
            });

            ui.horizontal(|ui| {
                ui.label("Max value");
                max.show(ui, |ui, max| ui.add(DragValue::new(max).speed(1e-2)));
            });

            ui.checkbox(&mut invert, "Invert");

            ui.checkbox(&mut underline, "Underline");
            ui.checkbox(&mut show_livezone, "Show live zone");

            ui.horizontal(|ui| {
                ui.label("Origin angle: ");
                ui.add(DragValue::new(&mut origin_angle).speed(1e-2));
            });

            ui.horizontal(|ui| {
                ui.label("Snap: ");
                snap.show(ui, |ui, snap_thresh| {
                    ui.add(DragValue::new(snap_thresh).speed(1e-2))
                });
                //ui.checkbox(&mut has_snap, "Snap");
                //ui.add_enabled(has_snap, DragValue::new(&mut snap_thresh).speed(1e-2));
            });

            ui.horizontal(|ui| {
                ui.label("Mouse sensitivity");
                ui.add(DragValue::new(&mut mouse_sensitivity).speed(1e-2));
            });

            ui.horizontal(|ui| {
                ui.label("Value per radian");
                ui.add(DragValue::new(&mut value_per_radian).speed(1e-2));
            });

            ui.group(|ui| {
                ui.strong("Drag mode");
                ui.selectable_value(&mut drag_mode, DragMode::CoordinateY, "Coordinate Y");
                ui.selectable_value(&mut drag_mode, DragMode::CoordinateX, "Coordinate X");
                ui.selectable_value(&mut drag_mode, DragMode::Radial, "Radial");
                ui.selectable_value(
                    &mut drag_mode,
                    DragMode::DistanceFromCenter,
                    "Distance From Center",
                );
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Dial test");

            let mut dial = Dial::new(&mut value)
                .drag_mode(drag_mode)
                .value_per_radian(value_per_radian)
                .min_value(min.into_option())
                .max_value(max.into_option())
                .invert(invert)
                .origin_angle(origin_angle)
                .mouse_sensitivity(mouse_sensitivity)
                .show_livezone(show_livezone)
                .with_position(
                    DialPosition::new(0)
                        .label("Zero")
                        .snap(snap.into())
                        .underline(underline)
                        .color(Color32::DARK_GREEN),
                )
                .with_position(
                    DialPosition::new(1)
                        .label("One")
                        .snap(snap.into())
                        .underline(underline),
                );

            if let Some(min) = min.into_option() {
                dial = dial.with_position(
                    DialPosition::new(min)
                        .label("Min")
                        .snap(snap.into())
                        .underline(underline),
                );
            }

            if let Some(max) = max.into_option() {
                dial = dial.with_position(
                    DialPosition::new(max)
                        .label("Max")
                        .snap(snap.into())
                        .underline(underline),
                );
            }

            ui.add(dial);
            ui.add(DragValue::new(&mut value).speed(1e-2));
        });
    })
    .unwrap();
}
