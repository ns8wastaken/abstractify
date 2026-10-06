mod cli_parser;
mod blend;
mod error;
mod primitive;
mod canvas;
mod generator;
mod abstractifier;

use clap::Parser;
use cli_parser::Cli;
use image::{ImageReader, imageops::FilterType};
use log::{LevelFilter, error, info, warn};

use crate::abstractifier::Abstractifier;
use crate::canvas::Canvas;
use crate::error::SquaredError;
use crate::generator::CandidateGenerator;
use crate::generator::color::AverageTargetColor;
use crate::generator::geometry::RandomGeometry;
use crate::primitive::ShapeKind;

fn main() {
    let cli = Cli::parse();

    // Set default log level based on the --quiet / -q flag
    let level = if cli.quiet {
        LevelFilter::Warn // Only warnings and errors
    } else {
        LevelFilter::Info // Show info, warnings, and errors
    };

    env_logger::Builder::new()
        .filter_level(level)
        .format_timestamp(None)
        .init();

    let Ok(reader) = ImageReader::open(&cli.path) else {
        warn!("Image file not found at path: {:?}", cli.path);
        return;
    };

    let Ok(img) = reader.decode() else {
        error!("Failed to decode image at path: {:?}", cli.path);
        return;
    };

    let target = img
        .resize(cli.rescale_size, cli.rescale_size, FilterType::Nearest)
        .into_rgba8();

    info!(
        "Successfully loaded image ({}x{})",
        target.width(),
        target.height()
    );

    let canvas = Canvas::average(&target);

    let generator = CandidateGenerator::new(
        RandomGeometry::new(
            vec![ShapeKind::Circle],
            0.25,
        ),
        // RandomColor::new(
        //     32,
        //     255,
        // ),
        AverageTargetColor::new(255 / 2),
    );

    let error_metric = SquaredError;

    let mut abstractifier = Abstractifier::new(
        target,
        canvas,
        generator,
        error_metric,
        cli.candidats,
        cli.seed,
    );

    let output_dir = "output";

    if let Err(err) = std::fs::create_dir_all(output_dir) {
        error!("Failed to create output directory: {err}");
        return;
    }

    for i in 1..=cli.steps {
        let result = abstractifier.step();

        match result.improvement {
            Some(improvement) => {
                info!(
                    "Iteration {i}: error = {}, improvement = {}",
                    result.error,
                    improvement
                );
            }
            None => {
                info!(
                    "Iteration {i}: no improvement, error = {}",
                    result.error
                );
            }
        }
    }

    let path = format!("{output_dir}/img.png");

    if let Err(err) = abstractifier.canvas().save(&path) {
        error!("Failed to save {path}: {err}");
        return;
    }
}
