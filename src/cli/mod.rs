use crate::DEFAULT_INPUT;
use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'R', long, default_value_t = DEFAULT_INPUT)]
    pub rgb: String,

    #[arg(short = 'H', long, default_value_t = DEFAULT_INPUT)]
    pub hex: String,
}
