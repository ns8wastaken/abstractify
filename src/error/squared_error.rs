use image::RgbaImage;

use crate::{error::ErrorMetric, primitive::Bounds};

pub struct SquaredError;

impl ErrorMetric for SquaredError {
    fn total(
        &self,
        target: &RgbaImage,
        canvas: &RgbaImage,
    ) -> u64 {
        target
            .pixels()
            .zip(canvas.pixels())
            .map(|(target, current)| {
                let dr = target[0] as i32 - current[0] as i32;
                let dg = target[1] as i32 - current[1] as i32;
                let db = target[2] as i32 - current[2] as i32;

                (dr * dr + dg * dg + db * db) as u64
            })
            .sum()
    }

    fn region(
        &self,
        target: &RgbaImage,
        canvas: &RgbaImage,
        bounds: Bounds,
    ) -> u64 {
        let width = target.width() as i32;
        let height = target.height() as i32;

        let x_min = bounds.x_min.max(0);
        let y_min = bounds.y_min.max(0);
        let x_max = bounds.x_max.min(width - 1);
        let y_max = bounds.y_max.min(height - 1);

        if x_min > x_max || y_min > y_max {
            return 0;
        }

        let mut error = 0;

        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let target = target.get_pixel(
                    x as u32,
                    y as u32,
                );

                let canvas = canvas.get_pixel(
                    x as u32,
                    y as u32,
                );

                for channel in 0..3 {
                    let difference = target[channel] as i32
                        - canvas[channel] as i32;

                    error += (difference * difference) as u64;
                }
            }
        }

        error
    }
}
