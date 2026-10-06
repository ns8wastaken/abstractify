use image::{Rgba, RgbaImage};

use crate::error::ErrorMetric;

pub struct SquaredError;

impl ErrorMetric for SquaredError {
    #[inline]
    fn pixel(
        &self,
        target: Rgba<u8>,
        current: Rgba<u8>,
    ) -> u64 {
        let dr = target[0] as i32 - current[0] as i32;
        let dg = target[1] as i32 - current[1] as i32;
        let db = target[2] as i32 - current[2] as i32;

        (dr * dr + dg * dg + db * db) as u64
    }

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
}
