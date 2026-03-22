use egui::{
    epaint::CubicBezierShape, global_theme_preference_buttons, Color32, DragValue, Painter, Pos2,
    Shape, Stroke, Vec2,
};

pub fn circular_arc(
    painter: &Painter,
    center: Pos2,
    radius: f32,
    begin_angle: f32,
    end_angle: f32,
    n_segments: usize,
    stroke: Stroke,
) {
    for points in circular_arc_beziers(center, radius, begin_angle, end_angle, n_segments) {
        let shape =
            CubicBezierShape::from_points_stroke(points, false, Color32::TRANSPARENT, stroke);
        painter.add(Shape::CubicBezier(shape));
    }
}

pub fn circular_arc_beziers(
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

pub fn circular_arc_bezier(
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

pub fn arc_from_derivatives(
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
