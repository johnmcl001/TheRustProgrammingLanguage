fn main() {
	let raw_str = r"Escapes don't work here: \x3F \u{211D}";
	println!("{}", raw_str);

	// if you need quotes in a raw string, add a pair of #s
	let quotes = r#"And then I said: "There is no escape!""#;
	println!("{}", quotes);

	// if "# is needed in the string just use more #s in the delimiter
	// there is no limit for the number #s that can be used
	let longer_delimiter = r###"A string with "# in it and even "##!"###;
	println!("{}", longer_delimiter);
}