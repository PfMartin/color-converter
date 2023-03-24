use clap::Parser;

mod cli;
mod converter;

fn main() {
    let args = cli::Cli::parse();

    let converter = converter::Converter::new(args.name);

    converter.greet()
}
