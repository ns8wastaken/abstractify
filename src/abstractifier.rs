use image::RgbaImage;
use rand::SeedableRng;
use rand::rngs::StdRng;

use crate::error::ErrorMetric;
use crate::evaluator::CandidateEvaluator;
use crate::generator::ShapeGenerator;
use crate::primitive::{Primitive, Shape};

#[derive(Debug, Clone)]
pub struct StepResult {
    pub error: u64,
    pub improvement: Option<u64>,
}

pub struct Abstractifier<G, E, V> {
    target: RgbaImage,
    canvas: RgbaImage,

    generator: G,
    error_metric: E,
    evaluator: V,

    rng: StdRng,

    error: u64,
    candidates_per_step: usize,
    shapes_used: Vec<Shape>,
}

impl<G, E, V> Abstractifier<G, E, V>
where
    G: ShapeGenerator,
    E: ErrorMetric + Sync,
    V: CandidateEvaluator,
{
    pub fn new(
        target: RgbaImage,
        canvas: RgbaImage,
        generator: G,
        error_metric: E,
        evaluator: V,
        candidates_per_step: usize,
        seed: u64,
    ) -> Self {
        let error = error_metric.total(&target, &canvas);

        Self {
            target,
            canvas,
            shapes_used: Vec::new(),
            generator,
            error_metric,
            evaluator,
            rng: StdRng::seed_from_u64(seed),
            error,
            candidates_per_step,
        }
    }

    pub fn step(&mut self) -> StepResult {
        let mut candidates =
            Vec::with_capacity(
                self.candidates_per_step,
            );

        for _ in 0..self.candidates_per_step {
            let shape = self.generator.generate(
                &mut self.rng,
                &self.target,
                &self.canvas,
            );

            candidates.push(shape);
        }

        let deltas = self.evaluator.evaluate_batch(
            &candidates,
            &self.target,
            &self.canvas,
            &self.error_metric,
        );

        let best = deltas
            .iter()
            .enumerate()
            .min_by_key(|(_, delta)| *delta);

        let Some((index, &delta)) = best else {
            return StepResult {
                error: self.error,
                improvement: None,
            };
        };

        if delta >= 0 {
            return StepResult {
                error: self.error,
                improvement: None,
            };
        }

        let shape = candidates.swap_remove(index);

        shape.draw(&mut self.canvas);

        let improvement = (-delta) as u64;

        self.error -= improvement;

        self.shapes_used.push(shape);

        StepResult {
            error: self.error,
            improvement: Some(improvement),
        }
    }

    pub fn canvas(&self) -> &RgbaImage {
        &self.canvas
    }
}
