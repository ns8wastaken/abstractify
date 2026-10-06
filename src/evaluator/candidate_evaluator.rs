use image::RgbaImage;

use crate::{
    error::ErrorMetric,
    primitive::Shape,
};

pub trait CandidateEvaluator {
    fn evaluate_batch<E: ErrorMetric + Sync>(
        &self,
        candidates: &[Shape],
        target: &RgbaImage,
        canvas: &RgbaImage,
        error_metric: &E,
    ) -> Vec<i64>;
}
