use image::{Rgba, RgbaImage};
use rand::{Rng, RngExt};

use crate::{
    generator::color::ColorGenerator,
    primitive::Shape,
};

pub struct RandomColor {
    min_alpha: u8,
    max_alpha: u8,
}

impl RandomColor {
    pub fn new(
        min_alpha: u8,
        max_alpha: u8,
    ) -> Self {
        assert!(
            min_alpha <= max_alpha,
            "min_alpha must not exceed max_alpha"
        );

        Self {
            min_alpha,
            max_alpha,
        }
    }
}

impl ColorGenerator for RandomColor {
    fn generate<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        _shape: &Shape,
        _target: &RgbaImage,
        _canvas: &RgbaImage,
    ) -> Rgba<u8> {
        Rgba([
            rng.random(),
            rng.random(),
            rng.random(),
            rng.random_range(
                self.min_alpha..=self.max_alpha,
            ),
        ])
    }
}
