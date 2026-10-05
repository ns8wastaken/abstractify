mod cli_parser;

use clap::Parser;
use cli_parser::Cli;
use image::ImageReader;
use log::{LevelFilter, error, info, warn};

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

    let img = img.into_rgb8();
    info!("Successfully loaded image ({}x{})", img.width(), img.height());
}
