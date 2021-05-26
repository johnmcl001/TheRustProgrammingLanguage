fn main() {
    println!("Iterative factorial range: {}", factorial_iterative_range(3));
    println!("Iterative factorial for loop: {}", factorial_iterative_for(3));
    println!("Iterative factorial while loop: {}", factorial_iterative_while(3));
    println!("Recursive factorial if: {}", factorial_rec_if(3));
    println!("Recursive factorial match: {}", factorial_rec_match(3));
}

fn factorial_iterative_range(n: u64) -> u64 {
    (2..=n).product()
}

fn factorial_iterative_for(n: u64) -> u64 {
    let mut acc = 1;
    for i in 2..n + 1 {
        acc *= i;
    }
    acc
}

fn factorial_iterative_while(n: u64) -> u64 {
    let mut acc = 1;
    let mut counter = 2;
    while counter <= n {
        acc *= counter;
        counter += 1;
    }
    acc
}

fn factorial_rec_if(n: u64) -> u64 {
    if n <= 2 {n} else {n * factorial_rec_if(n - 1)}
}

fn factorial_rec_match(n: u64) -> u64 {
    match n {
        1 => 1,
        n => n * factorial_rec_match(n - 1)
    }
}
