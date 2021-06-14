mod my {
	// public struct with public field of type T
	pub struct OpenBox<T> {
		pub contents: T,
	}

	// public struct with a private field of type T
	#[allow(dead_code)]
	pub struct ClosedBox<T> {
		contents: T,
	}

	impl<T> ClosedBox<T> {
		// public constructor method
		pub fn new(contents: T) -> ClosedBox<T> {
			ClosedBox {
				contents
			}
		}
	}
}

fn main() {
	// public structs with public fields can be construced as usual
	let open_box = my::OpenBox { contents: "public informaiton" };

	// and their fields can be normally accessed.
	println!("The open box contains: {}", open_box.contents);

	// public structs with private fields cannot be constructed using field names
	// Error: closed box has private fields
	// let closed_box = my::ClosedBox { contents: "classified information" };

	// structs with private fields can be created using public constructors
	let _closed_box = my::ClosedBox::new("classified information");

	// private fields of a public struct cannot be accessed.
	// Error: contents field is private
	// println!("The closed box contains: {}", _closed_box.contents);
}