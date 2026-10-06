use image::{Rgba, RgbaImage};

use crate::error::{ErrorMetric, SquaredError};

pub struct Rmse;

impl ErrorMetric for Rmse {
    fn total(
        &self,
        target: &RgbaImage,
        canvas: &RgbaImage,
    ) -> u64 {
        (SquaredError.total(target, canvas) / (canvas.width() * canvas.height()) as u64)
            .isqrt()
    }

    fn pixel(
        &self,
        target: Rgba<u8>,
        current: Rgba<u8>,
    ) -> u64 {
        let dr = target[0] as i32 - current[0] as i32;
        let dg = target[1] as i32 - current[1] as i32;
        let db = target[2] as i32 - current[2] as i32;

        (((dr * dr + dg * dg + db * db) as u64) / 2).isqrt()
    }
}
