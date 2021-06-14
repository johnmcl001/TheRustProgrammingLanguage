fn function() {
	println!("called function");
}

mod cool {
	pub fn function() {
		println!("called cool::function");
	}
}

mod my {
	fn function() {
		println!("called my::function");
	}

	mod cool {
		pub fn function() {
			println!("called my::cool::function");
		}
	}

	pub fn indirect_call() {
		// let's access all the functions named function from this scope
		println!("called my::indirect_call");

		// self refers to the current module scope, in this case "my"
		// Calling self::function cand calling function directly both give
		// the same result becaue they refer to the same function.
		self::function();
		function();

		// can also use self to access another module inside my
		self::cool::function();

		// the super keyword referes to the parent scope outside the my module
		super::function();

		// this will bind to the cool::function in the "crate" scope
		// in this case the crate scope is the outermost scope
		{
			use crate::cool::function as root_function;
			root_function();
		}
	}
}

fn main() {
	my::indirect_call();
}