use clap::Parser;

mod cli;
mod converter;

fn main() {
    let args = cli::Cli::parse();

    let conv: converter::Converter;
    match converter::Converter::new(args.input_color, args.hex, args.decimal) {
        Ok(c) => conv = c,
        Err(error) => {
            println!("\n{}\n", error);
            return;
        }
    }

    conv.print_info()
}
