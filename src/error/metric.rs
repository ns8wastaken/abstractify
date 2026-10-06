use image::{Rgba, RgbaImage};

pub trait ErrorMetric {
    fn pixel(
        &self,
        target: Rgba<u8>,
        current: Rgba<u8>,
    ) -> u64;

    fn total(
        &self,
        target: &RgbaImage,
        canvas: &RgbaImage,
    ) -> u64;
}
