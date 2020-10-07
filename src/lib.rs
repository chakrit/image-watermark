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
    static TEST_ID_IMAGE: &[u8] = include_bytes!("../testdata/images/id_card.jpg");
    static TEST_SIG_IMAGE: &[u8] = include_bytes!("../testdata/images/signature.png");
    static TEST_WATERMARK_COLOR: [u8; 4] = [255, 0, 0, 255];
    static TEST_WATERMARK_LINE_SPACING: f32 = 0.2;
    static TEST_OUTPUT_IMAGE: &[u8] = include_bytes!("../testdata/images/output.png");

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    fn it_applies_correctly() {
        use std::io::Write;

        let sig_img = image::load_from_memory(TEST_SIG_IMAGE).unwrap();
        let mark_lines = vec![
            Line::new(
                TEST_FONT,
                128.0,
                colors::from(TEST_WATERMARK_COLOR),
                "ทดสอบภาษาไทย English",
            )
            .unwrap()
            .with_spacing(TEST_WATERMARK_LINE_SPACING),
            Line::new(
                TEST_FONT,
                128.0,
                colors::from(TEST_WATERMARK_COLOR),
                "jumps over the lazy dog, เดอะควิกบราวน์ฟอกซ์",
            )
            .unwrap()
            .with_spacing(TEST_WATERMARK_LINE_SPACING),
            Line::new(
                TEST_FONT,
                128.0,
                colors::from(TEST_WATERMARK_COLOR),
                "01 January 2020",
            )
            .unwrap()
            .with_spacing(TEST_WATERMARK_LINE_SPACING),
        ];

        let (id_w, id_h) = (800, 600);
        let (paper_w, paper_h) = (2100, 2970);
        let (paper_x, paper_y) = ((paper_w - id_w) >> 1, (paper_h - id_h) >> 2);
        let (sig_w, sig_h) = sig_img.dimensions();
        let (sig_x, sig_y) = ((paper_w - sig_w) >> 1, (paper_h - sig_h) >> 1);

        let ops = vec![
            Op::Crop(0.9, 0.9),
            Op::ScaleExact(id_w, id_h),
            Op::Watermark(0.9, mark_lines),
            Op::PaperPaste(paper_w, paper_h, paper_x, paper_y),
            Op::Stamp(sig_img, sig_x, sig_y),
        ];

        let out_buf = apply(TEST_ID_IMAGE.to_vec(), ops).unwrap();
        if let Some(out_path) = std::env::var("WATERMARK_OUTPUT").ok() {
            let mut out_file = std::fs::File::create(out_path).unwrap();
            out_file.write_all(&out_buf).unwrap();
        }

        // TODO: Do a more precise pixel comparison
        assert_eq!(out_buf.len(), TEST_OUTPUT_IMAGE.len());
    }
}
