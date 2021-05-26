fn main() {
    println!("fibonacci_iter: {}", fibonacci_iter(5));
    println!("fibonacci_iter: {}", fibonacci_iter(5));
    println!("fibonacci_rec: {}", fibonacci_rec(5));
    println!("fibonacci_rec_match_1: {}", fibonacci_rec_match_1(5));
    println!("fibonacci_rec_match_2: {}", fibonacci_rec_match_2(5));
}

fn fibonacci_rec_match_2(n: u64) -> u64 {
    match n {
        0 => 0,
        1 => 1,
        n => fibonacci_rec_match_2(n - 1) + fibonacci_rec_match_2(n - 2)
    }
}

fn fibonacci_rec_match_1(n: u64) -> u64 {
    match n {
        0 | 1 => n,
        n => fibonacci_rec_match_1(n - 1) + fibonacci_rec_match_1(n - 2)
    }
}

fn fibonacci_rec(n: u64) -> u64 {
    if n == 0 || n == 1 {n} else {fibonacci_rec(n - 1) + fibonacci_rec(n - 2)}
}

fn fibonacci_iter(n: u64) -> u64 {
    let mut previous_number = 0;
    let mut current_number = 1;
    let mut tmp;
    let mut counter = n;
    while counter > 1 {
        tmp = current_number;
        current_number = previous_number + current_number;
        previous_number = tmp;
        counter -= 1;
    }
    current_number
}
