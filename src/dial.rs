use std::ops::RangeInclusive;

use egui::{Color32, Pos2, Rect, Response, Sense, Ui, Vec2, Widget, emath::Numeric};

use crate::utils::circular_arc_stroke;

type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<f64>) -> f64>;
fn get(get_set_value: &mut GetSetValue<'_>) -> f64 {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: f64) {
    (get_set_value)(Some(value));
}

/// A marked position on the dial
pub struct DialPosition {
    label: Option<String>,
    color: Option<Color32>,
    value: f64,
    line_length: Option<f32>,
    underline: bool,
    /// Whether to snap to this position, and if so, how close (in radians)
    /// Not needed when using integer positions
    snap: Option<f32>,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum DragMode {
    DistanceFromCenter,
    CoordinateY,
    CoordinateX,
    #[default]
    Radial,
}

/// A Dial widget
pub struct Dial<'a> {
    get_set_value: GetSetValue<'a>,
    /// Change in angle (in radians) per change in mouse position
    pub mouse_sensitivity: f64,
    /// Angle (in radians) at which the dial is at the "origin".
    pub origin_angle: f64,
    // /// Value at the origin
    // pub origin_value: f64,
    /// Change in value per change in angle (radians)
    pub value_per_angle: f64,
    /// The maximum value allowed (if any)
    pub max_value: Option<f64>,
    /// The minimum value allowed (if any)
    pub min_value: Option<f64>,
    /// The desired size of the widget
    pub desired_size: Vec2,
    /// The radius of the knob
    pub knob_radius: f32,
    /// The way the mouse drag works
    pub drag_mode: DragMode,
    /// Display a circular arc around the active area of the dial,
    /// at the specific distance from the knob (if any)
    pub show_livezone: bool,
    /// Marked dial positions
    pub positions: Vec<DialPosition>,
    /// How far away any markings are fromt he dial
    pub markings_offset: f32,
}

impl<'a> Dial<'a> {
    /// Creates a new dial with the default range and no clamping
    pub fn new<Num: Numeric>(value: &'a mut Num) -> Self {
        Self::from_get_set(move |v: Option<f64>| {
            if let Some(v) = v {
                *value = Num::from_f64(v);
            }
            value.to_f64()
        })
    }

    pub fn from_get_set(get_set_value: impl 'a + FnMut(Option<f64>) -> f64) -> Self {
        let knob_radius: f32 = 25.0;
        Self {
            get_set_value: Box::new(get_set_value),
            mouse_sensitivity: 5e-2,
            origin_angle: -std::f64::consts::FRAC_PI_2,
            //origin_value: 0.0,
            value_per_angle: 1.0,
            min_value: None,
            max_value: None,
            desired_size: Vec2::new(200.0, 100.0),
            knob_radius,
            drag_mode: DragMode::default(),
            show_livezone: true,
            positions: Vec::new(),
            markings_offset: 5.0,
        }
    }

    /// How much the dial position (in radians) changes when dragged one point (logical pixel).
    ///
    /// Should be finite and greater than zero.
    pub fn mouse_sensitivity(mut self, speed: impl Into<f64>) -> Self {
        self.mouse_sensitivity = speed.into();
        self
    }

    /// Sets the value at the origin
    pub fn origin_value<Num: Numeric>(mut self, value: Num) -> Self {
        self.origin_angle = self.angle_for_value(value.to_f64());
        self
    }

    /// Sets the angle (in radians) at the origin
    pub fn origin_angle(mut self, angle: f64) -> Self {
        self.origin_angle = angle;
        self
    }

    /// Sets the amount the value changes for each radian turned. See also `Self::mouse_sensitivity`.
    pub fn value_per_radian(mut self, value: f64) -> Self {
        self.value_per_angle = value.to_f64();
        self
    }

    /// Sets the min value
    pub fn min_value<Num: Numeric>(mut self, value: Option<Num>) -> Self {
        self.min_value = value.map(|v| v.to_f64());
        self
    }

    /// Sets the max value
    pub fn max_value<Num: Numeric>(mut self, value: Option<Num>) -> Self {
        self.max_value = value.map(|v| v.to_f64());
        self
    }

    /// Sets the desired widget size
    pub fn desired_size(mut self, size: Vec2) -> Self {
        self.desired_size = size;
        self
    }

    /// Sets the radius of the knob
    pub fn knob_radius(mut self, size: f32) -> Self {
        self.knob_radius = size;
        self
    }

    /// Sets the mode in which the mouse affects the radial position
    pub fn drag_mode(mut self, mode: DragMode) -> Self {
        self.drag_mode = mode;
        self
    }

    /// Whether to invert the direction of rotation (Defaults to clockwise = increase)
    pub fn invert(mut self, invert: bool) -> Self {
        if invert {
            self.value_per_angle *= -1.0;
        }
        self
    }

    /// Display a circular arc around the active area of the dial,
    /// at the specific distance from the knob (if any)
    pub fn show_livezone(mut self, livezone: bool) -> Self {
        self.show_livezone = livezone;
        self
    }

    /// How far away any markings are fromt he dial
    pub fn markings_offset(mut self, offset: f32) -> Self {
        self.markings_offset = offset;
        self
    }

    /// Shorthand for distributing the range of values between min and max, optionally avoiding
    /// 'deadzone' radians (leaving that as unreachable space between the max and min values)
    pub fn range<Num: Numeric>(self, range: RangeInclusive<Num>, deadzone: Option<f64>) -> Self {
        let usable_radians = std::f64::consts::TAU - deadzone.unwrap_or(0.0);
        let value_range = range.end().to_f64() - range.start().to_f64();
        self.min_value(Some(*range.start()))
            .max_value(Some(*range.end()))
            .origin_value(*range.start())
            .value_per_radian(value_range / usable_radians)
    }

    /// Add a marked position to the dial
    pub fn with_position(mut self, position: DialPosition) -> Self {
        self.positions.push(position);
        self
    }

    /// Returns the angle for a given value
    fn value_for_angle(&self, angle: f64) -> f64 {
        (angle - self.origin_angle) * self.value_per_angle // + self.origin_value
    }

    /// Returns the angle for a given value
    fn angle_for_value(&self, value: f64) -> f64 {
        //(value - self.origin_value) / self.value_per_angle + self.origin_angle
        value / self.value_per_angle + self.origin_angle
    }
}

impl Widget for Dial<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        // Background response
        let background_area = Rect::from_min_size(ui.cursor().min, self.desired_size);

        // Knob
        let center = background_area.center();
        let mut value = get(&mut self.get_set_value);
        let angle = self.angle_for_value(value);
        draw_knob(ui, center, angle, self.knob_radius);

        let knob_rect = Rect::from_center_size(
            center,
            Vec2::splat((self.knob_radius + self.markings_offset) * 2.0),
        );

        let knob_resp = ui.allocate_rect(knob_rect, egui::Sense::click_and_drag());

        let stroke = ui.style().visuals.widgets.active.fg_stroke;

        // Arc around knob
        if let (Some(min), Some(max), true) = (self.min_value, self.max_value, self.show_livezone) {
            circular_arc_stroke(
                ui.painter(),
                center,
                self.knob_radius + self.markings_offset,
                self.angle_for_value(min) as f32,
                self.angle_for_value(max) as f32,
                1.0,
                stroke,
            );
        }

        // Interaction with knob
        if let Some(mouse_pos) = knob_resp.interact_pointer_pos()
            && knob_resp.dragged()
        {
            let delta = self
                .drag_mode
                .calculate_delta(mouse_pos - center, knob_resp.drag_delta());
            value += delta as f64 * self.mouse_sensitivity * self.value_per_angle;

            if let Some(max) = self.max_value {
                value = value.min(max);
            }

            if let Some(min) = self.min_value {
                value = value.max(min);
            }

            set(&mut self.get_set_value, value);
        }

        let angle = self.angle_for_value(value);

        // Positions
        for position in &self.positions {
            let mut position_stroke = stroke.clone();
            if let Some(color) = position.color {
                position_stroke.color = color;
            }

            let pos_angle = self.angle_for_value(position.value);

            // Snap
            if let Some(snap_thresh) = position.snap
                && knob_resp.dragged()
            {
                if (pos_angle - angle).abs() < snap_thresh as f64 {
                    value = position.value;
                    set(&mut self.get_set_value, value);
                }
            }

            // Drawing
            let vect = Vec2::angled(pos_angle as _);

            let r = self.markings_offset + self.knob_radius;
            let p1 = center + vect * r;
            let p2 = position
                .line_length
                .map(|len| center + vect * (r + len))
                .unwrap_or(p1);

            ui.painter().line_segment([p1, p2], position_stroke);

            // Label
            if let Some(label) = &position.label {
                let anchor = egui::Align2([-vect.x, vect.y].map(|v| {
                    if v < 0.0 {
                        egui::Align::Min
                    } else {
                        egui::Align::Max
                    }
                }));

                let rect =
                    ui.painter()
                        .text(p2, anchor, label, Default::default(), position_stroke.color);

                if position.underline {
                    let p3 = p2 + Vec2::new(rect.width(), 0.0) * vect.x.signum();
                    ui.painter().line_segment([p2, p3], position_stroke);
                }

                let label_resp = ui.allocate_rect(rect, Sense::click());
                if label_resp.double_clicked() {
                    value = position.value;
                    set(&mut self.get_set_value, value);
                }
            }
        }

        ui.advance_cursor_after_rect(background_area);

        knob_resp
    }
}

fn draw_knob(ui: &Ui, center: Pos2, angle: f64, radius: f32) {
    let stroke = ui.style().visuals.widgets.active.fg_stroke;
    ui.painter().circle_stroke(center, radius, stroke.clone());

    let fill = ui.style().visuals.widgets.noninteractive.bg_stroke.color;
    ui.painter().circle_filled(center, radius, fill);

    let f = |r: f32| center + Vec2::angled(angle as _) * r;

    ui.painter()
        .line_segment([f(radius / 2.0), f(radius)], stroke);
}

impl DragMode {
    pub fn calculate_delta(&self, relpos: Vec2, drag_delta: Vec2) -> f32 {
        match self {
            Self::Radial => cross2d(relpos.normalized(), drag_delta),
            Self::DistanceFromCenter => relpos.normalized().dot(drag_delta),
            Self::CoordinateY => drag_delta.y,
            Self::CoordinateX => drag_delta.x,
        }
    }
}

fn cross2d(a: Vec2, b: Vec2) -> f32 {
    a.x * b.y - a.y * b.x
}

impl DialPosition {
    pub fn new<Num: Numeric>(value: Num) -> Self {
        Self {
            value: value.to_f64(),
            snap: None,
            label: None,
            underline: false,
            line_length: Some(20.0),
            color: None,
        }
    }

    pub fn snap(mut self, snap_threshold_radians: Option<f32>) -> Self {
        self.snap = snap_threshold_radians;
        self
    }

    pub fn label(mut self, label: impl Into<String>) -> Self {
        self.label = Some(label.into());
        self
    }

    pub fn underline(mut self, underline: bool) -> Self {
        self.underline = underline;
        self
    }

    pub fn line_length(mut self, length: Option<f32>) -> Self {
        self.line_length = length;
        self
    }

    pub fn color(mut self, color: Color32) -> Self {
        self.color = Some(color);
        self
    }
}
