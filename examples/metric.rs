use egui::{Layout, global_theme_preference_buttons};
use egui_simpletabs::{metric::{edit_metric_f64, metric_prefix_dragvalue}, tabs::TabWidgetExt};

#[derive(Default, Clone, Copy, PartialEq)]
enum Tabs {
    #[default]
    Home,
    TabOne,
    TabTwo,
}

fn main() {
    let mut voltage = 1e-6;
    let mut amperage = 1000.0;

    let options = eframe::NativeOptions::default();
    eframe::run_simple_native("Metric test", options, move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.add(edit_metric_f64(&mut voltage, "V"));
            ui.add(edit_metric_f64(&mut amperage, "A"));
        });

    })
    .unwrap();
}

