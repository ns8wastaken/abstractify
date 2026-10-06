use image::{Rgba, RgbaImage};
use rand::{Rng, RngExt};

use crate::primitive::{Circle, Shape, ShapeKind};

use super::ShapeGenerator;

pub struct RandomShapeGenerator {
    enabled: Vec<ShapeKind>,
    max_radius_fraction: f32,
    min_alpha: u8,
}

impl RandomShapeGenerator {
    pub fn new(
        enabled: Vec<ShapeKind>,
        max_radius_fraction: f32,
        min_alpha: u8,
    ) -> Self {
        Self {
            enabled,
            max_radius_fraction,
            min_alpha,
        }
    }

    fn generate_circle<R: Rng>(
        &self,
        target: &RgbaImage,
        rng: &mut R,
    ) -> Circle {
        let width = target.width() as i32;
        let height = target.height() as i32;

        let max_dimension = width.max(height);

        let max_radius = (max_dimension as f32 * self.max_radius_fraction)
            .round() as i32;

        let max_radius = max_radius.max(1);

        let radius = rng.random_range(1..=max_radius);

        let x = rng.random_range(-radius..width + radius);
        let y = rng.random_range(-radius..height + radius);

        Circle {
            x,
            y,
            radius: radius as u32,
            color: Rgba([
                rng.random(),
                rng.random(),
                rng.random(),
                rng.random_range(self.min_alpha..=255),
            ]),
        }
    }
}

impl ShapeGenerator for RandomShapeGenerator {
    fn generate(
        &mut self,
        target: &RgbaImage,
        _canvas: &RgbaImage,
    ) -> Shape {
        let mut rng = rand::rng();

        let kind = self.enabled[rng.random_range(0..self.enabled.len())];

        match kind {
            ShapeKind::Circle => {
                Shape::Circle(self.generate_circle(target, &mut rng))
            }
        }
    }
}
