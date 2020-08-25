use crate::{colors, futils::*, lines::Line};
use image::{
    imageops::{overlay, FilterType},
    DynamicImage, GenericImageView, Rgba, RgbaImage,
};
use imageproc::{
    drawing::draw_text_mut,
    geometric_transformations::{rotate_about_center, Interpolation},
};
use std::{cmp::min, iter::Iterator};

type Lines<'a> = Vec<Line<'a>>;

pub enum Op<'a> {
    Scale(f32),
    ScaleExact(u32, u32),
    Rotate(f32),
    Crop(f32, f32),
    CropExact(u32, u32),
    PaperPaste(u32, u32, u32, u32),
    Watermark(f32, Lines<'a>),
}

impl<'a> Op<'a> {
    pub fn apply(self, buf: DynamicImage) -> DynamicImage {
        match self {
            Self::Scale(s) => scale(buf, s),
            Self::ScaleExact(w, h) => scale_exact(buf, w, h),
            Self::Rotate(theta) => rotate(buf, theta),
            Self::Crop(ws, hs) => crop(buf, ws, hs),
            Self::CropExact(nw, nh) => crop_exact(buf, nw, nh),
            Self::PaperPaste(pw, ph, x, y) => paper_paste(buf, pw, ph, x, y),
            Self::Watermark(scale, lines) => watermark(buf, scale, lines),
        }
    }
}

fn scale(buf: DynamicImage, scale: f32) -> DynamicImage {
    let (w, h) = buf.dimensions();
    let (nw, nh) = ((w as f32 * scale) as u32, (h as f32 * scale) as u32);
    buf.resize(nw, nh, FilterType::Gaussian)
}

fn scale_exact(buf: DynamicImage, w: u32, h: u32) -> DynamicImage {
    buf.resize_exact(w, h, FilterType::Gaussian)
}

fn rotate(buf: DynamicImage, theta: f32) -> DynamicImage {
    let rgba = buf.to_rgba();
    let result = rotate_about_center(&rgba, theta, Interpolation::Bicubic, Rgba([0, 0, 0, 255]));
    DynamicImage::ImageRgba8(result)
}

fn crop(buf: DynamicImage, ws: f32, hs: f32) -> DynamicImage {
    let (w, h) = buf.dimensions();
    let (nw, nh) = ((w as f32 * ws) as u32, (h as f32 * hs) as u32);
    let (x, y) = ((w - nw) >> 1, (h - nh) >> 1);
    buf.crop_imm(x, y, nw, nh)
}

fn crop_exact(buf: DynamicImage, nw: u32, nh: u32) -> DynamicImage {
    let (w, h) = buf.dimensions();
    let (x, y) = ((w - nw) >> 1, (h - nh) >> 1);
    buf.crop_imm(x, y, nw, nh)
}

fn paper_paste(buf: DynamicImage, pw: u32, ph: u32, x: u32, y: u32) -> DynamicImage {
    let mut paper = RgbaImage::new(pw, ph);
    overlay(&mut paper, &buf, x, y);
    DynamicImage::ImageRgba8(paper)
}

fn watermark<'a>(buf: DynamicImage, mark_scale: f32, lines: Lines<'a>) -> DynamicImage {
    const PAD_FOR_ROTATE: f32 = 10.0;
    const WATERMARK_THETA: f32 = -0.16;

    // TODO: This assumes that width of the rendered text are not longer than its height.
    let top_ascent_h = lines.first().map(Line::ascent).unwrap_or_default();
    let bottom_descent_h = lines.last().map(Line::descent).unwrap_or_default();
    let padding = max_f32(top_ascent_h, bottom_descent_h) + PAD_FOR_ROTATE;

    let (mark_bb_w, mark_bb_h) = (
        lines.iter().map(Line::width).max_f32().unwrap_or_default() + 2.0 * padding,
        lines.iter().map(Line::height).sum_f32() + 2.0 * padding,
    );
    let mark_rect_size = max_f32(mark_bb_w, mark_bb_h);

    // draw the transparent watermark image in a buffer first (so we can rotate and scale without
    // impacting the target image)
    let mut mark_img: RgbaImage = RgbaImage::new(mark_rect_size as u32, mark_rect_size as u32);
    let mut origin = rusttype::point(padding, (mark_rect_size - mark_bb_h) * 0.5);
    for line in lines.iter() {
        origin.x = padding + ((mark_bb_w - line.width()) * 0.5); // center text horizontally
        draw_text_mut(
            &mut mark_img,
            line.color(),
            origin.x.round() as u32,
            origin.y.round() as u32,
            line.scale(),
            line.font().as_ref(),
            line.text(),
        );
        origin.y += line.height(); // move down 1 line
    }

    // rotate the mark
    mark_img = rotate_about_center(
        &mark_img,
        WATERMARK_THETA,
        Interpolation::Bicubic,
        colors::clear(),
    );

    // resize mask to fit the image
    let (base_w, base_h) = buf.dimensions();
    let scaled_mark_size = (min(base_w, base_h) as f32 * mark_scale).round() as u32;
    let mut scaled_mark_img = image::imageops::resize(
        &mut mark_img,
        scaled_mark_size,
        scaled_mark_size,
        FilterType::Gaussian,
    );
    let (mark_x, mark_y) = (
        (base_w - scaled_mark_size) >> 1,
        (base_h - scaled_mark_size) >> 1,
    );

    // actually draw the watermark
    let mut base_img = buf.to_rgba();
    overlay(&mut base_img, &mut scaled_mark_img, mark_x, mark_y);
    DynamicImage::ImageRgba8(base_img)
}
