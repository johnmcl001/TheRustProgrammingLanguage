fn main() {
    let triple = (1, -2, 3);

    println!("Tell me about {:?}", triple);
    match triple {
        (0, y, z) => println!("First is 0, y is {:?}, z is {:?}", y, z),
        (1, ..) => println!("First is 1, rest don't matter"),
        _ => println!("Doesn't matter what they are"),
    }
}