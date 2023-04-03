use clap::Parser;

mod cli;
mod converter;

const DEFAULT_INPUT: String = String::new();

fn main() {
    let args = cli::Cli::parse();

    let conv: converter::Converter;
    match converter::Converter::new(args.rgb, args.hex) {
        Ok(c) => conv = c,
        Err(error) => {
            println!("\n{}\n", error);
            return;
        }
    }

    conv.print_info()
}
