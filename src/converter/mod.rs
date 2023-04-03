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
        match Converter::validate_input_parameter(&rgb, &hex) {
            Ok(ok) => ok,
            Err(err) => return Err(err),
        }

        let input_format: ColorFormat;
        let output_format: ColorFormat;
        let input_color: String;

        if rgb != DEFAULT_INPUT {
            match Converter::validate_rgb_format(&rgb) {
                Ok(_) => input_format = ColorFormat::Rgb,
                Err(err) => return Err(err),
            };
            output_format = ColorFormat::Hex;
            input_color = rgb;
        } else if hex != DEFAULT_INPUT {
            input_format = ColorFormat::Rgb;
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

    fn validate_input_parameter(rgb: &String, hex: &String) -> Result<(), String> {
        if *rgb == DEFAULT_INPUT && *hex == DEFAULT_INPUT {
            return Err(String::from(
                "Please provide a color code in hexadecimal or RGB format.",
            ));
        } else if *rgb != DEFAULT_INPUT && *hex != DEFAULT_INPUT {
            return Err(String::from(
                "Please provide only one color code in hexadecimal or RGB format.",
            ));
        }

        return Ok(());
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_input_parameters() {
        let test_cases = [
            (String::from("ffffff"), String::from("255,255,255"), true),
            (DEFAULT_INPUT, String::from("255,255,255"), false),
            (String::from("ffffff"), DEFAULT_INPUT, false),
        ];

        for (hex, rgb, has_err) in test_cases.iter() {
            let mut e = String::new();

            match Converter::validate_input_parameter(&rgb, &hex) {
                Ok(ok) => ok,
                Err(err) => e = err,
            }
            assert_eq!(e != String::new(), *has_err)
        }
    }
}
