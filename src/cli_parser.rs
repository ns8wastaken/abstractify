use clap::Parser;

#[derive(Parser)]
pub struct Cli {
    /// Path to the image file
    pub path: String,

    /// Suppress informational output (only show warnings and errors)
    #[arg(short, long)]
    pub quiet: bool,
}
