use image::RgbaImage;

use crate::primitive::Bounds;

pub trait ErrorMetric {
    fn total(
        &self,
        target: &RgbaImage,
        canvas: &RgbaImage,
    ) -> u64;

    fn region(
        &self,
        target: &RgbaImage,
        canvas: &RgbaImage,
        bounds: Bounds,
    ) -> u64;
}
