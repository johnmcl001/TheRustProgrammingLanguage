use ansi_term::Colour::{Red, Green, Blue};
use ansi_term::Style;
use ansi_term::{ANSIString, ANSIStrings};

fn main() {
    println!("{}", Red.paint("Hello, world!"));
    println!("{}", Style::new().bold().paint("Hello, world!"));
    println!("{}", Green.bold().paint("Hello, world!"));

    let strings: &[ANSIString<'static>] = &[
        Green.paint("["),
        Red.bold().paint("This Value"),
        Blue.paint("]"),
    ];

    println!("Value: {}", ANSIStrings(strings));

}
