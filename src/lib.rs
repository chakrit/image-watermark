extern crate image;
extern crate imageproc;
extern crate rusttype;

use image::imageops::{overlay, resize, FilterType};
use image::{GenericImageView, ImageFormat, RgbaImage};
use imageproc::drawing::draw_text_mut;
use imageproc::geometric_transformations::{rotate_about_center, Interpolation};
use std::cmp::{max, min};
use std::iter::Iterator;

mod colors;
mod lines;
mod watermark;

pub use crate::colors::Color;
pub use crate::lines::Line;
pub use crate::result::Error;
pub use crate::result::Result;
pub use crate::watermark::Watermark;

pub mod result;

const PAD_FOR_ROTATE: u32 = 10;

pub fn apply(base_img_buf: Vec<u8>, watermark: Watermark) -> Result<Vec<u8>> {
    let lines = watermark.lines;

    // adds enough padding so that rotating the image wouldn't result in corners of the text being
    // clipped by the border.
    let top_ascent_h = lines.first().map(Line::ascent).unwrap_or_default();
    let bottom_descent_h = lines.last().map(Line::descent).unwrap_or_default();
    let padding = max(top_ascent_h, bottom_descent_h) + PAD_FOR_ROTATE;

    let (mark_bb_w, mark_bb_h) = (
        lines.iter().map(Line::width).max().unwrap_or_default() + 2 * padding,
        lines.iter().map(Line::height).sum::<u32>() + 2 * padding,
    );
    let mark_rect_size = max(mark_bb_w, mark_bb_h);

    // draw the transparent watermark image in a buffer first (so we can rotate and scale without
    // impacting the target image)
    let mut mark_img = RgbaImage::new(mark_rect_size, mark_rect_size);
    let mut origin = rusttype::point(padding, (mark_rect_size - mark_bb_h) >> 1);
    for line in lines.iter() {
        origin.x = padding + ((mark_bb_w - line.width()) >> 1); // center text horizontally
        draw_text_mut(
            &mut mark_img,
            line.color(),
            origin.x,
            origin.y,
            line.scale(),
            line.font().as_ref(),
            line.text(),
        );
        origin.y += line.height(); // move down 1 line
    }

    // rotate the mark
    if watermark.mark_scale != Default::default() {
        mark_img = rotate_about_center(
            &mark_img,
            watermark.mark_theta,
            Interpolation::Bicubic,
            colors::clear(),
        );
    }

    // load the base image
    let mut base_img = image::load_from_memory(&base_img_buf)?;
    let (mut base_w, mut base_h) = base_img.dimensions();

    // crops
    if watermark.crop_scale != Default::default() {
        let (crop_w, crop_h) = (
            (base_w as f32 * watermark.crop_scale.0).round() as u32,
            (base_h as f32 * watermark.crop_scale.1).round() as u32,
        );
        let (crop_x, crop_y) = ((base_w - crop_w) >> 1, (base_h - crop_h) >> 1);
        base_img = base_img.crop(crop_x, crop_y, crop_w, crop_h);
        base_w = crop_w;
        base_h = crop_h;
    }

    // resize mask to fit the (cropped-)image based on mark_scale
    let scaled_mark_size = (min(base_w, base_h) as f32 * watermark.mark_scale).round() as u32;
    let mut scaled_mark_img = resize(
        &mut mark_img,
        scaled_mark_size,
        scaled_mark_size,
        FilterType::Gaussian,
    );

    let (mark_x, mark_y) = (
        (base_w - scaled_mark_size) >> 1,
        (base_h - scaled_mark_size) >> 1,
    );
    overlay(&mut base_img, &mut scaled_mark_img, mark_x, mark_y);

    let mut out_buf: Vec<u8> = Vec::new();
    base_img.write_to(&mut out_buf, ImageFormat::Png)?;
    Ok(out_buf)
}

#[cfg(test)]
mod tests {
    use super::*;

    static TEST_FONT: &[u8] = include_bytes!("../testdata/fonts/Athiti-Bold.ttf");
    static TEST_INPUT_IMAGE: &[u8] =
        include_bytes!("../testdata/images/lenin-estrada-OI1ToozsKBw-unsplash.jpg");
    static TEST_WATERMARK_COLOR: [u8; 4] = [255, 255, 255, 255];
    static TEST_OUTPUT_IMAGE: &[u8] =
        include_bytes!("../testdata/images/lenin-estrada-OI1ToozsKBw-unsplash.watermarked.png");

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_applies_correctly() {
        let lines = vec![
            Line::new(
                TEST_FONT,
                128.0,
                colors::from(TEST_WATERMARK_COLOR),
                "ทดสอบภาษาไทย English",
            )
            .unwrap(),
            Line::new(
                TEST_FONT,
                128.0,
                colors::from(TEST_WATERMARK_COLOR),
                "jumps over the lazy dog, เดอะควิกบราวน์ฟอกซ์",
            )
            .unwrap(),
            Line::new(
                TEST_FONT,
                128.0,
                colors::from(TEST_WATERMARK_COLOR),
                "01 January 2020",
            )
            .unwrap(),
        ];

        let watermark = Watermark::scaled(0.8)
            .and_cropped(0.5, 0.8)
            .and_rotated(-0.16) // 30 degrees counter-clockwise
            .with_lines(lines);

        let out_buf = apply(TEST_INPUT_IMAGE.to_vec(), watermark).unwrap();
        // let out_path = std::path::Path::new("./output.png");
        // std::fs::write(&out_path, &out_buf).unwrap();
        assert_eq!(out_buf.len(), TEST_OUTPUT_IMAGE.len());
    }
}
