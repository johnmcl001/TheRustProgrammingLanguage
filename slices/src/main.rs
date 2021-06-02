fn main() {
    let str = String::from("Hello world");
    let first_word = first_word(&str);
    println!("First word: {}", first_word);

    // Array slicing
    let a = [1, 2, 3, 4, 5];
    println!("Slice of a: {:?}", &a[2..4]);
}

fn first_word(str: &str) -> &str {
    let bytes = str.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &str[..i];
        }
    }

    &str[..]
}
