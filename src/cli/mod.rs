use clap::Parser;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Cli {
    #[arg(short = 'R', long)]
    pub rgb: bool,

    #[arg(short = 'H', long)]
    pub hex: bool,

    #[arg(short, long)]
    pub input_color: String,
}
