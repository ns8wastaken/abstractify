use image::RgbaImage;

use crate::primitive::Shape;

pub trait ShapeGenerator {
    fn generate(
        &mut self,
        target: &RgbaImage,
        canvas: &RgbaImage,
    ) -> Shape;
}
