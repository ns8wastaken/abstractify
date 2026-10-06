use image::RgbaImage;

use crate::primitive::Bounds;

pub trait Primitive {
    fn bounds(&self) -> Bounds;
    fn draw(&self, canvas: &mut RgbaImage);
}
