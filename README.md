# IMAGE-WATERMARK

```rs
use image_watermark::*;

let lines = vec![
    Line::new(
        TEST_FONT,
        128.0,
        colors::from(TEST_WATERMARK_COLOR),
        "the quick brown fox jumps over the lazy dog",
    )
    .unwrap(),
];

let ops = vec![
    Op::Scale(0.8),
    Op::Crop(0.5, 0.8),
    Op::Watermark(0.8, lines),
];

let out_buf = apply(TEST_INPUT_IMAGE.to_vec(), ops).unwrap();
let out_path = std::path::Path::new("./output.png");
std::fs::write(&out_path, &out_buf).unwrap();
```
