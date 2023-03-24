#[derive(PartialEq)]
enum ColorFormat {
    Hex,
    Decimal,
}

pub struct Converter {
    input_color_code: String,
    output_format: ColorFormat,
}

impl Converter {
    pub fn new(input_color_code: String, hex: bool, decimal: bool) -> Result<Converter, String> {
        let color_code: String;
        match Converter::validate_color_code(input_color_code) {
            Ok(c) => color_code = c,
            Err(err) => return Err(err),
        }

        let output_format: ColorFormat;
        match Converter::validate_format(hex, decimal) {
            Ok(o) => output_format = o,
            Err(err) => return Err(err),
        }

        Ok(Converter {
            input_color_code: color_code,
            output_format: output_format,
        })
    }

    fn validate_color_code(color_code: String) -> Result<String, String> {
        if color_code.contains('.') {
            return Err(String::from("Not a valid color code"));
        }

        Ok(color_code)
    }

    fn validate_format(hex: bool, decimal: bool) -> Result<ColorFormat, String> {
        let output_format: ColorFormat;
        if hex && decimal {
            return Err(String::from("Only one of the two color output formats can be set.\nChoose either --hex / -H or --decimal / -D."));
        }

        if hex {
            output_format = ColorFormat::Hex;
        } else if decimal {
            output_format = ColorFormat::Decimal;
        } else {
            return Err(String::from("At least one color output format must be specified.\nChoose either --hex / -H or --decimal / -D."));
        }

        Ok(output_format)
    }

    pub fn print_info(&self) {
        let mut format = String::new();

        if self.output_format == ColorFormat::Hex {
            format = String::from("hexadecimal");
        } else if self.output_format == ColorFormat::Decimal {
            format = String::from("decimal")
        }

        println!(
            "\nThe color {}, will be converted to {} format.\n",
            self.input_color_code, format
        )
    }
}
