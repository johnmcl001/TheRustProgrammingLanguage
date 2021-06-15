extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("argument_parsing")
        .version("1.0")
        .author("John")
        .about("parses command line arguments")
        .arg(Arg::with_name("file")
            .short("f")
            .long("file")
            .takes_value(true)
            .help("A file"))
        .arg(Arg::with_name("num")
            .short("n")
            .long("number")
            .takes_value(true)
            .help("A number"))
        .get_matches();

    let file = matches.value_of("file").unwrap_or("input.txt");
    println!("The file passed is: {}", file);

    let number_string = matches.value_of("num");
    match number_string {
        None => println!("No number passed"),
        Some(s) => {
            match s.parse::<i32>() {
                Ok(n) => println!("Number: {}", n),
                Err(_) => println!("Not a number: {}", s),
            }
        }
    }
}
