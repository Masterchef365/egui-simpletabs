//! Skeuomorphic dials
use std::ops::RangeInclusive;

use egui::{
    Color32, Painter, Pos2, Rect, Response, Sense, Shape, Stroke, Ui, Vec2, Widget, emath::Numeric,
    epaint::CubicBezierShape,
};

use crate::utils::{circular_arc_bezier, circular_arc_stroke, throttle};

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

/// How a drag motion affects the knob
#[derive(Copy, Clone, Debug, PartialEq)]
pub enum KnobStyle {
    Circular,
    Fluted {
        n_segments: usize,
        /// 0 to 1
        depth: f32,
    },
}

/// How a drag motion affects the knob
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum DragMode {
    DistanceFromCenter,
    CoordinateY,
    CoordinateX,
    #[default]
    Radial,
}

/// Which positions are valid for the knob
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum TurningMode {
    /// Knob can move in to any position
    #[default]
    Analog,
    /// Knob can only reside at defined 'positions'
    Positional,
    /// Knob can only reside at integers
    Integral,
}

/// A series of marks on the dial, on an interval.
#[derive(Copy, Clone)]
pub struct ScaleMarking {
    pub length: f32,
    pub interval: f64,
    pub stroke: Option<Stroke>,
}

/// A Dial widget
/// ```rust
/// let dial = Dial::new(&mut value)
///     .drag_mode(drag_mode)
///     .value_per_radian(value_per_radian)
///     .min_value(min.into_option())
///     .max_value(max.into_option())
///     .invert(invert)
///     .origin_angle(origin_angle)
///     .origin_value(origin_value)
///     .mouse_sensitivity(mouse_sensitivity)
///     .show_livezone(show_livezone)
///     .with_scale_marking(ScaleMarking::default().with_interval(0.5))
///     .with_position(
///         DialPosition::new(0)
///             .label("Zero")
///             .snap(snap.into())
///             .underline(underline)
///             .color(Color32::DARK_GREEN),
///     )
///     .with_position(
///         DialPosition::new(1)
///             .label("One")
///             .snap(snap.into())
///             .underline(underline),
///     );
///
/// ui.add(dial);
/// ```
pub struct Dial<'a> {
    get_set_value: GetSetValue<'a>,
    /// Change in angle (in radians) per change in mouse position
    pub mouse_sensitivity: f64,
    /// Angle (in radians) at which the dial is at the "origin".
    pub origin_angle: f64,
    /// Value at the origin
    pub origin_value: f64,
    /// Change in value per change in angle (radians)
    pub value_per_radian: f64,
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
    /// How far away any markings are fromt he dial
    pub markings_offset: f32,
    /// How is the knob free to move?
    pub turning_mode: TurningMode,
    /// How many discrete moves the dial may make per second
    pub throttle_turn_rate: f64,
    /// Knob style
    pub knob_style: KnobStyle,
    /// Scale markings
    scale_markings: Vec<ScaleMarking>,
    /// Marked dial positions
    positions: Vec<DialPosition>,
}

impl<'a> Dial<'a> {
    /// Creates a new dial with the default range and no clamping
    pub fn new<Num: Numeric>(value: &'a mut Num) -> Self {
        Self::from_get_set::<Num>(move |v: Option<f64>| {
            if let Some(v) = v {
                *value = Num::from_f64(v);
            }
            value.to_f64()
        })
    }

    fn from_get_set<Num: Numeric>(get_set_value: impl 'a + FnMut(Option<f64>) -> f64) -> Self {
        let knob_radius: f32 = 25.0;
        Self {
            knob_style: KnobStyle::default(),
            get_set_value: Box::new(get_set_value),
            mouse_sensitivity: match Num::INTEGRAL {
                true => 1.0,
                false => 5e-2,
            },
            origin_angle: -std::f64::consts::FRAC_PI_2,
            origin_value: 0.0,
            value_per_radian: 1.0,
            min_value: None,
            max_value: None,
            desired_size: Vec2::new(200.0, 100.0),
            knob_radius,
            drag_mode: DragMode::default(),
            show_livezone: true,
            positions: Vec::new(),
            markings_offset: 5.0,
            turning_mode: match Num::INTEGRAL {
                true => TurningMode::Integral,
                false => TurningMode::Analog,
            },
            throttle_turn_rate: 5.0,
            scale_markings: vec![],
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
    pub fn origin_angle(mut self, angle: f64) -> Self {
        self.origin_angle = angle;
        self
    }

    /// Sets the amount the value changes for each radian turned. See also `Self::mouse_sensitivity`.
    pub fn value_per_radian(mut self, value: f64) -> Self {
        self.value_per_radian = value.to_f64();
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
            self.value_per_radian *= -1.0;
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

    /// How many discrete moves the dial may make per second
    pub fn throttle_turn_rate(mut self, rate: f64) -> Self {
        self.throttle_turn_rate = rate;
        self
    }

    /// How is the knob free to move?
    pub fn turning_mode(mut self, mode: TurningMode) -> Self {
        self.turning_mode = mode;
        self
    }

    /// How does the knob look?
    pub fn knob_style(mut self, knob_style: KnobStyle) -> Self {
        self.knob_style = knob_style;
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

        // We sort here, so that we can easily traverse for indices
        self.positions.sort_by(|a, b| {
            a.value
                .partial_cmp(&b.value)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        self
    }

    /// Add a series of markings to the dial
    pub fn with_scale_marking(mut self, marking: ScaleMarking) -> Self {
        self.scale_markings.push(marking);
        self
    }

    /// Returns the angle for a given value
    fn value_for_angle(&self, angle: f64) -> f64 {
        (angle - self.origin_angle) * self.value_per_radian + self.origin_value
    }

    /// Returns the angle for a given value
    fn angle_for_value(&self, value: f64) -> f64 {
        (value - self.origin_value) / self.value_per_radian + self.origin_angle
    }
}

impl Widget for Dial<'_> {
    fn ui(mut self, ui: &mut Ui) -> Response {
        // Background response
        let background_area = Rect::from_min_size(ui.cursor().min, self.desired_size);

        let center = background_area.center();

        // Scale markings
        if let (Some(min), Some(max)) = (self.min_value, self.max_value) {
            for marking in &self.scale_markings {
                let n = ((max - min) / marking.interval).floor() as usize;

                let offset = (min / marking.interval).ceil() * marking.interval;

                for i in 0..n {
                    let angle = self.angle_for_value(offset + marking.interval * i as f64);
                    let r_inner = self.knob_radius + self.markings_offset;
                    let r_outer = r_inner + marking.length;

                    let p_inner = center + Vec2::angled(angle as _) * r_inner;
                    let p_outer = center + Vec2::angled(angle as _) * r_outer;
                    let stroke = marking
                        .stroke
                        .unwrap_or(ui.style().visuals.widgets.noninteractive.fg_stroke);

                    ui.painter().line_segment([p_inner, p_outer], stroke);
                }
            }
        }

        // Knob
        let mut value = get(&mut self.get_set_value);
        let angle = self.angle_for_value(value);
        self.knob_style.draw(ui, center, angle, self.knob_radius);

        let knob_rect = Rect::from_center_size(
            center,
            Vec2::splat((self.knob_radius + self.markings_offset) * 2.0),
        );

        let mut knob_resp = ui.allocate_rect(knob_rect, egui::Sense::click_and_drag());

        let stroke = ui.style().visuals.widgets.active.fg_stroke;

        // Interaction with knob
        if let Some(mouse_pos) = knob_resp.interact_pointer_pos()
            && knob_resp.dragged()
        {
            let delta = self
                .drag_mode
                .calculate_delta(mouse_pos - center, knob_resp.drag_delta())
                as f64;

            match self.turning_mode {
                TurningMode::Analog => {
                    value += delta * self.mouse_sensitivity * self.value_per_radian;
                    knob_resp.mark_changed();
                }
                TurningMode::Integral => {
                    if delta.abs() * self.mouse_sensitivity > 1.0 {
                        if throttle(ui.ctx(), "knob", self.throttle_turn_rate) {
                            value += delta.signum();
                            knob_resp.mark_changed();
                        }
                    }
                }
                TurningMode::Positional => {
                    if delta.abs() * self.mouse_sensitivity > 1.0 {
                        if throttle(ui.ctx(), "knob", self.throttle_turn_rate) {
                            let mut closest_idx = None;
                            let mut closest_diff = f64::INFINITY;
                            for (idx, position) in self.positions.iter().enumerate() {
                                let diff = (position.value - value).abs();
                                if diff < closest_diff {
                                    closest_idx = Some(idx);
                                    closest_diff = diff;
                                }
                            }
                            let closest_idx = closest_idx.unwrap_or(0);

                            let next_idx = if delta > 0.0 {
                                let next = closest_idx + 1;
                                (next < self.positions.len()).then(|| next)
                            } else {
                                closest_idx.checked_sub(1)
                            };

                            if let Some(next) = next_idx {
                                value = self.positions[next].value;
                                knob_resp.mark_changed();
                            }
                        }
                    }
                }
            }

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

        ui.advance_cursor_after_rect(background_area);

        knob_resp
    }
}

fn draw_fancy_knob(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    begin_angle: f32,
    end_angle: f32,
    n_segments: usize,
    depth: f32,
    stroke: Stroke,
) {
    let total_angle = end_angle - begin_angle;
    let angle_step = total_angle / n_segments as f32;

    let begin_angle = begin_angle - angle_step / 2.0;

    for i in 0..n_segments {
        let a1 = begin_angle + angle_step * i as f32;
        let a2 = begin_angle + angle_step * (i + 1) as f32;
        if i & 1 == 0 {
            let points = circular_arc_bezier(center, radius, a1, a2, angle_step);

            let shape =
                CubicBezierShape::from_points_stroke(points, false, Color32::TRANSPARENT, stroke);
            painter.add(Shape::CubicBezier(shape));
        } else {
            let v1 = Vec2::angled(a1);
            let p1 = center + v1 * radius;

            let v2 = Vec2::angled(a2);
            let p2 = center + v2 * radius;

            let points = [p1, p1.lerp(center, depth), p2.lerp(center, depth), p2];

            let shape =
                CubicBezierShape::from_points_stroke(points, false, Color32::TRANSPARENT, stroke);
            painter.add(Shape::CubicBezier(shape));
        }
    }
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
            underline: true,
            line_length: Some(20.0),
            color: None,
        }
    }

    /// Applies only to Analog TurningMode; snap to the value if within the given number of
    /// radians.
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

impl ScaleMarking {
    /// Set the length of the line segments in the scale markings
    pub fn with_length(mut self, length: f32) -> Self {
        self.length = length;
        self
    }

    /// Set the numeric interval between the markings
    pub fn with_interval<Num: Numeric>(mut self, interval: Num) -> Self {
        self.interval = interval.to_f64();
        self
    }

    pub fn with_stroke(mut self, stroke: Stroke) -> Self {
        self.stroke = Some(stroke);
        self
    }
}

impl Default for ScaleMarking {
    fn default() -> Self {
        Self {
            length: 10.0,
            interval: 1.0,
            stroke: None,
        }
    }
}

impl Default for KnobStyle {
    fn default() -> Self {
        Self::Fluted {
            n_segments: 12,
            depth: 0.1,
        }
    }
}

impl KnobStyle {
    pub fn draw(&self, ui: &Ui, center: Pos2, angle: f64, radius: f32) {
        let fill = ui.style().visuals.widgets.noninteractive.bg_stroke.color;
        ui.painter().circle_filled(center, radius, fill);

        let stroke = ui.style().visuals.widgets.active.fg_stroke;

        match self {
            KnobStyle::Circular => {
                ui.painter().circle_stroke(center, radius, stroke.clone());
            }
            KnobStyle::Fluted { n_segments, depth } => {
                draw_fancy_knob(
                    ui.painter(),
                    center,
                    radius,
                    angle as f32 + 0.0,
                    angle as f32 + std::f32::consts::TAU,
                    *n_segments,
                    *depth,
                    stroke,
                );
            }
        }

        let f = |r: f32| center + Vec2::angled(angle as _) * r;

        ui.painter()
            .line_segment([f(radius / 2.0), f(radius)], stroke);
    }
}

pub fn choice<T: PartialEq + Clone>(ui: &mut Ui, value: &mut T, choices: &[(T, &'static str)]) {
    let mut idx = choices.iter().position(|(v, _)| v == value).unwrap();
    let spacing =  
        choices.len().max(6) as f64 / std::f64::consts::TAU;

    let mut dial = Dial::new(&mut idx).value_per_radian(spacing).turning_mode(TurningMode::Positional);
    for (idx, (_key, label)) in choices.iter().enumerate() {
        dial = dial.with_position(DialPosition::new(idx).label(*label));
    }

    let resp = ui.add(dial);

    if resp.changed() {
        *value = choices[idx].0.clone();
    }
}
