use egui::{Color32, DragValue, Painter, Pos2, Shape, Stroke, Vec2, epaint::CubicBezierShape, global_theme_preference_buttons};

pub fn circular_arc(painter: &Painter, center: Pos2, radius: f32, begin_angle: f32, end_angle: f32, n_segments: usize, stroke: Stroke) {
    for points in circular_arc_beziers(center, radius, begin_angle, end_angle, n_segments) {
        let shape = CubicBezierShape::from_points_stroke(points, false, Color32::TRANSPARENT, stroke);
        painter.add(Shape::CubicBezier(shape));
    }
}

pub fn circular_arc_beziers(center: Pos2, radius: f32, begin_angle: f32, end_angle: f32, n_segments: usize) -> impl Iterator<Item=[Pos2; 4]> {
    let total_angle = end_angle - begin_angle;
    let angle_step = total_angle / n_segments as f32;
    (0..n_segments).map(move |i| circular_arc_bezier(center, radius, begin_angle + angle_step * i as f32, begin_angle + angle_step * (i+1) as f32))
}

pub fn circular_arc_bezier(center: Pos2, radius: f32, begin_angle: f32, end_angle: f32) -> [Pos2; 4] {
    let begin_vect = Vec2::angled(begin_angle);
    let end_vect = Vec2::angled(end_angle);
    arc_from_derivatives(
        center + begin_vect * radius, 
        begin_vect.rot90(),
        center + end_vect * radius, 
        end_vect.rot90(),
    )
}

pub fn arc_from_derivatives(
    begin_pos: Pos2, begin_deriv: Vec2, 
    end_pos: Pos2, end_deriv: Vec2, 
) -> [Pos2; 4] {
    [
        begin_pos,
        begin_pos + begin_deriv / 3.0,
        end_pos + end_deriv / 3.0,
        end_pos,
    ]
}

