use std::f32::consts::FRAC_PI_2;

use egui::{
    Color32, CornerRadius, InnerResponse, Pos2, Stroke, Style, Ui, Vec2,
};

use crate::utils::circular_arc_stroke;

pub struct GroupBox {
    frame: egui::Frame,
    label: String,
    text_color: Option<Color32>,
    text_margin: f32,
}

impl GroupBox {
    pub fn new(style: &Style, label: impl Into <String>) -> Self {
        Self::from_frame(egui::Frame::default(), style, label)
    }

    pub fn from_frame(frame: egui::Frame, style: &Style, label: impl Into<String>) -> Self {
        // Same as frame.group()
        let frame = frame
            .inner_margin(6)
            .corner_radius(style.visuals.widgets.noninteractive.corner_radius)
            .stroke(style.visuals.widgets.noninteractive.bg_stroke);

        Self::from_frame_raw(frame, label)
    }

    pub fn from_frame_raw(frame: egui::Frame, label: impl Into<String>) -> Self {
        // Allow enough margin height for the text
        let height = egui::FontId::default().size;
        let mut margin = frame.outer_margin;
        margin.top = margin.top.max(height as i8);

        let frame = frame.outer_margin(margin);

        Self {
            frame,
            label: label.into(),
            text_color: None,
            text_margin: 5.0,
        }
    }

    pub fn text_margin(mut self, margin: f32) -> Self {
        self.text_margin = margin;
        self
    }

    pub fn text_color(mut self, text_color: Color32) -> Self {
        self.text_color = Some(text_color);
        self
    }

    pub fn stroke(mut self, stroke: Stroke) -> Self {
        self.frame.stroke = stroke;
        self
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        // Surpress existing stroke
        let frame = self.frame.clone().stroke(Stroke::NONE);
        let corners = frame.corner_radius;

        // Add inside of widget
        let mut prepared = frame.begin(ui);
        let ret = add_contents(&mut prepared.content_ui);
        let mut widget_rect = prepared.frame.widget_rect(prepared.content_ui.min_rect());
        let response = prepared.end(ui);

        let painter = ui.painter();

        // Add text
        let text_color = self.get_text_color(ui.style());
        let text_rect = painter.text(
            widget_rect.min + Vec2::X * (self.text_margin + corners.nw as f32),
            egui::Align2::LEFT_BOTTOM,
            &self.label,
            Default::default(),
            text_color,
        );

        // Make the outline 'intersect' the text
        widget_rect.min.y -= text_rect.height() / 2.0;

        let stroke = self.frame.stroke;
        let w2 = stroke.width / 2.0;
        let tl = widget_rect.left_top() + Vec2::new(-w2, -w2);
        let tr = widget_rect.right_top() + Vec2::new(w2, -w2);
        let bl = widget_rect.left_bottom() + Vec2::new(-w2, w2);
        let br = widget_rect.right_bottom() + Vec2::new(w2, w2);

        let tl_radius = self.frame.corner_radius.nw as f32;
        let tr_radius = self.frame.corner_radius.ne as f32;
        let bl_radius = self.frame.corner_radius.sw as f32;
        let br_radius = self.frame.corner_radius.se as f32;

        painter.line_segment(
            [
                tl + Vec2::X * tl_radius,
                Pos2::new((text_rect.min.x - self.text_margin / 4.0).min(tr.x), tl.y),
            ],
            stroke,
        );
        painter.line_segment(
            [
                Pos2::new((text_rect.max.x + self.text_margin / 4.0).min(tr.x), tl.y),
                tr - Vec2::X * tr_radius,
            ],
            stroke,
        );

        if corners != CornerRadius::ZERO {
            let ninety = FRAC_PI_2;
            for (start, stop, coord) in [
                (
                    ninety * 2.0,
                    ninety * 3.0,
                    tl + Vec2::new(corners.nw as f32, corners.nw as f32),
                ),
                (
                    ninety * 3.0,
                    ninety * 4.0,
                    tr + Vec2::new(-(corners.ne as f32), corners.ne as f32),
                ),
                (
                    ninety * 1.0,
                    ninety * 2.0,
                    bl + Vec2::new(corners.sw as f32, -(corners.sw as f32)),
                ),
                (
                    ninety * 0.0,
                    ninety * 1.0,
                    br + Vec2::new(-(corners.se as f32), -(corners.se as f32)),
                ),
            ] {
                circular_arc_stroke(painter, coord, corners.nw as _, start, stop, 1.0, stroke);
            }
        }

        painter.line_segment([bl + Vec2::X * bl_radius, br - Vec2::X * br_radius], stroke);

        painter.line_segment([tl + Vec2::Y * tl_radius, bl - Vec2::Y * tr_radius], stroke);

        painter.line_segment([tr + Vec2::Y * bl_radius, br - Vec2::Y * br_radius], stroke);

        InnerResponse::new(ret, response)
    }

    fn get_text_color(&self, style: &Style) -> Color32 {
        self.text_color
            .unwrap_or(style.visuals.widgets.noninteractive.bg_stroke.color)
    }
}

pub trait FrameGroupBoxExt {
    fn group_box(self, label: impl Into<String>) -> GroupBox;
}

impl FrameGroupBoxExt for egui::Frame {
    fn group_box(self, label: impl Into<String>) -> GroupBox {
        GroupBox::from_frame_raw(self, label)
    }
}

pub trait UiGroupBoxExt {
    fn group_box<R>(
        self,
        label: impl Into<String>,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R>;
}

impl UiGroupBoxExt for &mut Ui {
    fn group_box<R>(
        self,
        label: impl Into<String>,
        add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
        GroupBox::new(self.style(), label.into()).show(self, add_contents)
    }
}

pub fn group_box<R>(
    ui: &mut Ui, 
    label: impl Into<String>, 
    add_contents: impl FnOnce(&mut Ui) -> R,
    ) -> InnerResponse<R> {
    ui.group_box(label, add_contents)
}
