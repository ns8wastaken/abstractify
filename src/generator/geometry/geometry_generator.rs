use image::RgbaImage;
use rand::Rng;

use crate::primitive::Shape;

pub trait GeometryGenerator {
    fn generate<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        target: &RgbaImage,
        canvas: &RgbaImage,
    ) -> Shape;
}
