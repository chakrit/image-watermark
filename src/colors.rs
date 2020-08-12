use image::Rgba;

pub type Color = Rgba<u8>;

pub fn from(v: [u8; 4]) -> Color {
    Rgba::<u8>(v)
}

pub fn clear() -> Color {
    Rgba::from([0, 0, 0, 0])
}

pub fn black() -> Color {
    Rgba::from([0, 0, 0, 255])
}

pub fn white() -> Color {
    Rgba::from([255, 255, 255, 255])
}
