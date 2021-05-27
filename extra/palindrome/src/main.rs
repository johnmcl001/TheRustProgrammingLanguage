fn main() {
    let not_a_palindrome = "Hello, world!";
    let palindrome = "Madam, I'm Adam";
    println!("Recursive check");
    println!("{} {} palindrome", not_a_palindrome, if is_palindrome_rec(not_a_palindrome) {"is a"} else {"is not a"});
    println!("{} {} palindrome", palindrome, if is_palindrome_rec(palindrome) {"is a"} else {"is not a"});
    println!("\nIterative check");
    println!("{} {} palindrome", not_a_palindrome, if is_palindrome_iter(not_a_palindrome) {"is a"} else {"is not a"});
    println!("{} {} palindrome", palindrome, if is_palindrome_iter(palindrome) {"is a"} else {"is not a"});
}

fn is_palindrome_iter(str: &str) -> bool {
    let str_cleaned = is_palindrome_helper(str);
    str_cleaned.chars()
        .zip(str_cleaned.chars().rev())
        .all(|x| x.0 == x.1)
}

fn is_palindrome_rec(str: &str) -> bool {
    check_palindrome_rec(is_palindrome_helper(str))
}

fn check_palindrome_rec(str: String) -> bool {
    if str.chars().next().unwrap() != str.chars().last().unwrap() {
        return false;
    }

    str.len() <= 1 || is_palindrome_rec(&str[1..str.len() - 1])
}

fn is_palindrome_helper(str: &str) -> String {
    str.chars()
        .filter(|x| x.is_ascii_alphanumeric())
        .map(|x| x.to_lowercase().to_string())
        .collect()
}
