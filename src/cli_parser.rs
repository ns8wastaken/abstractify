use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Path to the image file
    pub path: String,

    /// Suppress informational output (only show warnings and errors)
    #[arg(short, long)]
    pub quiet: bool,

    /// Number of candidate shapes generated initially at each step
    #[arg(short, long, default_value_t = 100)]
    pub candidats: usize,

    /// Number of steps to be run (number of shapes added)
    #[arg(short = 'n', long, default_value_t = 100)]
    pub steps: u64,

    /// Seed for the rng
    #[arg(short, long, default_value_t = 0)]
    pub seed: u64,

    /// Largest side length of the rescaled image while approximating shapes (aspect ratio is preserved)
    #[arg(short, long, default_value_t = 500)]
    pub rescale_size: u32,
}
