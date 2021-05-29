fn main() {
    let mut count = 0u32;

    println!("Let's count to infinity");

    // Infinite loop
    loop {
        count += 1;

        if count == 3 {
            println!("three");

            continue; // skip rest of this iteration
        }

        println!("{}", count);

        if count == 5 {
            println!("enough");

            break; // exit loop
        }
    }
}
