use image::RgbaImage;

use crate::{error::{ErrorMetric, SquaredError}, primitive::Bounds};

pub struct Rmse;

impl ErrorMetric for Rmse {
    fn total(
        &self,
        target: &RgbaImage,
        canvas: &RgbaImage,
    ) -> u64 {
        let squared = SquaredError;
        (squared.total(target, canvas) / (canvas.width()* canvas.height()) as u64)
            .isqrt()
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

        error / ((x_max - x_min) * (y_max - y_min)) as u64
    }
}
