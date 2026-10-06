use image::{Rgba, RgbaImage};
use rand::Rng;

use crate::primitive::Shape;

pub trait ColorGenerator {
    fn generate<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        shape: &Shape,
        target: &RgbaImage,
        canvas: &RgbaImage,
    ) -> Rgba<u8>;
}
