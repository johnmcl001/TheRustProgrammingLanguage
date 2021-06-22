fn main() {
	// a reference to a string allocated in read only memory
	let pangram: &'static str = "the quick brown fox jumps over the lazy dog";
	println!("pangram: {}", pangram);

	// iterate over words in reverse, no new string allocated
	for word in pangram.split_whitespace().rev() {
		println!("> {}", word);
	}

	// copy chars into a vector, sort and remove duplicates
	let mut chars: Vec<char> = pangram.chars().collect();
	chars.sort();
	chars.dedup();

	// Create an empty and growable String
	let mut string = String::new();
	for c in chars {
		// insert a char at the end of string
		string.push(c);
		// insert a string at the end of string
		string.push_str(", ");
	}

	// the trimmed string is a slice to the original string, no new allocation performed
	let chars_to_trim: &[char] = &[' ', ','];
	let trimmed_str: &str = string.trim_matches(chars_to_trim);
	println!("Used characters: {}", trimmed_str);

	// heap allocate a string
	let alice = String::from("I like dogs");
	// allocate new memory and store the modified string there
	let bob: String = alice.replace("dog", "cat");

	println!("Alice says: {}", alice);
	println!("Bob says: {}", bob);
}