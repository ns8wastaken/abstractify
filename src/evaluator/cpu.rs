use image::RgbaImage;
use rayon::{ThreadPool, ThreadPoolBuilder, prelude::*};

use crate::blend::Blender;
use crate::error::ErrorMetric;
use crate::evaluator::CandidateEvaluator;
use crate::primitive::{Primitive, Shape};

#[derive(Debug)]
pub enum CpuMode {
    Sequential,
    Parallel(usize),
}

#[derive(Debug)]
pub struct CpuEvaluator {
    mode: CpuMode,
    thread_pool: Option<ThreadPool>,
}

impl CpuEvaluator {
    pub fn new(mode: CpuMode) -> Self {
        let resolved_mode = match mode {
            CpuMode::Sequential => CpuMode::Sequential,

            CpuMode::Parallel(jobs) => {
                let jobs = if jobs == 0 {
                    std::thread::available_parallelism()
                        .map(|n| n.get())
                        .unwrap_or(1)
                } else {
                    jobs
                };

                if jobs <= 1 {
                    CpuMode::Sequential
                } else {
                    CpuMode::Parallel(jobs)
                }
            }
        };

        let thread_pool = match resolved_mode {
            CpuMode::Sequential => None,

            CpuMode::Parallel(jobs) => Some(
                ThreadPoolBuilder::new()
                    .num_threads(jobs)
                    .build()
                    .expect("failed to create CPU evaluator thread pool")
            ),
        };

        Self { mode, thread_pool }
    }

    fn evaluate<E: ErrorMetric>(
        &self,
        shape: &Shape,
        target: &RgbaImage,
        canvas: &RgbaImage,
        error_metric: &E,
    ) -> i64 {
        let mut delta = 0i64;

        shape.for_each_pixel(
            canvas.width(),
            canvas.height(),
            |x, y| {
                let target_pixel =
                    *target.get_pixel(x, y);

                let current_pixel =
                    *canvas.get_pixel(x, y);

                let candidate_pixel =
                    Blender::alpha(
                        current_pixel,
                        shape.color(),
                    );

                let current_error =
                    error_metric.pixel(
                        target_pixel,
                        current_pixel,
                    );

                let candidate_error =
                    error_metric.pixel(
                        target_pixel,
                        candidate_pixel,
                    );

                delta += candidate_error as i64
                    - current_error as i64;
            },
        );

        delta
    }

    fn evaluate_sequential<E: ErrorMetric>(
        &self,
        candidates: &[Shape],
        target: &RgbaImage,
        canvas: &RgbaImage,
        error_metric: &E,
    ) -> Vec<i64> {
        candidates
            .iter()
            .map(|shape| {
                self.evaluate(
                    shape,
                    target,
                    canvas,
                    error_metric,
                )
            })
            .collect()
    }

    fn evaluate_parallel<E>(
        &self,
        candidates: &[Shape],
        target: &RgbaImage,
        canvas: &RgbaImage,
        error_metric: &E,
    ) -> Vec<i64>
    where
        E: ErrorMetric + Sync,
    {
        self.thread_pool
            .as_ref()
            .expect("parallel evaluator has no thread pool")
            .install(|| {
                candidates
                    .par_iter()
                    .map(|shape| {
                        self.evaluate(
                            shape,
                            target,
                            canvas,
                            error_metric,
                        )
                    })
                    .collect()
            })
    }
}

impl CandidateEvaluator for CpuEvaluator {
    fn evaluate_batch<E>(
        &self,
        candidates: &[Shape],
        target: &RgbaImage,
        canvas: &RgbaImage,
        error_metric: &E,
    ) -> Vec<i64>
    where
        E: ErrorMetric + Sync,
    {
        match self.mode {
            CpuMode::Sequential => {
                self.evaluate_sequential(
                    candidates,
                    target,
                    canvas,
                    error_metric,
                )
            }

            CpuMode::Parallel(_) => {
                self.evaluate_parallel(
                    candidates,
                    target,
                    canvas,
                    error_metric,
                )
            }
        }
    }
}
