use image::RgbaImage;
use rand::Rng;

use crate::generator::color::ColorGenerator;
use crate::generator::geometry::GeometryGenerator;
use crate::generator::ShapeGenerator;
use crate::primitive::Primitive;

pub struct CandidateGenerator<G, C> {
    geometry: G,
    color: C,
}

impl<G, C> CandidateGenerator<G, C> {
    pub fn new(geometry: G, color: C) -> Self {
        Self {
            geometry,
            color,
        }
    }
}

impl<G, C> ShapeGenerator for CandidateGenerator<G, C>
where
    G: GeometryGenerator,
    C: ColorGenerator,
{
    fn generate<R: Rng + ?Sized>(
        &mut self,
        rng: &mut R,
        target: &RgbaImage,
        canvas: &RgbaImage,
    ) -> crate::primitive::Shape {
        let mut shape = self.geometry.generate(
            rng,
            target,
            canvas,
        );

        let color = self.color.generate(
            rng,
            &shape,
            target,
            canvas,
        );

        shape.set_color(color);

        shape
    }
}
