use crate::lines::Line;
use std::default::Default;
use std::fmt::{Display, Formatter};

#[derive(Default, Debug)]
pub struct Watermark<'a> {
    pub mark_scale: f32,
    pub mark_theta: f32,
    pub crop_scale: (f32, f32),

    pub lines: Vec<Line<'a>>,
}

impl<'a> Display for Watermark<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        if self.mark_scale != Default::default() {
            write!(f, "scale {:?} ", self.mark_scale)?;
        }
        if self.mark_theta != Default::default() {
            write!(f, "rotate {:?} ", self.mark_theta)?;
        }
        if self.crop_scale != Default::default() {
            write!(f, "crop {:?} ", self.crop_scale)?;
        }
        Ok(())
    }
}

impl<'a> Watermark<'a> {
    pub fn scaled(mark_scale: f32) -> Self {
        Self::default().and_scaled(mark_scale)
    }
    pub fn and_scaled(self, mark_scale: f32) -> Self {
        Self { mark_scale, ..self }
    }

    pub fn cropped(h_scale: f32, v_scale: f32) -> Self {
        Self::default().and_cropped(h_scale, v_scale)
    }
    pub fn and_cropped(self, h_scale: f32, v_scale: f32) -> Self {
        Self {
            crop_scale: (h_scale, v_scale),
            ..self
        }
    }

    pub fn rotated(theta: f32) -> Self {
        Self::default().and_rotated(theta)
    }
    pub fn and_rotated(self, mark_theta: f32) -> Self {
        Self { mark_theta, ..self }
    }

    pub fn lines(lines: Vec<Line<'a>>) -> Self {
        Self::default().with_lines(lines)
    }
    pub fn with_lines(self, lines: Vec<Line<'a>>) -> Self {
        Self { lines, ..self }
    }
}
