use image::{Rgba, RgbaImage};

use crate::primitive::Bounds;

pub trait Primitive {
    fn bounds(&self) -> Bounds;

    fn contains(
        &self,
        x: i32,
        y: i32,
    ) -> bool;

    fn color(&self) -> Rgba<u8>;

    fn set_color(
        &mut self,
        color: Rgba<u8>,
    );

    fn for_each_pixel<F>(
        &self,
        width: u32,
        height: u32,
        f: F,
    )
    where
        F: FnMut(u32, u32);

    fn draw(
        &self,
        canvas: &mut RgbaImage,
    );
}
