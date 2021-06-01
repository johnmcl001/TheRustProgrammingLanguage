fn main() {
    let str = String::from("Hello world");
    let first_word = first_word(&str);
    println!("First word: {}", first_word);
}

fn first_word(str: &String) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}
