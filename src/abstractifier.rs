use image::RgbaImage;

use crate::error::ErrorMetric;
use crate::shape_generator::ShapeGenerator;
use crate::primitive::{Primitive, Shape};

#[derive(Debug, Clone)]
pub struct StepResult {
    pub error: u64,
    pub improvement: Option<u64>,
    pub candidates_tested: usize,
}

pub struct Abstractifier<G, E> {
    target: RgbaImage,
    canvas: RgbaImage,

    shapes_used: Vec<Shape>,

    generator: G,
    error_metric: E,

    error: u64,
    candidates_per_step: usize,
}

impl<G, E> Abstractifier<G, E>
where
    G: ShapeGenerator,
    E: ErrorMetric,
{
    pub fn new(
        target: RgbaImage,
        canvas: RgbaImage,
        generator: G,
        error_metric: E,
        candidates_per_step: usize,
    ) -> Self {
        let error = error_metric.total(&target, &canvas);

        Self {
            target,
            canvas,
            shapes_used: Vec::new(),
            generator,
            error_metric,
            error,
            candidates_per_step,
        }
    }

    pub fn step(&mut self) -> StepResult {
        let previous_error = self.error;

        let mut best_shape = None;
        let mut best_error = self.error;

        for _ in 0..self.candidates_per_step {
            let shape = self.generator.generate(
                &self.target,
                &self.canvas,
            );

            let mut candidate = self.canvas.clone();
            shape.draw(&mut candidate);

            let error = self.error_metric.total(
                &self.target,
                &candidate,
            );

            if error < best_error {
                best_error = error;
                best_shape = Some(shape);
            }
        }

        let Some(shape) = best_shape else {
            return StepResult {
                error: self.error,
                improvement: None,
                candidates_tested: self.candidates_per_step,
            };
        };

        shape.draw(&mut self.canvas);
        self.shapes_used.push(shape);

        self.error = best_error;

        StepResult {
            error: self.error,
            improvement: Some(previous_error - self.error),
            candidates_tested: self.candidates_per_step,
        }
    }

    pub fn canvas(&self) -> &RgbaImage {
        &self.canvas
    }
}
