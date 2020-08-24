use crate::{
    colors::Color,
    result::{Error, Result},
};
use rusttype::{point, Font, Scale};
use std::{cmp::Ordering, sync::Arc};
use unicode_normalization::UnicodeNormalization;

#[derive(Debug)]
pub struct Line<'t> {
    font: Arc<Font<'t>>,
    scale: Scale,
    color: Color,
    text: &'t str,

    layout_w: f32,
    layout_h: f32,
}

impl<'t> Line<'t> {
    pub fn new(font_bytes: &'t [u8], scale: f32, color: Color, text: &'t str) -> Result<Self> {
        let font = Font::try_from_bytes(font_bytes)
            .ok_or_else(|| Error::LoadError("font load failure".to_string()))?;

        let scale = Scale::uniform(scale);
        let (w, h) = Self::layout(&font, scale, text);

        Ok(Line {
            font: Arc::new(font),
            scale,
            color,
            text,
            layout_w: w,
            layout_h: h,
        })
    }
    pub fn new_raw(font: Arc<Font<'t>>, scale: Scale, color: Color, text: &'t str) -> Self {
        let (w, h) = Self::layout(font.as_ref(), scale, text);

        Line {
            font: font.clone(),
            scale,
            color,
            text,
            layout_w: w,
            layout_h: h,
        }
    }

    pub fn font(&self) -> Arc<Font<'t>> {
        self.font.clone()
    }
    pub fn scale(&self) -> Scale {
        self.scale
    }
    pub fn color(&self) -> Color {
        self.color
    }
    pub fn text(&self) -> &str {
        self.text
    }
    pub fn width(&self) -> f32 {
        self.layout_w
    }
    pub fn height(&self) -> f32 {
        self.layout_h
    }

    pub fn ascent(&self) -> f32 {
        self.font.v_metrics(self.scale).ascent
    }
    pub fn descent(&self) -> f32 {
        self.font.v_metrics(self.scale).descent
    }

    fn layout<'c>(font: &Font, scale: Scale, text: &'c str) -> (f32, f32) {
        let text = text.nfc().collect::<String>();
        let (mut w, mut h): (f32, f32) = (0.0, 0.0);

        let v_metrics = font.v_metrics(scale);
        let layout = font.layout(&text, scale, point(0.0, v_metrics.ascent));
        for g in layout {
            let bb = g.pixel_bounding_box();
            if bb.is_none() {
                continue;
            }

            let bb = bb.unwrap();
            let (max_x, max_y) = (bb.max.x as f32, bb.max.y as f32);
            w = if Some(Ordering::Greater) == w.partial_cmp(&max_x) {
                w
            } else {
                max_x
            };
            h = if Some(Ordering::Greater) == h.partial_cmp(&max_y) {
                h
            } else {
                max_y
            };
        }

        (w, h + v_metrics.descent)
    }
}
