fn main() {
    let not_a_palindrome = "Hello, world!";
    let palindrome = "madam, i'm adam";
    println!("{} {} palindrome", not_a_palindrome, if is_palindrome_rec(not_a_palindrome) {"is a"} else {"is not a"});
    println!("{} {} palindrome", palindrome, if is_palindrome_rec(palindrome) {"is a"} else {"is not a"});
}

fn is_palindrome_rec(str: &str) -> bool {
    let str_cleaned: String = str.chars()
        .map(|x| match x {
            'A'..='Z' => x,
            'a'..='z' => x,
            _ => '\0'
        }).collect();
    println!("{}", str_cleaned);
    is_palindrome_rec_helper(&str_cleaned)
}

fn is_palindrome_rec_helper(str: &str) -> bool {
    println!("{}", str);
    if str.len() <= 1 {
        println!("HERE");
        return true;
    }

    if str.chars().next().unwrap() != str.chars().last().unwrap() {
        return false;
    }

    is_palindrome_rec_helper(&str[1..str.len() - 1])
}
