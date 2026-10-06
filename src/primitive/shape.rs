use image::RgbaImage;

use super::{Bounds, Circle, Primitive};

#[derive(Clone, Copy)]
pub enum ShapeKind {
    Circle,
    // TODO: Rectangle,
    // TODO: Triangle,
}

#[derive(Clone, Copy)]
pub enum Shape {
    Circle(Circle),
    // TODO: Rectangle(Rectangle),
    // TODO: Triangle(Triangle),
}

impl Primitive for Shape {
    fn bounds(&self) -> Bounds {
        match self {
            Shape::Circle(circle) => circle.bounds(),
        }
    }

    fn draw(&self, canvas: &mut RgbaImage) {
        match self {
            Shape::Circle(circle) => circle.draw(canvas),
        }
    }
}
