use std::f32::consts::{FRAC_PI_2, PI};

use egui::{
    Color32, CornerRadius, InnerResponse, Response, Stroke, Style, TextStyle, Ui, Vec2, WidgetText,
};

use crate::utils::circular_arc_stroke;

pub struct GroupBox {
    frame: egui::Frame,
    label: String,
    text_color: Option<Color32>,
}

impl GroupBox {
    pub fn new(label: String) -> Self {
        Self::from_frame(egui::Frame::default(), label)
    }

    pub fn from_frame(frame: egui::Frame, label: impl Into<String>) -> Self {
        // Allow enough height for the text
        let height = egui::FontId::default().size;
        let mut margin = frame.outer_margin;
        margin.top = margin.top.max(height as i8);
        let frame = frame.outer_margin(margin);

        Self {
            frame,
            label: label.into(),
            text_color: None,
        }
    }

    pub fn text_color(mut self, text_color: Color32) {
        self.text_color = Some(text_color);
    }

    pub fn show<R>(self, ui: &mut Ui, add_contents: impl FnOnce(&mut Ui) -> R) -> InnerResponse<R> {
        // Surpress existing stroke
        let frame = self.frame.clone().stroke(Stroke::NONE);
        let corners = frame.corner_radius;

        // Add
        let mut prepared = frame.begin(ui);
        let ret = add_contents(&mut prepared.content_ui);
        let widget_rect = prepared.frame.widget_rect(prepared.content_ui.min_rect());
        let response = prepared.end(ui);

        let painter = ui.painter();

        let text_color = self.get_text_color(ui.style());
        let text_rect = painter.text(
            widget_rect.min,
            egui::Align2::LEFT_BOTTOM,
            &self.label,
            Default::default(),
            text_color,
        );

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

        painter.line_segment([tl + Vec2::X * tl_radius, tr - Vec2::X * tr_radius], stroke);

        let ninety = FRAC_PI_2;
        for (start, stop, coord) in [
            (ninety * 2.0, ninety * 3.0, tl + Vec2::new(corners.nw as f32, corners.nw as f32)),
            (ninety * 3.0, ninety * 4.0, tr + Vec2::new(-(corners.ne as f32), corners.ne as f32)),
            (ninety * 1.0, ninety * 2.0, bl + Vec2::new(corners.sw as f32, -(corners.sw as f32))),
            (ninety * 0.0, ninety * 1.0, br + Vec2::new(-(corners.se as f32), -(corners.se as f32))),
        ] {
            circular_arc_stroke(
                painter,
                coord,
                corners.nw as _,
                start,
                stop,
                1.0,
                stroke,
            );
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
    fn groupbox(self, label: impl Into<String>) -> GroupBox;
}

impl FrameGroupBoxExt for egui::Frame {
    fn groupbox(self, label: impl Into<String>) -> GroupBox {
        GroupBox::from_frame(self, label)
    }
}
