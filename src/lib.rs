extern crate image;
extern crate imageproc;
extern crate rusttype;

mod futils;
mod lines;
mod op;
mod result;

#[allow(unused_imports)/* required for .dimensions() on DynamicImage */]
use image::GenericImageView;
use std::iter::IntoIterator;

pub mod colors;

pub use crate::{colors::Color, lines::Line, op::Op, result::Error, result::Result};

pub fn apply<'a>(
    img_data: Vec<u8>,
    pipelines: impl IntoIterator<Item = Op<'a>>,
) -> Result<Vec<u8>> {
    let mut img: image::DynamicImage = image::load_from_memory(&img_data)?;
    for op in pipelines {
        img = op.apply(img);
    }

    let mut out_buf: Vec<u8> = Vec::new();
    img.write_to(&mut out_buf, image::ImageFormat::Png)?;
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

        let ops = vec![
            Op::Scale(0.8),
            Op::Crop(0.5, 0.8),
            Op::Watermark(0.8, lines),
        ];

        let out_buf = apply(TEST_INPUT_IMAGE.to_vec(), ops).unwrap();
        // let out_path = std::path::Path::new("./output.png");
        // std::fs::write(&out_path, &out_buf).unwrap();
        assert_eq!(out_buf.len(), TEST_OUTPUT_IMAGE.len());
    }
}
