use image::{Rgba, RgbaImage};

pub struct Canvas;

impl Canvas {
    pub fn average(target: &RgbaImage) -> RgbaImage {
        let mut r = 0u64;
        let mut g = 0u64;
        let mut b = 0u64;

        let count =
            target.width() as u64 * target.height() as u64;

        for pixel in target.pixels() {
            r += pixel[0] as u64;
            g += pixel[1] as u64;
            b += pixel[2] as u64;
        }

        let background = Rgba([
            (r / count) as u8,
            (g / count) as u8,
            (b / count) as u8,
            255,
        ]);

        RgbaImage::from_pixel(
            target.width(),
            target.height(),
            background,
        )
    }

    pub fn transparent(target: &RgbaImage) -> RgbaImage {
        RgbaImage::new(
            target.width(),
            target.height(),
        )
    }

    pub fn solid(
        target: &RgbaImage,
        color: Rgba<u8>,
    ) -> RgbaImage {
        RgbaImage::from_pixel(
            target.width(),
            target.height(),
            color,
        )
    }
}
