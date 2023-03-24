mod converter;

fn main() {
    let converter = converter::Converter::new(String::from("Martin"));

    converter.greet()
}
