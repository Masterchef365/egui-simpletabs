use egui_simpletabs::buttons::*;

fn main() {
    let mut paused = false;
    eframe::run_simple_native("Button test", Default::default(), move |ctx, _frame| {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                play_pause_button(ui, &mut paused);
                single_step_button(ui);
                reset_step_button(ui);
                serious_button(ui, "U");
            });
        });
    })
    .unwrap();
}


