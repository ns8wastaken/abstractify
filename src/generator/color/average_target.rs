use image::{Rgba, RgbaImage};
use rand::Rng;

use crate::{
    generator::color::ColorGenerator,
    primitive::{Primitive, Shape},
};

pub struct AverageTargetColor {
    alpha: u8,
}

impl AverageTargetColor {
    pub fn new(alpha: u8) -> Self {
        Self { alpha }
    }
}

impl ColorGenerator for AverageTargetColor {
    fn generate<R: Rng + ?Sized>(
        &mut self,
        _rng: &mut R,
        shape: &Shape,
        target: &RgbaImage,
        _canvas: &RgbaImage,
    ) -> Rgba<u8> {
        let bounds = shape.bounds();

        let width = target.width() as i32;
        let height = target.height() as i32;

        let x_min = bounds.x_min.max(0);
        let y_min = bounds.y_min.max(0);
        let x_max = bounds.x_max.min(width - 1);
        let y_max = bounds.y_max.min(height - 1);

        if x_min > x_max || y_min > y_max {
            return Rgba([0, 0, 0, self.alpha]);
        }

        let mut r = 0u64;
        let mut g = 0u64;
        let mut b = 0u64;
        let mut count = 0u64;

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                if !shape.contains(x, y) {
                    continue;
                }

                let pixel =
                    target.get_pixel(
                        x as u32,
                        y as u32,
                    );

                r += pixel[0] as u64;
                g += pixel[1] as u64;
                b += pixel[2] as u64;

                count += 1;
            }
        }

        if count == 0 {
            return Rgba([0, 0, 0, self.alpha]);
        }

        Rgba([
            (r / count) as u8,
            (g / count) as u8,
            (b / count) as u8,
            self.alpha,
        ])
    }
}
