use crate::List::*;

enum List {
	// Cons: tuple struct that wraps an element and a pointer to the next node
	Cons(u32, Box<List>),
	// Nil: A node that signifies the end of the linked list
	Nil,
}

// Methods can be attached to an enum
impl List {
	// create empty list
	fn new() -> List{
		// Nil has type List
		Nil
	}

	// consume a list, return the same list with a new element at its front
	fn prepend(self, elem: u32) -> List {
		// cons also has type List
		Cons(elem, Box::new(self))
	}

	// return the length of the list
	fn len(&self) -> u32 {
		// self has to be matched because the behavior of this method depends
		// on the variant of self
		// self has type &List and *self has type List, matching on a concrete
		// type T is preferred over a match on a reference &T
		match *self {
			// can't take ownership of the tail because self is borrowed,
			// instead take a reference to the tail
			Cons(_, ref tail) => 1 + tail.len(),
			// Base case: an empty list has zero length
			Nil => 0
		}
	}

	// return representation of the list as a heap allocated string
	fn stringify(&self) -> String {
		match *self {
			Cons(head, ref tail) => {
				// format! is similar to print! but returns a heap allocated
				// string instead of printing to the console
				format!("{}, {}", head, tail.stringify())
			},
			Nil => {
				format!("Nil")
			},
		}
	}
}

fn main() {
	// create empty linked list
	let mut list = List::new();

	// prepend elements
	list = list.prepend(1);
	list = list.prepend(2);
	list = list.prepend(3);

	// show the final state of the list
	println!("Linked list has length: {}", list.len());
	println!("{}", list.stringify());

}