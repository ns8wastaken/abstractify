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

    fn draw(&self, canvas: &mut RgbaImage) {
        let bounds = self.bounds();

        let width = canvas.width() as i32;
        let height = canvas.height() as i32;

        let x_min = bounds.x_min.max(0);
        let y_min = bounds.y_min.max(0);
        let x_max = bounds.x_max.min(width - 1);
        let y_max = bounds.y_max.min(height - 1);

        let r2 = (self.radius as i32).pow(2);

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let dx = x - self.x;
                let dy = y - self.y;

                if dx * dx + dy * dy <= r2 {
                    let dst = *canvas.get_pixel(x as u32, y as u32);
                    let blended = Blender::alpha(dst, self.color);

                    canvas.put_pixel(x as u32, y as u32, blended);
                }
            }
        }
    }
}
