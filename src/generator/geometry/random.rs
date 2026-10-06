use image::{Rgba, RgbaImage};
use rand::{Rng, RngExt};

use crate::{
    generator::geometry::GeometryGenerator,
    primitive::{
        Circle,
        Shape,
        ShapeKind,
    },
};

pub struct RandomGeometry {
    enabled_shapes: Vec<ShapeKind>,
    max_radius_fraction: f32,
}

impl RandomGeometry {
    pub fn new(
        enabled_shapes: Vec<ShapeKind>,
        max_radius_fraction: f32,
    ) -> Self {
        assert!(
            !enabled_shapes.is_empty(),
            "At least one shape type must be enabled"
        );

        Self {
            enabled_shapes,
            max_radius_fraction,
        }
    }

    fn generate_circle<R: Rng + ?Sized>(
        &self,
        rng: &mut R,
        target: &RgbaImage,
    ) -> Shape {
        let width = target.width() as i32;
        let height = target.height() as i32;

        let max_radius =
            (width.max(height) as f32
                * self.max_radius_fraction)
                .round() as i32;

        let max_radius = max_radius.max(1);

        let radius =
            rng.random_range(1..=max_radius);

        let x =
            rng.random_range(-radius..width + radius);

        let y =
            rng.random_range(-radius..height + radius);

        Shape::Circle(Circle {
            x,
            y,
            radius: radius as u32,
            color: Rgba([0, 0, 0, 0]),
        })
    }
}

impl GeometryGenerator for RandomGeometry {
    fn generate<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        target: &RgbaImage,
        _canvas: &RgbaImage,
    ) -> Shape {
        let index = rng.random_range(0..self.enabled_shapes.len());

        match self.enabled_shapes[index] {
            ShapeKind::Circle => {
                self.generate_circle(rng, target)
            }
        }
    }
}
