#[derive(PartialEq)]
enum ColorFormat {
    Hex,
    Rgb,
}

pub struct Converter {
    input_color_code: String,
    output_format: ColorFormat,
}

impl Converter {
    pub fn new(input_color_code: String, is_hex: bool, is_rgb: bool) -> Result<Converter, String> {
        let color_code: String;
        match Converter::validate_color_code(input_color_code) {
            Ok(c) => color_code = c,
            Err(err) => return Err(err),
        }

        let output_format: ColorFormat;
        match Converter::validate_format(is_hex, is_rgb) {
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

    fn validate_format(is_hex: bool, is_rgb: bool) -> Result<ColorFormat, String> {
        let output_format: ColorFormat;
        if is_hex && is_rgb {
            return Err(String::from("Only one of the two color output formats can be set.\nChoose either --is_hex / -H or --is_rgb / -D."));
        }

        if is_hex {
            output_format = ColorFormat::Hex;
        } else if is_rgb {
            output_format = ColorFormat::Rgb;
        } else {
            return Err(String::from("At least one color output format must be specified.\nChoose either --is_hex / -H or --is_rgb / -D."));
        }

        Ok(output_format)
    }

    pub fn print_info(&self) {
        let mut format = String::new();

        if self.output_format == ColorFormat::Hex {
            format = String::from("hexadecimal");
        } else if self.output_format == ColorFormat::Rgb {
            format = String::from("RGB")
        }

        println!(
            "\nThe color {}, will be converted to {} format.\n",
            self.input_color_code, format
        )
    }
}
