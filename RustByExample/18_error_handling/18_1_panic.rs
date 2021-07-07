fn drink(beverage: &str) {
	if beverage == "lemonade" {
		panic!("Too much sugar");
	}

	println!("Drinking {}", beverage);
}

fn main() {
	drink("water");
	drink("lemonade");
}