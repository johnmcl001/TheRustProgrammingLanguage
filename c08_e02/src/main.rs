fn main() {
    println!("{} = {}", "first", convert_to_pig_latin(String::from("first")));
    println!("{} = {}", "apple", convert_to_pig_latin(String::from("apple")));
}

fn convert_to_pig_latin(s: String) -> String {
    let mut char_vec: Vec<char> = s.chars().collect();
    char_vec.append(&mut construct_suffix(&char_vec[0]));
    if !is_vowel(&char_vec[0]) {
        char_vec.remove(0);
    }
    char_vec.iter().collect()
}

fn is_vowel(c: &char) -> bool {
    vec!['a', 'e', 'i', 'o', 'u'].contains(&c)
}

fn construct_suffix(c: &char) -> Vec<char> {
    vec!['-', if is_vowel(c) { 'h' } else { *c }, 'a', 'y']
}
