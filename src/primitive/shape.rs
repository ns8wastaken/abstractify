use image::{Rgba, RgbaImage};

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

    fn contains(
        &self,
        x: i32,
        y: i32,
    ) -> bool {
        match self {
            Shape::Circle(circle) => {
                circle.contains(x, y)
            }
        }
    }

    fn color(&self) -> Rgba<u8> {
        match self {
            Shape::Circle(circle) => circle.color(),
        }
    }

    fn set_color(
        &mut self,
        color: Rgba<u8>,
    ) {
        match self {
            Shape::Circle(circle) => {
                circle.set_color(color);
            }
        }
    }

    fn draw(
        &self,
        canvas: &mut RgbaImage,
    ) {
        match self {
            Shape::Circle(circle) => {
                circle.draw(canvas);
            }
        }
    }
}
