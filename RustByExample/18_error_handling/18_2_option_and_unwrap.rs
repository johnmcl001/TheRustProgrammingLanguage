fn give_adult(drink: Option<&str>) {
	match drink {
		Some("lemonade") => println!("Too sugary"),
		Some(inner) => println!("Refreshing {}", inner),
		None => println!("No drink"),
	}
}

fn drink(drink: Option<&str>) {
	let inside = drink.unwrap();
	if inside == "lemonade" {
		panic!("No lemonade");
	}
	println!("Thanks for the {}", inside);
}

fn main() {
	let water = Some("water");
	let lemonade = Some("lemonade");
	let void = None;

	give_adult(water);
	give_adult(lemonade);
	give_adult(void);

	let coffee = Some("coffee");
	let nothing = None;

	drink(coffee);
	drink(nothing); // panics
}