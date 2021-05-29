fn main() {
    let n = 5;

    if n < 0 {
        println!("{} is negative", n);
    } else if n > 0 {
        println!("{} is positive", n);
    } else {
        println!("{} is zero", n);
    }

    let big_n =
        if n < 10 && n > -10 {
            println!(", and is a small number, increase ten-fold");
            10 * n // Returns an i32
        } else {
            println!(", and is a big number, halve the number");
            n / 2 // Returns an i32 as weel
        }; // Need semi colon here

    println!("{} -> {}", n, big_n);
}
