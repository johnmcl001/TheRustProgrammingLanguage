// bind deeply::nested::function path to other_function
use deeply::nested::function as other_function;

fn function() {
	println!("Called function");
}

mod deeply {
	pub mod nested {
		pub fn function() {
			println!("called deeply::nested::function");
		}
	}
}

fn main() {
	// easier access to deeply::nested::function
	other_function();

	println!("Entering block");
	{
		// equivalent to use deeply::nested::function as function
		// this shadows the outside one
		use crate::deeply::nested::function;

		// use bindings have a local scope, in this case shadowing of function
		// is only in this block
		function();

		println!("Leaving block");
	}

	function();
}