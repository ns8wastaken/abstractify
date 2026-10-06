use image::{Rgba, RgbaImage};

use crate::blend::Blender;
use crate::primitive::{Bounds, Primitive};

#[derive(Clone, Copy)]
pub struct Circle {
    pub x: i32,
    pub y: i32,
    pub radius: u32,
    pub color: Rgba<u8>,
}

impl Primitive for Circle {
    fn bounds(&self) -> Bounds {
        let r = self.radius as i32;

        Bounds {
            x_min: self.x - r,
            y_min: self.y - r,
            x_max: self.x + r,
            y_max: self.y + r,
        }
    }

    fn contains(
        &self,
        x: i32,
        y: i32,
    ) -> bool {
        let dx = x - self.x;
        let dy = y - self.y;

        let radius = self.radius as i32;

        dx * dx + dy * dy
            <= radius * radius
    }

    fn color(&self) -> Rgba<u8> {
        self.color
    }

    fn set_color(
        &mut self,
        color: Rgba<u8>,
    ) {
        self.color = color;
    }

    fn for_each_pixel<F>(
        &self,
        width: u32,
        height: u32,
        mut f: F,
    )
    where
        F: FnMut(u32, u32),
    {
        let bounds = self.bounds();

        let x_min = bounds.x_min.max(0);
        let y_min = bounds.y_min.max(0);

        let x_max =
            bounds.x_max.min(width as i32 - 1);

        let y_max =
            bounds.y_max.min(height as i32 - 1);

        if x_min > x_max || y_min > y_max {
            return;
        }

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if self.contains(x, y) {
                    f(x as u32, y as u32);
                }
            }
        }
    }

    fn draw(&self, canvas: &mut RgbaImage) {
        self.for_each_pixel(
            canvas.width(),
            canvas.height(),
            |x, y| {
                let dst = *canvas.get_pixel(x, y);

                let blended = Blender::alpha(
                    dst,
                    self.color,
                );

                canvas.put_pixel(
                    x,
                    y,
                    blended,
                );
            },
        );
    }
}
