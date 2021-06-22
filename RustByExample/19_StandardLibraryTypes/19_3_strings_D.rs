use std::str;

fn main() {
	// not actually a &str
	let bytestring: &[u8; 21] = b"this is a byte string";

	// Bytes arrays don't have Display
	println!("A byte string: {:?}", bytestring);

	// byte strings can have byte escapes
	let escaped = b"\x52\x75\x73\x74 as bytes";
	// but no unicode escapes
	// let escaped = b"\u{211D} is not allowed";
	println!("Some escaped bytes: {:?}", escaped);

	// raw byte strings work just like raw strings
	let raw_bytestring = br"\u{211D} is not escaped here";
	println!("{:?}", raw_bytestring);

	// converting a byte array to str can fail
	if let Ok(my_str) = str::from_utf8(raw_bytestring) {
		println!("And the same as text: {}", my_str);
	}

	let _quotes = br#"You can also use "fancier" formatting, \
					like with normal raw strings"#;

	// bytes strings don't have to be utf8
	let shift_jis = b"\x82\xe6\x82\xa8\x82\xb1\x82\xbb";

	// but they can't always be converted to str
	match str::from_utf8(shift_jis) {
		Ok(my_str) => println!("Conversion successful: {}", my_str),
		Err(e) => println!("Conversion failed: {:?}", e),
	};
}