use crate::DEFAULT_INPUT;

#[derive(Clone, Copy, PartialEq, Debug)]
enum ColorFormat {
    Hex,
    Rgb,
}

pub struct Converter {
    input_color_code: String,
    input_format: ColorFormat,
    output_format: ColorFormat,
}

impl Converter {
    pub fn new(rgb: String, hex: String) -> Result<Converter, String> {
        let input_format: ColorFormat;
        match Converter::validate_rgb_format(&rgb) {
            Ok(_) => input_format = ColorFormat::Rgb,
            Err(err) => return Err(err),
        }

        let output_format: ColorFormat;
        let input_color: String;
        if input_format == ColorFormat::Rgb && rgb != DEFAULT_INPUT {
            output_format = ColorFormat::Hex;
            input_color = rgb;
        } else if input_format == ColorFormat::Hex && hex != DEFAULT_INPUT {
            output_format = ColorFormat::Rgb;
            input_color = hex;
        } else {
            return Err(String::from("Could not determine the output format"));
        }

        Ok(Converter {
            input_color_code: input_color,
            input_format: input_format,
            output_format: output_format,
        })
    }

    fn validate_rgb_format(rgb: &String) -> Result<(), String> {
        if rgb.contains('.') {
            let err_string = format!("{} is not a valid RGB color code. RGB doesn't use '.'", rgb);

            return Err(err_string);
        }

        return Ok(());
    }

    pub fn print_info(&self) {
        let input_format = Converter::get_format(self.input_format);
        let output_format = Converter::get_format(self.output_format);

        println!(
            "\nThe color {} is of format {} and will be converted to {} format.\n",
            self.input_color_code, input_format, output_format
        )
    }

    fn get_format(color_format: ColorFormat) -> String {
        let mut format = String::new();
        if color_format == ColorFormat::Hex {
            format = String::from("hexadecimal");
        } else if color_format == ColorFormat::Rgb {
            format = String::from("RGB")
        }

        format
    }
}
