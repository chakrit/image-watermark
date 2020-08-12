# IMAGE-WATERMARK

```rs
use image_watermark as watermark;

let lines = vec![
    watermark::Line::new(
        TEST_FONT,
        128.0,
        watermark::colors::from([255, 255, 255, 255]),
        "watermark text",
    )
    .unwrap(),
];

let watermark = watermark::Watermark::scaled(0.8)
    .and_cropped(0.5, 0.8)
    .and_rotated(-0.16) // 30 degrees counter-clockwise
    .with_lines(lines);

let out_buf = watermark::apply(TEST_INPUT_IMAGE.to_vec(), watermark).unwrap();
assert_eq!(out_buf.len(), TEST_OUTPUT_IMAGE.len());
```
