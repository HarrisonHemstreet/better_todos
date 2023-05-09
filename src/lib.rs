use termcolor::{Color, ColorChoice, ColorSpec, StandardStream, WriteColor};
use std::io::Write;

// fn main() {
//     color_print("this is a todo bruh", "green");
//     better_todo("this is a todo bruh");
// }

fn better_todo(todo: &str) {
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Set the color specification for the text
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(Color::Red)); // Set the foreground color to red

    // Write colored text to the console
    stdout.set_color(&color_spec).unwrap();
    writeln!(&mut stdout, "{}", todo).unwrap();
}

fn color_print(msg: &str, mut color: &str ) {
    let color_conversion: Color = match color.to_lowercase().as_str() {
        "black" => Color::Black,
        "blue" => Color::Blue,
        "green" => Color::Green,
        "red" => Color::Red,
        "cyan" => Color::Cyan,
        "magenta" => Color::Magenta,
        "yellow" => Color::Yellow,
        "white" => Color::White,
        _ => unreachable!(),
    };
    // Create a `StandardStream` for writing to the console
    let mut stdout = StandardStream::stdout(ColorChoice::Always);

    // Set the color specification for the text
    let mut color_spec = ColorSpec::new();
    color_spec.set_fg(Some(color_conversion)); // Set the foreground color to red

    // Write colored text to the console
    stdout.set_color(&color_spec).unwrap();
    writeln!(&mut stdout, "{}", msg).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_better_todo() {
        let result = better_todo("hi testing");
        assert_eq!(result, ());
    }

    #[test]
    fn test_color_print() {
        let result = color_print("hi testing", "blue");
        assert_eq!(result, ());
    }

    #[test]
    fn test_color_print_capitalization() {
        let result = color_print("hi testing", "BLUE");
        assert_eq!(result, ());
    }
}
