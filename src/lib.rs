use egui::{
    Color32, NumExt, Response, RichText, Sense, Shape, Stroke, TextStyle, Ui, Vec2,
    Widget, WidgetInfo, WidgetText, WidgetType,
};

pub struct TabWidget {
    selected: bool,
    text: WidgetText,
}

pub trait TabWidgetExt {
    fn add_tab<Value: PartialEq>(
        &mut self,
        current_value: &mut Value,
        selected_value: Value,
        text: impl Into<WidgetText>,
    ) -> Response;

    fn cap_tabs(&mut self);
}

impl TabWidgetExt for Ui {
    fn add_tab<Value: PartialEq>(
        &mut self,
        current_value: &mut Value,
        selected_value: Value,
        text: impl Into<WidgetText>,
    ) -> Response {
        let mut response = TabWidget::new(*current_value == selected_value, text).ui(self);
        if response.clicked() && *current_value != selected_value {
            *current_value = selected_value;
            response.mark_changed();
        }
        response
    }

    fn cap_tabs(&mut self) {
        let rect = self.available_rect_before_wrap();
        let stroke = self.style().visuals.window_stroke();
        let rect = rect.expand(stroke.width);
        let v = tab_edge_vect(self);
        self.painter().add(Shape::line_segment(
            [rect.left_bottom() - v, rect.right_bottom() + v],
            stroke,
        ));
    }
}

impl TabWidget {
    pub fn new(selected: bool, text: impl Into<WidgetText>) -> Self {
        Self {
            selected,
            text: text.into(),
        }
    }
}

fn tab_edge_vect(ui: &mut Ui) -> Vec2 {
    let button_padding = ui.spacing().button_padding;
    Vec2::X * button_padding.x / 1.25
}

impl Widget for TabWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let Self { selected, text } = self;

        let button_padding = ui.spacing().button_padding;
        let total_extra = button_padding + button_padding;

        let wrap_width = ui.available_width() - total_extra.x;
        let galley = text.into_galley(ui, None, wrap_width, TextStyle::Button);

        let mut desired_size = total_extra + galley.size();
        desired_size.y = desired_size.y.at_least(ui.spacing().interact_size.y);
        let (rect, response) = ui.allocate_at_least(desired_size, Sense::click());
        response.widget_info(|| {
            WidgetInfo::selected(
                WidgetType::SelectableLabel,
                ui.is_enabled(),
                selected,
                galley.text(),
            )
        });

        if ui.is_rect_visible(response.rect) {
            // Figure out where to put the text
            let text_pos = ui
                .layout()
                .align_size_within_rect(galley.size(), rect.shrink2(button_padding))
                .min;

            let visuals = ui.style().interact_selectable(&response, selected);

            // Draw the outline of the tab
            let stroke = if selected {
                visuals.fg_stroke
            } else {
                ui.style().visuals.window_stroke()
            };

            ui.painter().rect_stroke(
                rect,
                0.0,
                if response.hovered() && !selected {
                    Stroke::new(1.0, ui.style().visuals.text_color())
                } else {
                    stroke
                },
                egui::StrokeKind::Outside,
            );

            // Mask over the bottom stroke
            if selected {
                let mut mask = rect;
                mask.min.y = mask.max.y;
                mask.max.y += stroke.width;
                mask = mask.expand2(egui::Vec2::Y * stroke.width);

                ui.painter()
                    .rect_filled(mask, 0.0, ui.style().visuals.window_fill());
            }

            // Draw the bottom bit of the tab
            let v = tab_edge_vect(ui);
            let r = rect.expand(stroke.width);
            ui.painter().add(Shape::line_segment(
                [r.right_bottom(), r.right_bottom() + v],
                stroke,
            ));
            ui.painter().add(Shape::line_segment(
                [r.left_bottom(), r.left_bottom() - v],
                stroke,
            ));

            // Draw the text
            ui.painter().galley(text_pos, galley, visuals.text_color());
        }

        response
    }
}

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

const PREFIXES: [&'static str; 17] = ["y", "z", "a", "f", "p", "n", "μ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y"];

pub const fn first_prefix_exp() -> i32 {
    -3 * (PREFIXES.len() as i32 - 1) / 2
}

pub fn to_metric_prefix(value: f64, unit: impl std::fmt::Display) -> String {
    let exponent = (value.abs().log10() / 3.0).floor() as i32 * 3;
    let idx = (exponent - dbg!(first_prefix_exp())) / 3;

    let prefix = (idx >= 0).then(|| idx as usize).and_then(|i| PREFIXES.get(i));

    if let Some(prefix) = prefix {
        format!("{:.0} {prefix}{unit}", value / 10_f64.powi(exponent))
    } else {
        format!("{:.0} {unit}", value) // Fallback in case exponent is out of range
    }
}

/*
/// Returns (value, unit) for the given string
pub fn from_metric_prefix(s: &str) -> Result<(f64, &str), ()> {
    let prefixes = [
        "y", "z", "a", "f", "p", "n", "μ", "m", "", "k", "M", "G", "T", "P", "E", "Z", "Y",
    ];

    let first_prefix_exp = -24;

    let exponent = (value.abs().log10() / 3.0).floor() as i32 * 3;
    let idx = (exponent - first_prefix_exp) / 3;

    let prefix = (idx >= 0).then(|| idx as usize).and_then(|i| prefixes.get(i));

    if let Some(prefix) = prefix {
        format!("{:.0} {prefix}{unit}", value / 10_f64.powi(exponent))
    } else {
        format!("{:.0} {unit}", value) // Fallback in case exponent is out of range
    }
}
*/

#[test]
fn test_to_metric_prefix() {
    assert_eq!(to_metric_prefix(1000.0, 'V'), "1 kV");
    assert_eq!(to_metric_prefix(0.001, "Ohm"), "1 mOhm");
}
