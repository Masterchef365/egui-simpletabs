use std::ops::RangeInclusive;

use egui::{emath::Numeric, Painter, Pos2, Response, Ui, Vec2, Widget};

type GetSetValue<'a> = Box<dyn 'a + FnMut(Option<f64>) -> f64>;
fn get(get_set_value: &mut GetSetValue<'_>) -> f64 {
    (get_set_value)(None)
}

fn set(get_set_value: &mut GetSetValue<'_>, value: f64) {
    (get_set_value)(Some(value));
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum DragMode {
    DistanceFromCenter,
    #[default]
    CoordinateY,
    CoordinateX,
    Radial,
}

pub struct Dial<'a> {
    get_set_value: GetSetValue<'a>,
    /// Change in angle (in radians) per change in mouse position
    pub mouse_sensitivity: f64,
    /// Angle (in radians) at which the dial is at the "origin".
    pub origin_angle: f64,
    /// Value at the origin
    pub origin_value: f64,
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
            mouse_sensitivity: std::f64::consts::TAU / knob_radius as f64,
            origin_angle: std::f64::consts::FRAC_PI_2,
            origin_value: 0.0,
            value_per_angle: 1.0,
            min_value: None,
            max_value: None,
            desired_size: Vec2::new(200.0, 100.0),
            knob_radius,
            drag_mode: DragMode::default(),
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
        self.origin_value = value.to_f64();
        self
    }

    /// Sets the angle (in radians) at the origin
    pub fn origin_angle<Num: Numeric>(mut self, angle: f64) -> Self {
        self.origin_angle = angle;
        self
    }

    /// Sets the amount the value changes for each radian turned. See also `Self::mouse_sensitivity`.
    pub fn value_per_radian(mut self, value: f64) -> Self {
        self.value_per_angle = value.to_f64();
        self
    }

    /// Sets the min value
    pub fn min_value<Num: Numeric>(mut self, value: Num) -> Self {
        self.min_value = Some(value.to_f64());
        self
    }

    /// Sets the max value
    pub fn max_value<Num: Numeric>(mut self, value: Num) -> Self {
        self.max_value = Some(value.to_f64());
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


    /// Shorthand for distributing the range of values between min and max, optionally avoiding
    /// 'deadzone' radians (leaving that as unreachable space between the max and min values)
    pub fn range<Num: Numeric>(self, range: RangeInclusive<Num>, deadzone: Option<f64>) -> Self {
        let usable_radians = std::f64::consts::TAU - deadzone.unwrap_or(0.0);
        let value_range = range.end().to_f64() - range.start().to_f64();
        self.min_value(*range.start())
            .max_value(*range.end())
            .origin_value(*range.start())
            .value_per_radian(value_range / usable_radians)
    }

    /// Returns the angle for a given value
    fn value_for_angle(&mut self, angle: f64) -> f64 {
        (angle - self.origin_angle) * self.value_per_angle + self.origin_value
    }

    /// Returns the angle for a given value
    fn angle_for_value(&mut self, value: f64) -> f64 {
        (value - self.origin_value) / self.value_per_angle + self.origin_angle
    }
}

impl Widget for Dial<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        let resp = ui.allocate_response(self.desired_size, egui::Sense::drag());

        let center = resp.rect.center();
        let value = get(&mut self.get_set_value);
        let angle = self.angle_for_value(value);
        draw_knob(ui, center, angle, self.knob_radius);

        if let Some(mouse_pos) = resp.interact_pointer_pos() && resp.dragged() {
            let delta = self.drag_mode.calculate_delta(mouse_pos - center, resp.drag_delta());
            let mut new = value + delta as f64 * self.mouse_sensitivity * self.value_per_angle;

            if let Some(max) = self.max_value {
                new = new.max(max);
            }

            if let Some(min) = self.min_value {
                new = new.min(min);
            }

            set(&mut self.get_set_value, new);
        }

        resp
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
