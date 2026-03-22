use egui::{Color32, RichText, Ui};

pub fn play_pause_button(ui: &mut Ui, paused: &mut bool) {
    let text = match *paused {
        true => "   ▶   ",
        false => "   ⏸   ",
    };

    let hover_text = match *paused {
        true => "Play",
        false => "Pause",
    };

    let color = match *paused {
        true => Color32::DARK_RED,
        false => Color32::from_rgb(79, 200, 0),
    };

    let text = RichText::new(text).color(Color32::WHITE);

    let button = egui::Button::new(text).fill(color).min_size(BUTTON_SIZE);

    let resp = ui.add(button).on_hover_text(hover_text);
    if resp.clicked() {
        *paused = !*paused;
    }
}

const BUTTON_SIZE: egui::Vec2 = egui::Vec2::splat(40.0);

pub fn single_step_button(ui: &mut Ui) -> egui::Response {
    serious_button(ui, "⏭").on_hover_text("Single-step")
}

pub fn reset_step_button(ui: &mut Ui) -> egui::Response {
    serious_button(ui, "↺").on_hover_text("Reset")
}

pub fn serious_button(ui: &mut Ui, symbol: &str) -> egui::Response {
    let text = RichText::new(format!("   {symbol}   ")).color(Color32::WHITE);

    let button = egui::Button::new(text).min_size(BUTTON_SIZE);

    ui.add(button)
}
