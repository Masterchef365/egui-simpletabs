use std::ops::Mul;

use egui::{Color32, Painter, Pos2, Response, Shape, Stroke, Ui, Vec2, epaint::CubicBezierShape};

pub fn circular_arc_stroke(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    mut begin_angle: f32,
    mut end_angle: f32,
    resolution: f32,
    stroke: Stroke,
) {
    if begin_angle > end_angle {
        std::mem::swap(&mut begin_angle, &mut end_angle);
    }

    let end_angle = begin_angle + (end_angle - begin_angle).clamp(0.0, std::f32::consts::TAU);

    let n_segments = (end_angle - begin_angle)
        .mul(resolution)
        .abs()
        .ceil()
        .clamp(0.0, 100.0) as usize;
    for points in circular_arc_beziers(center, radius, begin_angle, end_angle, n_segments) {
        let shape =
            CubicBezierShape::from_points_stroke(points, false, Color32::TRANSPARENT, stroke);
        painter.add(Shape::CubicBezier(shape));
    }
}

fn circular_arc_beziers(
    center: Pos2,
    radius: f32,
    begin_angle: f32,
    end_angle: f32,
    n_segments: usize,
) -> impl Iterator<Item = [Pos2; 4]> {
    let total_angle = end_angle - begin_angle;
    let angle_step = total_angle / n_segments as f32;
    (0..n_segments).map(move |i| {
        circular_arc_bezier(
            center,
            radius,
            begin_angle + angle_step * i as f32,
            begin_angle + angle_step * (i + 1) as f32,
            angle_step,
        )
    })
}

fn circular_arc_bezier(
    center: Pos2,
    radius: f32,
    begin_angle: f32,
    end_angle: f32,
    angle_step: f32,
) -> [Pos2; 4] {
    let begin_vect = Vec2::angled(begin_angle) * radius;
    let end_vect = Vec2::angled(end_angle) * radius;
    arc_from_derivatives(
        center + begin_vect,
        begin_vect.rot90(),
        center + end_vect,
        end_vect.rot90(),
        angle_step,
    )
}

fn arc_from_derivatives(
    begin_pos: Pos2,
    begin_deriv: Vec2,
    end_pos: Pos2,
    end_deriv: Vec2,
    angle_step: f32,
) -> [Pos2; 4] {
    // https://en.wikipedia.org/wiki/B%C3%A9zier_curve#Properties
    let factor = 4.0 * (angle_step / 4.0).tan() / 3.0;

    [
        begin_pos,
        begin_pos - begin_deriv * factor,
        end_pos + end_deriv * factor,
        end_pos,
    ]
}

#[derive(Clone, Copy)]
pub struct IndecisiveOption<T> {
    pub is_some: bool,
    pub value: T,
}

impl<T: Default> From<Option<T>> for IndecisiveOption<T> {
    fn from(value: Option<T>) -> Self {
        Self {
            is_some: value.is_some(),
            value: value.unwrap_or_default(),
        }
    }
}

impl<T> Into<Option<T>> for IndecisiveOption<T> {
    fn into(self) -> Option<T> {
        self.into_option()
    }
}

impl<T> IndecisiveOption<T> {
    pub fn into_option(self) -> Option<T> {
        self.is_some.then(|| self.value)
    }
}

impl<T: Default> IndecisiveOption<T> {
    pub fn show<F>(&mut self, ui: &mut Ui, show_value: F) -> Response
    where
        F: FnOnce(&mut Ui, &mut T) -> Response,
    {
        ui.horizontal(move |ui| {
            ui.checkbox(&mut self.is_some, "");
            ui.add_enabled(self.is_some, |ui: &mut Ui| show_value(ui, &mut self.value));
        })
        .response
    }
}
